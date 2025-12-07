use chrono::NaiveDate;
use serde::Serialize;

use crate::models::item::Item;

///
/// 納品書
#[derive(Debug, Serialize)]
pub struct Invoice {
    /// オーダーID
    ///
    pub order_id: String,

    ///
    /// 注文日
    pub order_date: NaiveDate,

    ///
    /// 出荷日
    pub shipping_date: Option<NaiveDate>,

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

    ///
    /// JSONに変換
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    ///
    /// 整形されたJSONに変換
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
