#![allow(clippy::all)] // 自動生成モジュールのため許容する
pub mod codegen;
pub mod root_codegen;

pub use codegen::types;
pub use codegen::{Client, ClientArticlesExt, ClientRootExt, ResponseValue};
pub use root_codegen::types as auth_types;
pub use root_codegen::{Client as AuthClient, ClientAuthExt, ResponseValue as AuthResponseValue};

// re-export
pub use progenitor::progenitor_client::Error;
