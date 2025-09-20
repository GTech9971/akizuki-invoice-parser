///
/// 購入品
pub struct Item {
    ///
    /// カタログID
    pub catalog_id: String,

    ///
    /// 部品名
    pub name: String,

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
