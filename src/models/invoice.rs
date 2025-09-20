use chrono::NaiveDate;

use crate::models::item::Item;

///
/// 納品書
pub struct Invoice {
    /// オーダーID
    ///
    pub order_id: String,

    ///
    /// 注文日
    pub order_date: NaiveDate,

    ///
    /// 出荷日
    pub shipping_date: NaiveDate,

    ///
    /// 購入品リスト
    pub items: Vec<Item>,
}

impl Invoice {
    ///
    /// 合計金額
    #[rustfmt::skip]
    pub fn total_price(&self) -> u32 {
        return self.items.iter()
                        .map(|item| item.total_price())
                        .sum();
    }
}
