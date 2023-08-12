#![allow(unused_imports)]
use axum::{
    extract,
    response::{self, IntoResponse},
    routing, Router,
};
use serde::{Deserialize, Serialize, __private::ser::serialize_tagged_newtype};
use std::{any, collections::HashMap, fs, net, sync, thread, time, vec};
mod cli;
mod typedefs;

#[tokio::main]
async fn main() {
    let _db = sync::Arc::new(typedefs::InMemModDB {
        regitered_modules: Vec::new(),
    });

    let cli_app = Router::new().route("/cli", routing::get(cli::cli_main));

    let app = Router::new()
        .merge(cli_app)
        .route(
            "/api/v1/to_dependencies",
            routing::post(crate::dependencies_handler),
        )
        .route("/api/v1/to_peer", routing::post(crate::peers_handler))
        .route(
            "/api/v1/register/:app_name/:id",
            routing::post(crate::registrations_handler),
        )
        .route("/", routing::get(crate::handle_root));

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3001));

    let results = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
    match results {
        Ok(_) => {}
        Err(e) => {
            log::error!("failed, result error {e}");
        }
    }
}

async fn registrations_handler(
    extract::Path((app_name, app_id)): extract::Path<(String, u32)>,
    extract::Json(mut payload): extract::Json<typedefs::RegisterRequest>,
) -> response::Response {
    payload.app_id = app_id;
    payload.app_name.clone_from(&app_name);
    let s = format!("request from '{app_name}:{app_id}' with data {payload}"); // payload needs std::fmt::Display
    s.into_response()
}

async fn peers_handler(
    extract::Path((app_name, app_id)): extract::Path<(String, u32)>,
    extract::Json(mut payload): extract::Json<typedefs::InfoToOtherMS>,
) -> response::Response {
    payload.app_id = app_id;
    payload.app_name.clone_from(&app_name);
    //
    // TODO: from here, we have to work on the data!
    String::from("to_peers not implemented").into_response()
}

async fn dependencies_handler(
    extract::Path((app_name, app_id)): extract::Path<(String, u32)>,
    extract::Json(mut payload): extract::Json<typedefs::InfoToOtherMS>,
) -> response::Response {
    payload.app_id = app_id;
    payload.app_name.clone_from(&app_name);
    //
    // TODO: from here, we have to work on the data!
    String::from("to_dependencies not implemented").into_response()
}

//
async fn handle_root() -> response::Response {
    let json_data = fs::read_to_string("./hosts.json");
    match json_data {
        Ok(data) => {
            let this = typedefs::GenericResponse {
                code: 0,
                data: serde_json::to_value(data).unwrap(),
            };
            response::Json(this).into_response()
        }
        Err(e) => {
            println!("handle_root(): failed, err {}", e);

            let this = typedefs::GenericResponse {
                code: -1,
                data: serde_json::to_value(typedefs::ThisEM {
                    host_id: 0,
                    hosts: HashMap::from([
                        (0, "127.0.0.1".to_string()),
                        (1, "127.0.0.1".to_string()),
                        (2, "127.0.0.1".to_string()),
                    ]),
                })
                .unwrap(),
            };
            response::Json(this).into_response()
        }
    }
    // TODO: from here, we have to work on the data!
}
