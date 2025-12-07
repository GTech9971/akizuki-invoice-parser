use std::fs;

use akizuki_invoice_parser::{
    models::{invoice::Invoice, item::Item},
    parser::html_parser,
};

#[test]
fn test_order_id() {
    let html_content: String = fs::read_to_string("./assets/before-shipment.html").unwrap();

    let actual: Option<String> = html_parser::parse_invoice_order_id(&html_content);

    assert_eq!(actual.unwrap(), "EC251206-076906061-01");
}

#[test]
fn test_order_date() {
    let html_content: String = fs::read_to_string("./assets/before-shipment.html").unwrap();

    let actual: Option<String> = html_parser::parse_order_date(&html_content);

    assert_eq!(actual.unwrap(), "2025年12月06日");
}

#[test]
fn test_shipping_date() {
    let html_content: String = fs::read_to_string("./assets/before-shipment.html").unwrap();

    let actual: Option<String> = html_parser::parse_shipping_date(&html_content);

    assert_eq!(actual, None);
}

#[test]
fn test_items() {
    let html_content: String = fs::read_to_string("./assets/before-shipment.html").unwrap();

    let actual: Vec<Item> = html_parser::parse_items(&html_content);

    assert_eq!(actual.len(), 8);

    let expected = vec![
        (
            "114526",
            "チップ積層セラミックコンデンサー 1μF25V X7R 1608",
            "/img/goods/M/126900.jpg",
            1,
            100,
        ),
        (
            "102615",
            "SOP28ピン 1.27mm DIP変換基板 金フラッシュ",
            "/img/goods/M/102615.jpg",
            3,
            50,
        ),
        (
            "110497",
            "SSOP20ピン(0.65mm)・SOP20ピン(1.27mm)DIP変換基板",
            "/img/goods/M/110497.jpg",
            3,
            50,
        ),
        (
            "105154",
            "SOP8(1.27mm)DIP変換基板 金フラッシュ",
            "/img/goods/M/126264.jpg",
            3,
            110,
        ),
        (
            "110648",
            "PICマイコン PIC16F1938T-I/SO",
            "/img/goods/M/110648.jpg",
            3,
            320,
        ),
        (
            "104437",
            "PICマイコン PIC18F14K50T-I/SS(USB内蔵)",
            "/img/goods/M/104437.jpg",
            3,
            350,
        ),
        (
            "130017",
            "PICマイコン PIC16F15313T-I/SN",
            "/img/goods/M/130017.jpg",
            5,
            110,
        ),
        (
            "113161",
            "チップ積層セラミックコンデンサー 10μF35V X5R 1608",
            "/img/goods/M/126899.jpg",
            2,
            150,
        ),
    ];

    for (i, (catalog_id, name, img_url, quantity, unit_price)) in expected.iter().enumerate() {
        let item: &Item = &actual[i];
        assert_eq!(item.catalog_id, catalog_id.to_string());
        assert_eq!(item.name, name.to_string());
        assert_eq!(item.img_url, img_url.to_string());
        assert_eq!(item.quantity, *quantity);
        assert_eq!(item.unit_price, *unit_price);
    }
}

#[test]
fn test_invoice() {
    let html_content: String = fs::read_to_string("./assets/before-shipment.html").unwrap();

    let actual: Invoice = html_parser::parse_invoice(&html_content).ok().unwrap();

    assert_eq!(actual.order_id, "EC251206-076906061-01",);
    assert_eq!(actual.order_date.to_string(), "2025-12-06",);
    assert_eq!(actual.shipping_date, None);
    assert_eq!(actual.items.len(), 8);
}
