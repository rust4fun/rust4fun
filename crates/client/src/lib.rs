#![allow(clippy::all)] // 自動生成モジュールのため許容する
pub mod auth_codegen;
pub mod codegen;

pub use auth_codegen::types as auth_types;
pub use auth_codegen::{Client as AuthClient, ResponseValue as AuthResponseValue};
pub use codegen::types;
pub use codegen::{Client, ClientPlanetsExt, ClientRootExt, ResponseValue};

// re-export
pub use progenitor::progenitor_client::Error;
