use std::fs;

use akizuki_invoice_parser::{
    models::{invoice::Invoice, item::Item},
    parser::html_parser,
};

#[test]
fn test_order_id() {
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();

    let actual: Option<String> = html_parser::parse_invoice_order_id(&html_content);

    assert_eq!(actual.unwrap(), "EC250831-391903137-01");
}

#[test]
fn test_order_date() {
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();

    let actual: Option<String> = html_parser::parse_order_date(&html_content);

    assert_eq!(actual.unwrap(), "2025年08月31日");
}

#[test]
fn test_shipping_date() {
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();

    let actual: Option<String> = html_parser::parse_shipping_date(&html_content);

    assert_eq!(actual.unwrap(), "2025年09月01日");
}

#[test]
fn test_items() {
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();

    let actual: Vec<Item> = html_parser::parse_items(&html_content);

    assert_eq!(actual.len(), 8);

    let expected = vec![
        (
            "108617",
            "丸ピンICソケット ( 6P)",
            "/img/goods/M/108617.jpg",
            10,
            20,
        ),
        (
            "113582",
            "積層セラミックコンデンサー 0.1μF50V X7R 2.54mm",
            "/img/goods/M/128008.jpg",
            2,
            100,
        ),
        (
            "101318",
            "5mm赤色LED 625nm 7cd60度",
            "/img/goods/M/101318.jpg",
            1,
            150,
        ),
        (
            "106405",
            "5mm緑色LED 525nm OSG58A5111A",
            "/img/goods/M/127499.jpg",
            1,
            200,
        ),
        (
            "110887",
            "PICマイコン PIC16F1455-I/P",
            "/img/goods/M/110887.jpg",
            5,
            300,
        ),
        (
            "100006",
            "ICソケット (14P)",
            "/img/goods/M/128818.jpg",
            1,
            120,
        ),
        (
            "109862",
            "L型ピンソケット 1×6(6P)",
            "/img/goods/M/109862.jpg",
            10,
            25,
        ),
        (
            "100167",
            "ピンヘッダー 1×40 (40P)",
            "/img/goods/M/100167.jpg",
            2,
            35,
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
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();

    let actual: Invoice = html_parser::parse_invoice(&html_content).ok().unwrap();

    assert_eq!(actual.order_id, "EC250831-391903137-01",);
    assert_eq!(actual.order_date.to_string(), "2025-08-31",);
    assert_eq!(actual.shipping_date.to_string(), "2025-09-01");
    assert_eq!(actual.items.len(), 8);
}

#[test]
fn test_fail() {
    let actual: Result<Invoice, String> = html_parser::parse_invoice("invalid html");

    assert_eq!(actual.is_err(), true);
    assert_eq!(actual.err().unwrap(), "注文日の取得に失敗");
}
