use std::fs;

use akizuki_invoice_parser::parser::html_parser;

#[test]
fn test_order_id() {
    let html_content = fs::read_to_string("./assets/sample.html").unwrap();

    let actual = html_parser::parse_invoice_order_id(&html_content);

    assert_eq!(actual.unwrap(), "EC250831-391903137-01");
}

#[test]
fn test_order_date() {
    let html_content = fs::read_to_string("./assets/sample.html").unwrap();

    let actual = html_parser::parse_order_date(&html_content);

    assert_eq!(actual.unwrap(), "2025年08月31日");
}

#[test]
fn test_shipping_date() {
    let html_content = fs::read_to_string("./assets/sample.html").unwrap();

    let actual = html_parser::parse_shipping_date(&html_content);

    assert_eq!(actual.unwrap(), "2025年09月01日");
}

#[test]
fn test_items() {
    let html_content = fs::read_to_string("./assets/sample.html").unwrap();

    let actual = html_parser::parse_items(&html_content);

    assert_eq!(actual.len(), 0);
}
