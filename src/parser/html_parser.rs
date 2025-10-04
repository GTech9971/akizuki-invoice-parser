use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::models::{invoice::Invoice, item::Item};

///
/// html納品書からオーダーIDを取り出す
///
/// # Example
///
/// ```
/// use akizuki_invoice_parser::parser::html_parser;
/// use std::fs;
///
/// let html_content = fs::read_to_string("./assets/sample.html").unwrap();
/// let order_id = html_parser::parse_invoice_order_id(&html_content);
/// assert_eq!(order_id.unwrap(), "EC250831-391903137-01");
/// ```
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
/// html納品書から注文日を取り出す
///
/// # Example
///
/// ```
/// use akizuki_invoice_parser::parser::html_parser;
/// use std::fs;
///
/// let html_content = fs::read_to_string("./assets/sample.html").unwrap();
/// let order_date = html_parser::parse_order_date(&html_content);
/// assert_eq!(order_date.unwrap(), "2025年08月31日");
/// ```
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
///
/// # Example
///
/// ```
/// use akizuki_invoice_parser::parser::html_parser;
/// use std::fs;
///
/// let html_content = fs::read_to_string("./assets/sample.html").unwrap();
/// let shipping_date = html_parser::parse_shipping_date(&html_content);
/// assert_eq!(shipping_date.unwrap(), "2025年09月01日");
/// ```
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
///
/// # Example
///
/// ```
/// use akizuki_invoice_parser::parser::html_parser;
/// use std::fs;
///
/// let html_content = fs::read_to_string("./assets/sample.html").unwrap();
/// let items = html_parser::parse_items(&html_content);
/// assert_eq!(items.len(), 8);
/// assert_eq!(items[0].catalog_id, "108617");
/// ```
pub fn parse_items(html_content: &str) -> Vec<Item> {
    let document: Html = Html::parse_document(html_content);

    let root_selector =
        Selector::parse("div.block-purchase-history-detail--order-body tbody").unwrap();

    let catalog_id_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-code").unwrap();

    let name_selector: Selector =
        Selector::parse("div.block-purchase-history-detail--goods-name").unwrap();

    let img_url_selector: Selector = Selector::parse("img").unwrap();

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

            let img_url: String = tr
                .select(&img_url_selector)
                .next()
                .map(|x| {
                    x.attr("data-src")
                        .expect("画像URLの取得に失敗")
                        .trim()
                        .to_string()
                })
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
                img_url: img_url,
                quantity: quantity,
                unit_price: unit_of_price,
            };

            dbg!(&item);

            result_list.push(item);
        }
    }

    return result_list;
}

///
/// htmlから納品書を解析する
///
/// # Example
///
/// ```
/// use akizuki_invoice_parser::parser::html_parser;
/// use std::fs;
///
/// let html_content = fs::read_to_string("./assets/sample.html").unwrap();
/// let invoice = html_parser::parse_invoice(&html_content).ok().unwrap();
/// assert_eq!(invoice.order_id, "EC250831-391903137-01");
/// assert_eq!(invoice.items.len(), 8);
///
/// // JSON変換も可能
/// let json = invoice.to_json().unwrap();
/// println!("{}", json);
/// ```
pub fn parse_invoice(html_content: &str) -> Result<Invoice, String> {
    let order_date: String = parse_order_date(html_content).ok_or("注文日の取得に失敗")?;
    let shipping_date: String = parse_shipping_date(html_content).ok_or("出荷日の取得に失敗")?;

    let order_id: String = parse_invoice_order_id(html_content).ok_or("オーダーIDの取得に失敗")?;

    let items: Vec<Item> = parse_items(html_content);

    return Ok(Invoice {
        order_id: order_id,
        order_date: NaiveDate::parse_from_str(&order_date, "%Y年%m月%d日").expect(""),
        shipping_date: NaiveDate::parse_from_str(&shipping_date, "%Y年%m月%d日").expect(""),
        items: items,
    });
}
