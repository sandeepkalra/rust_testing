use axum::response::{IntoResponse, Response};

pub async fn cli_main() -> Response {
    String::from("cli command not supported yet").into_response()
}
