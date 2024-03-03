# Rust の実装における module 記載 ルール

## 構成
`mod.rs` を使うのではなく、 `submodule.rs` と `submodule/` のパスでモジュールを構築する。

## 順番
```rust
use rust_study_db_connector as db;  // 1. alias (改行)

mod sub_mosule; 　　　　　　　　　　 　// 2. sub module の宣言 (改行)

pub use sub_mosule::hoge; 　　　 　　// 3. sub module の export (改行)

use crate::fuga::Fuga;              // 4. 使用する crate 内の module の宣言
use db::repository::FugaRepository; // 5. 使用する crate 外の workspace module の宣言
use uuid::Uuid;                     // 6. crate 外の module の宣言
```

## グループ化
ネストしなければ基本的にグループ化しても良い。（しなくても良い）  

```rust
// ok
use crate::fuga::{Fuga, FugaMemory, FugaHistory};     

// bad
use crate::fuga::{Fuga, kou::{KouModel, KouHistory}};

// ok
use crate::fuga::Fuga;
use crate::fuga::kou::{KouModel, KouHistory};
```

## prelude
極力使用したくないが、 `yew` など意識的に宣言しにくいものは使用してもよい。
