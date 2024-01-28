#![allow(clippy::all)] // 自動生成モジュールのため許容する
pub mod codegen;
pub mod root_codegen;

pub use codegen::types;
pub use codegen::{Client, ClientArticlesExt};
pub use root_codegen::types as rtypes;
pub use root_codegen::Client as RootClient;
