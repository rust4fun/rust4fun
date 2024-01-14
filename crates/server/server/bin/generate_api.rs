/// utoipa で定義した opeaapi をファイルに出力する bin ファイル
/// TODO: cli tool に移行する
use rust_study_server::router::ApiDoc;

use utoipa::OpenApi;

fn main() {
    let content = ApiDoc::openapi().to_yaml().expect("Err generate yaml");
    std::fs::write("./doc/specifications/api_v1.yml", content).expect("Err generate yaml");
}
