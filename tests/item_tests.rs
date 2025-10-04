use akizuki_invoice_parser::models::item::Item;

#[test]
fn test_total_price() {
    let item = Item {
        catalog_id: "123456".to_string(),
        name: "componentA".to_string(),
        img_url: "http://xxx".to_string(),
        quantity: 10,
        unit_price: 3,
    };

    assert_eq!(item.total_price(), 30);
}
