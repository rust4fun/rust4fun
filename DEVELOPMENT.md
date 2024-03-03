# クイックスタート
**先に勉強会の organizations に招待してもらう必要があります。**

## リポジトリをクローンする
``` bash
git clone git@github.com:noshishiRust/rust-study-tools.git
```

## 開発環境の準備

### Rust
公式サイトより

https://www.rust-lang.org/ja/tools/install

### cargo-make
```bash
cargo install cargo-make
```

### Docker
公式サイトより

https://docs.docker.com/get-docker/

### Node.js
公式サイトより

https://nodejs.org/en

## 開発ツールのインストール
cargo-make を使用して、必要なパッケージ等のイントールを行います。  
以下コマンドで、 
- `cargo-shuttle`: shuttle runtime and cli
- `trunk`: build frontend tool
- `wasm-bindgen-cli`: build wasm cli
- `node_module`: npm package

がインストールされます。

```bash
cargo make init-project
```

## shuttle にログイン
[shuttle](https://www.shuttle.rs/)でアカウント作成後、`API KEY` を取得。 

ツールをインストールして、ログインする。
``` bash
cargo install shuttle

cargo-shuttle login
```

## ローカルで起動する
``` bash
cd rust4fun
cargo make run
```

## branch 運用
デフォルトブランチ (`dev`) から作業ブランチを作成し、 `dev` への PR を行う。

## 用語 
Study Spheres -> Planets -> talk or voice

## ルール
[Rust の実装における module 記載 ルール](./MODULE.md)

## Shuttle について
shuttle の crate や rust の version を更新すると、 deploy 時に restart が求められることがあります。 
この場合、 organization の admin のユーザーに相談してください。
