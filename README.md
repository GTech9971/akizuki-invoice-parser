# akizuki-invoice-parser

秋月電子の納品書HTMLを解析してInvoice構造体に変換するRustライブラリです。

## 機能

- HTMLファイルからオーダーID、注文日、出荷日を取得
- 購入品リストの解析（カタログID、商品名、数量、単価）
- Invoice構造体のJSON変換
- Tauriアプリケーションとの連携に最適

## 使用方法

### 基本的な使用例

```rust
use akizuki_invoice_parser::parser::html_parser;
use std::fs;

// HTMLファイルを読み込み
let html_content = fs::read_to_string("./assets/sample.html").unwrap();

// 納品書を解析
let invoice = html_parser::parse_invoice(&html_content).ok().unwrap();

println!("オーダーID: {}", invoice.order_id);
println!("注文日: {}", invoice.order_date);
println!("出荷日: {}", invoice.shipping_date);
println!("購入品数: {}", invoice.items.len());

// 合計金額を計算
println!("合計金額: {}円", invoice.total_price());
```

### JSON変換

```rust
use akizuki_invoice_parser::parser::html_parser;
use std::fs;

let html_content = fs::read_to_string("./assets/sample.html").unwrap();
let invoice = html_parser::parse_invoice(&html_content);

// JSON形式で出力
let json = invoice.to_json().unwrap();
println!("{}", json);

// 整形されたJSON
let json_pretty = invoice.to_json_pretty().unwrap();
println!("{}", json_pretty);
```

### 個別データの取得

```rust
use akizuki_invoice_parser::parser::html_parser;
use std::fs;

let html_content = fs::read_to_string("./assets/sample.html").unwrap();

// オーダーIDのみ取得
let order_id = html_parser::parse_invoice_order_id(&html_content);
println!("オーダーID: {:?}", order_id);

// 注文日のみ取得
let order_date = html_parser::parse_order_date(&html_content);
println!("注文日: {:?}", order_date);

// 出荷日のみ取得
let shipping_date = html_parser::parse_shipping_date(&html_content);
println!("出荷日: {:?}", shipping_date);

// 購入品リストのみ取得
let items = html_parser::parse_items(&html_content);
println!("購入品数: {}", items.len());
```

## データ構造

### Invoice

```rust
pub struct Invoice {
    pub order_id: String,
    pub order_date: NaiveDate,
    pub shipping_date: NaiveDate,
    pub items: Vec<Item>,
}
```

### Item

```rust
pub struct Item {
    pub catalog_id: String,
    pub name: String,
    pub img_url: String,
    pub quantity: u32,
    pub unit_price: u32,
}
```

## JSON出力例

```json
{
  "order_id": "EC250831-391903137-01",
  "order_date": "2025-08-31",
  "shipping_date": "2025-09-01",
  "items": [
    {
      "catalog_id": "108617",
      "name": "丸ピンICソケット ( 6P)",
      "img_url": "/img/goods/M/108617.jpg",
      "quantity": 10,
      "unit_price": 20
    },
    {
      "catalog_id": "113582",
      "name": "積層セラミックコンデンサー 0.1μF50V X7R 2.54mm",
      "img_url": "/img/goods/M/113582.jpg",
      "quantity": 2,
      "unit_price": 100
    }
  ]
}
```

## 依存関係

- `chrono` - 日付処理
- `scraper` - HTML解析
- `serde` - シリアライゼーション
- `serde_json` - JSON変換

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。
