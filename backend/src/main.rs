mod db;
use std::env;

use actix_files::{Files, NamedFile};
use actix_web::{
    post,
    web::{get, post, Data, Json},
    App, HttpServer, Responder, Result,
};
use db::*;
#[post("/delete")]
async fn delete(
    data: Data<DatabaseState>,
    request_data: Json<middleware::Request>,
) -> impl Responder {
    return Json(middleware::Response::Error(
        String::from("Not yet implemented"),
        String::from("Basic"),
    ));

    /*match request_data {
        middleware::Request::PageShow(_) => todo!(),
        middleware::Request::Getter(_) => todo!(),
        middleware::Request::GetterDeleteBlockList(_) => todo!(),
        middleware::Request::PageAdd(_) => todo!(),
        middleware::Request::PageDelete(_) => todo!(),
        middleware::Request::PadeEdit(_) => todo!(),
    }*/
}
#[post("/add")]
async fn add(data: Data<DatabaseState>, request_data: Json<middleware::Request>) -> impl Responder {
    return Json(middleware::Response::Error(
        String::from("Not yet implemented"),
        String::from("Basic"),
    ));

    /*match request_data {
        middleware::Request::PageShow(_) => todo!(),
        middleware::Request::Getter(_) => todo!(),
        middleware::Request::GetterDeleteBlockList(_) => todo!(),
        middleware::Request::PageAdd(_) => todo!(),
        middleware::Request::PageDelete(_) => todo!(),
        middleware::Request::PadeEdit(_) => todo!(),
    }*/
}
#[post("/edit")]
async fn editor(
    data: Data<DatabaseState>,
    request_data: Json<middleware::Request>,
) -> impl Responder {
    return Json(middleware::Response::Error(
        String::from("Not yet implemented"),
        String::from("Basic"),
    ));

    /*match request_data {
        middleware::Request::PageShow(_) => todo!(),
        middleware::Request::Getter(_) => todo!(),
        middleware::Request::GetterDeleteBlockList(_) => todo!(),
        middleware::Request::PageAdd(_) => todo!(),
        middleware::Request::PageDelete(_) => todo!(),
        middleware::Request::PadeEdit(_) => todo!(),
    }*/
}
#[post("/")]
async fn viewer(
    data: Data<DatabaseState>,
    request_data: Json<middleware::Request>,
) -> impl Responder {
    return Json(middleware::Response::Error(
        String::from("Not yet implemented"),
        String::from("Basic"),
    ));

    /*match request_data {
        middleware::Request::PageShow(_) => todo!(),
        middleware::Request::Getter(_) => todo!(),
        middleware::Request::GetterDeleteBlockList(_) => todo!(),
        middleware::Request::PageAdd(_) => todo!(),
        middleware::Request::PageDelete(_) => todo!(),
        middleware::Request::PadeEdit(_) => todo!(),
    }*/
}
#[derive(Debug, Clone)]
struct DatabaseState {
    connection: sea_orm::DatabaseConnection,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {
        connection: connection,
    };
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(Files::new("/pkg", "./frontend/pkg"))
            .service(viewer)
            .service(editor)
            .service(add)
            .service(delete)
            .default_service(Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
// For Heroku
/*
  ip 0.0.0.0
*/
