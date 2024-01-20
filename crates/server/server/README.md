# server
バックエンド API サーバーの実装コード群です。

## デプロイ先
`shuttle.rs`

## API 仕様
[`utopia`](https://github.com/juhaku/utoipa) という crate を使用して、各サーバーのエンドポイント、スキーマを `openapi` 仕様で出力できるようにしています。書き方がやや助長になるので、書くのが面倒ですが。

`tools` を使用して `server` コードの `utopia` 部分を解析して `opqnapi` で出力 (./doc/specifications/api_v1.yml) しています。
