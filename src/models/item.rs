use serde::Serialize;

///
/// 購入品
#[derive(Debug, Serialize)]
pub struct Item {
    ///
    /// カタログID
    pub catalog_id: String,

    ///
    /// 部品名
    pub name: String,

    ///
    /// 画像URL(相対パス)
    /// # Example
    /// /img/goods/M/128818.jpg
    pub img_url: String,

    ///
    /// 個数
    pub quantity: u32,

    ///
    /// 金額(1個あたり)
    pub unit_price: u32,
}

impl Item {
    ///
    /// 合計金額
    pub fn total_price(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
