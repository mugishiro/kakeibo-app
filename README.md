# 家計簿アプリ (Kakeibo App)

Rustで作成されたシンプルな家計簿管理アプリケーションです。

## 機能

- 収入・支出の登録
- カテゴリ別の集計
- 日付別の履歴表示
- JSONファイルでのデータ永続化

## 技術スタック

- **言語**: Rust
- **データ形式**: JSON
- **日付処理**: chrono
- **シリアライゼーション**: serde

## セットアップ

### 前提条件

- Rust 1.70以上
- Cargo

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/your-username/kakeibo-app.git
cd kakeibo-app

# 依存関係をインストール
cargo build

# アプリケーションを実行
cargo run
```

## 使用方法

1. アプリケーションを起動
2. メニューから操作を選択
   - 0: 登録
   - 1: 集計
3. 指示に従ってデータを入力

## プロジェクト構造

```
src/
├── main.rs          # エントリーポイント
├── lib.rs           # ライブラリ定義
├── models/          # データモデル
│   └── mod.rs
└── services/        # ビジネスロジック
    ├── mod.rs
    ├── io/          # 入出力処理
    └── validate/    # バリデーション
```

## ライセンス

MIT License