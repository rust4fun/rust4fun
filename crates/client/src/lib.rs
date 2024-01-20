#![allow(clippy::all)] // 自動生成モジュールのため許容する
pub mod codegen;

pub use codegen::types;
pub use codegen::{Client, ClientArticlesExt};
