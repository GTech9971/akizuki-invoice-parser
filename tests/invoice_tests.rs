use std::fs;

use akizuki_invoice_parser::{
    models::{invoice::Invoice, item::Item},
    parser::html_parser,
};
use chrono::Local;

#[test]
fn test_total_price() {
    let item1 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        img_url: "".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    let item2 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        img_url: "".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    let item3 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        img_url: "".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    let sut: Invoice = Invoice {
        order_id: "1234567890-rfgea".to_string(),
        order_date: Local::now().date_naive(),
        shipping_date: Local::now().date_naive(),
        items: vec![item1, item2, item3],
    };

    assert_eq!(sut.total_price(), 90);
}

#[test]
fn test_invoice_json() {
    let html_content: String = fs::read_to_string("./assets/sample.html").unwrap();
    let invoice: Invoice = html_parser::parse_invoice(&html_content);

    let json_result = invoice.to_json();
    assert!(json_result.is_ok());

    let json_pretty_result = invoice.to_json_pretty();
    assert!(json_pretty_result.is_ok());

    // JSONが正しくパースできることを確認
    let json_str = json_result.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["order_id"], "EC250831-391903137-01");
    assert_eq!(parsed["order_date"], "2025-08-31");
    assert_eq!(parsed["shipping_date"], "2025-09-01");
    assert_eq!(parsed["items"].as_array().unwrap().len(), 8);
}
