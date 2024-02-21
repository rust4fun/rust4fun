# Rust 勉強サイトの実装コード
このリポジトリは、 Rust 勉強用のサイトの実装コードです。

まだまだあらあら実装コードなので、ご容赦ください。

# 開発目的

Rust の勉強に関することをまとめるサイトを開発して、この勉強会で得た知識が反復できるようにする。あわよくば、学習サイトになればいいなと思う。

# 欲しい機能

メイン機能

- ユーザーログイン、ログアウト（認証機能）
- 記事をURL登録できる
- 各記事を表示する
    - qiita みたいに url を入力すると iframe で 画像とタイトルと概要が表示される
    - サムネイルとタイトルを自動取得
- 簡単なユーザーの共有機能

サブ機能

- いいね機能
- お気に入り登録機能
- 自分メモ作成機能

将来的な機能

- 検索機能
- 記事要約機能
- レコメンド
- 記事を作成できる機能
- github 連携

# workspace 構成

```txt
workspace
    ├ .github/ # github actions, template 置き場
    ├ .vscode/ # vscode の設定
    ├ crates/ # 各 rust コードの集まり
    │   ├ client # api server の request client lib クレート
    │   ├ front # yew の wasm クレート
    │   ├ server/ # backend api server のコードの集まり
    │       ├ db-connector # db 層の lib クレート
    │       ├ server # メイン bin クレート
    │   ├ shared # 共通で使用するコードを集めた lib クレート
    │   ├ tools # コードの自動生成など tools を集めた cli bin クレート
    ├ dist/ # サーブする静的ファイル置き場
    ├ doc/ # 各種ドキュメント置き場
    │   ├ er # server の DB の ER 図
    │   ├ specifications/ # server の openapi 仕様
    ├ .gitignore
    ├ .Cargo.lock
    ├ .Cargo.toml # cargo workspace 管理
    ├ LICENSE # 念の為作ってる
    ├ Makefile.toml # Rust のタスクランナー
    ├ README.md
    └ Shuttle.toml # cargo-shuttle project 管理
```

# 開発環境
- 言語 - Rust (v1.75.0 .. 常に最新を維持する予定)
- CI/CD - github
- Deploy - [shuttle](https://www.shuttle.rs/)
- 認証 - [Auth0](https://auth0.com/jp) (予定、直近は自前で用意)

# クイックスタート
**先に勉強会の organizations に招待してもらう必要があります。**

## Rust の開発環境を整える
公式サイトより

https://www.rust-lang.org/ja/tools/install

## リポジトリをクローンする
``` bash
git clone git@github.com:noshishiRust/rust-study-tools.git
```

## Docker のインストール
公式サイトより

https://docs.docker.com/get-docker/

## cargo-shuttle をインストールする
[shuttle](https://www.shuttle.rs/)でアカウント作成後、`API KEY` を取得。 

ツールをインストールして、ログインする。
``` bash
cargo install shuttle

cargo-shuttle login
```

## ローカルで起動する
データベースの使用には、 `Docker` が必要です。

``` bash
cd rust4fun
cargo-shuttle run --port 8080
```

# 必要な tools のインストール

## タスクランナー
- `cargo-make`

## wasm のビルド
- `trunk`
- `wasm-bindgen-cli`

# その他
## branch 運用

## cargo-shuttle について

## chat 
(Learn)Spheres -> Planets -> talk or voice
