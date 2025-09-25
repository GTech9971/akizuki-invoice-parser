use scraper::{Html, Selector};

use crate::models::{invoice::Invoice, item::Item};

///
/// html納品書からオーダーIDを取り出す
pub fn parse_invoice_order_id(html_content: &str) -> Option<String> {
    let document: Html = Html::parse_document(html_content);

    let selector: Selector = Selector::parse("td.block-purchase-history-detail--order-id").unwrap();

    let order_id_string: Option<String> = document
        .select(&selector)
        .next()
        .map(|x| x.text().collect::<Vec<_>>().join("").trim().to_string());

    return order_id_string;
}

///
/// html納品書から出荷日を取り出す
pub fn parse_order_date(html_content: &str) -> Option<String> {
    let document: Html = Html::parse_document(html_content);

    let selector = Selector::parse("td.block-purchase-history-detail--order-dt").unwrap();

    let order_date_string: Option<String> = document
        .select(&selector)
        .next()
        .map(|x| x.text().collect::<Vec<_>>().join("").trim().to_string());

    return order_date_string;
}

///
/// 納品書から出荷日を取得する
pub fn parse_shipping_date(html_content: &str) -> Option<String> {
    let document: Html = Html::parse_document(html_content);

    let selector = Selector::parse("td.block-purchase-history-detail--ship-dt").unwrap();

    let shipping_date_string: Option<String> = document
        .select(&selector)
        .next()
        .map(|x| x.text().collect::<Vec<_>>().join("").trim().to_string());

    return shipping_date_string;
}

///
/// 購入品リストを返す
pub fn parse_items(html_content: &str) -> Vec<Item> {
    let document: Html = Html::parse_document(html_content);

    let root_selector =
        Selector::parse("div.block-purchase-history-detail--order-body tbody").unwrap();

    let catalog_id_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-code").unwrap();

    let name_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-name").unwrap();

    let quantity_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-qty").unwrap();

    let total_price_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-total-price").unwrap();

    let mut result_list: Vec<Item> = Vec::new();

    if let Some(tbody) = document.select(&root_selector).next() {
        let tr_selector = Selector::parse("tr").unwrap();
        for tr in tbody.select(&tr_selector) {
            let catalog_id: String = tr
                .select(&catalog_id_selector)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let name: String = tr
                .select(&name_selector)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let quantity_ret: Result<u32, String> = tr
                .select(&quantity_selector)
                .next()
                .ok_or("数量の取得に失敗".to_string())
                .and_then(|e| {
                    let text: String = e.text().collect::<String>().trim().to_string();
                    return text
                        .parse::<u32>()
                        .map_err(|_| format!("数量の変換に失敗:{text}"));
                });

            let total_price_ret: Result<u32, String> = tr
                .select(&total_price_selector)
                .next()
                .ok_or("合計金額が見つかりませんでした。".to_string())
                .and_then(|e| {
                    let text: String = e
                        .text()
                        .collect::<String>()
                        .trim()
                        .replace("￥", "")
                        .replace(",", "")
                        .to_string();
                    return text
                        .parse::<u32>()
                        .map_err(|_| format!("合計金額の変換に失敗:{}", text));
                });

            let quantity: u32 = quantity_ret.unwrap();
            let total_price: u32 = total_price_ret.unwrap();
            let unit_of_price: u32 = total_price / quantity;

            let item: Item = Item {
                catalog_id: catalog_id,
                name: name,
                quantity: quantity,
                unit_price: unit_of_price,
            };

            dbg!(&item);

            result_list.push(item);
        }
    }

    return result_list;
}
