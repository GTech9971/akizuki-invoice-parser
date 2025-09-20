use akizuki_invoice_parser::models::{invoice::Invoice, item::Item};
use chrono::Local;

#[test]
fn test_total_price() {
    let item1 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    let item2 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    let item3 = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
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
