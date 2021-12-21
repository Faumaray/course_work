mod db;
use actix_files::{Files, NamedFile};
use actix_web::{
    post,
    web::{get, post, Data, Json},
    App, HttpServer, Responder, Result,
};
use db::*;
#[allow(clippy::unused_async)]
async fn index() -> Result<NamedFile> {
    println!("Serving at 127.0.0.1:9999");
    Ok(NamedFile::open("./frontend/index.html")?)
}
#[post("/")]
async fn viewer(
    data: Data<DatabaseState>,
    request_data: Json<middleware::RequestBody>,
) -> impl Responder {
    match request_data.kind {
        middleware::DatabaseRequest::Error => {
            return Json(middleware::ResponseBody {
                err: Some(format!("Not Implemented")),
                kind: middleware::DatabaseRequest::Error,
                games: None,
                locations: None,
                mobs: None,
                loot: None,
            });
        }
        middleware::DatabaseRequest::Initial => match db::get_all_games(&data.connection).await {
            Ok(game_list) => {
                let mut game_names = Vec::with_capacity(game_list.len());
                for model in game_list {
                    game_names.push(model.game_name);
                }
                return Json(middleware::ResponseBody {
                    err: None,
                    kind: middleware::DatabaseRequest::Initial,
                    games: Some(game_names.clone()),
                    locations: None,
                    mobs: None,
                    loot: None,
                });
            }
            Err(err_msg) => {
                return Json(middleware::ResponseBody {
                    err: Some(format!("{}", err_msg)),
                    kind: middleware::DatabaseRequest::Error,
                    games: None,
                    locations: None,
                    mobs: None,
                    loot: None,
                });
            }
        },
        middleware::DatabaseRequest::LocationsByGame => {
            println!("Locations by {}", request_data.name.clone());
            match db::get_all_locations_by_game(&data.connection, request_data.name.clone()).await {
                Ok(locations_list) => {
                    let mut locations = Vec::with_capacity(locations_list.len());
                    for model in locations_list {
                        locations.push((
                            model.location_name,
                            model.on_map.unwrap_or_default(),
                            model.descr.unwrap_or_default(),
                        ));
                    }
                    return Json(middleware::ResponseBody {
                        err: None,
                        kind: middleware::DatabaseRequest::LocationsByGame,
                        games: None,
                        locations: Some(locations.clone()),
                        mobs: None,
                        loot: None,
                    });
                }
                Err(err_msg) => {
                    return Json(middleware::ResponseBody {
                        err: Some(format!("{}", err_msg)),
                        kind: middleware::DatabaseRequest::Error,
                        games: None,
                        locations: None,
                        mobs: None,
                        loot: None,
                    });
                }
            }
        }
        middleware::DatabaseRequest::ListsByLocation => {
            match db::get_all_mobs_by_location(&data.connection, request_data.name.clone()).await {
                Ok(mobs_list) => {
                    let mut mobs = Vec::with_capacity(mobs_list.len());
                    for model in mobs_list {
                        mobs.push((
                            model.mob_name,
                            model.preview.unwrap_or_default(),
                            model.desct.unwrap_or_default(),
                        ));
                    }
                    match db::get_all_loot_by_location(&data.connection, request_data.name.clone())
                        .await
                    {
                        Ok(loot_list) => {
                            let mut loots = Vec::with_capacity(loot_list.len());
                            for model in loot_list {
                                loots.push((
                                    model.loot_name,
                                    model.preview.unwrap_or_default(),
                                    model.descr.unwrap_or_default(),
                                ));
                            }
                            return Json(middleware::ResponseBody {
                                err: None,
                                kind: middleware::DatabaseRequest::ListsByLocation,
                                games: None,
                                locations: None,
                                mobs: Some(mobs.clone()),
                                loot: Some(loots.clone()),
                            });
                        }
                        Err(err_msg) => {
                            return Json(middleware::ResponseBody {
                                err: Some(format!("{}", err_msg)),
                                kind: middleware::DatabaseRequest::Error,
                                games: None,
                                locations: None,
                                mobs: None,
                                loot: None,
                            });
                        }
                    }
                }
                Err(err_msg) => {
                    return Json(middleware::ResponseBody {
                        err: Some(format!("{}", err_msg)),
                        kind: middleware::DatabaseRequest::Error,
                        games: None,
                        locations: None,
                        mobs: None,
                        loot: None,
                    });
                }
            }
        }
        middleware::DatabaseRequest::Success => todo!(),
    }
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
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(Files::new("/pkg", "./frontend/pkg"))
            .service(viewer)
            .default_service(get().to(index))
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}
