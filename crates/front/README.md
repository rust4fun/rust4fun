# frontend
Rust のフロントエンドフレームワーク [`yew`](https://yew.rs/ja/) を使用したフロント実装コード群です。

## 設計思想
React の 関数コンポーネントの型で書いていこうと思っています。

## UI
UI こだわるとキリがないので、今の所何かしらあるものを使うことにしています。（慣れてきたら bootstrap から独自のコンポーネントづくりしたい）

とりあえず、 [`yew-bootstrap`](https://github.com/isosphere/yew-bootstrap) という crate があったので、未完成ですが使用しています。

## API client
自動生成で作成した `client` を使用しています。
