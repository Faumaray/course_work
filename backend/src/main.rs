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
#[post("/add")]
async fn add(data: Data<DatabaseState>, request_data: Json<middleware::AddBody>) -> impl Responder {
    match request_data.kind {
        middleware::AddRequest::AddGame => {
            match db::add_game(&data.connection, request_data.game_name.clone().unwrap()).await {
                Ok(_) => {
                    return Json(middleware::AddBody {
                        kind: middleware::AddRequest::Success,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: None,
                        mob_name: None,
                        loot_name: None,
                        description: None,
                        preview: None,
                    })
                }
                Err(_) => {
                    return Json(middleware::AddBody {
                        kind: middleware::AddRequest::Error,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: None,
                        mob_name: None,
                        loot_name: None,
                        description: None,
                        preview: None,
                    })
                }
            }
        }
        middleware::AddRequest::AddLocation => todo!(),
        middleware::AddRequest::AddMob => todo!(),
        middleware::AddRequest::AddLoot => todo!(),
        middleware::AddRequest::GetGameList => match db::get_all_games(&data.connection).await {
            Ok(games) => {
                let mut out = Vec::with_capacity(games.len());
                for model in games {
                    out.push(model.game_name);
                }
                return Json(middleware::AddBody {
                    kind: middleware::AddRequest::GetGameList,
                    game_list: Some(out),
                    location_list: None,
                    mob_list: None,
                    game_name: None,
                    location_name: None,
                    mob_name: None,
                    loot_name: None,
                    description: None,
                    preview: None,
                });
            }
            Err(err) => {
                return Json(middleware::AddBody {
                    kind: middleware::AddRequest::Error,
                    game_list: None,
                    location_list: None,
                    mob_list: None,
                    game_name: None,
                    location_name: None,
                    mob_name: None,
                    loot_name: None,
                    description: None,
                    preview: None,
                })
            }
        },
        middleware::AddRequest::GetLocationList => todo!(),
        middleware::AddRequest::GetMobList => todo!(),
        _ => {
            return Json(middleware::AddBody {
                kind: middleware::AddRequest::Error,
                game_list: None,
                location_list: None,
                mob_list: None,
                game_name: None,
                location_name: None,
                mob_name: None,
                loot_name: None,
                description: None,
                preview: None,
            })
        }
    }
}
#[post("/edit")]
async fn editor(
    data: Data<DatabaseState>,
    request_data: Json<middleware::EditRequestBody>,
) -> impl Responder {
    match request_data.kind {
        middleware::EditRequest::Initial => match request_data.edit_type {
            middleware::EditType::Game => {
                return Json(middleware::EditResponseBody {
                    kind: middleware::EditRequest::Error,
                    description: None,
                    image: None,
                    name: request_data.name.clone(),
                });
            }
            middleware::EditType::Location => {
                match db::get_location_by_name(&data.connection, request_data.name.clone().unwrap())
                    .await
                {
                    Ok(data) => {
                        if let Some(model) = data {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Initial,
                                description: model.descr,
                                image: model.on_map,
                                name: Some(model.location_name),
                            });
                        } else {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Error,
                                description: None,
                                image: None,
                                name: None,
                            });
                        }
                    }
                    Err(err) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Mob => {
                match db::get_mob_by_name(&data.connection, request_data.name.clone().unwrap())
                    .await
                {
                    Ok(data) => {
                        if let Some(model) = data {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Initial,
                                description: model.desct,
                                image: model.preview,
                                name: Some(model.mob_name),
                            });
                        } else {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Error,
                                description: None,
                                image: None,
                                name: None,
                            });
                        }
                    }
                    Err(err) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Loot => {
                match db::get_loot_by_name(&data.connection, request_data.name.clone().unwrap())
                    .await
                {
                    Ok(data) => {
                        if let Some(model) = data {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Initial,
                                description: model.descr,
                                image: model.preview,
                                name: Some(model.loot_name),
                            });
                        } else {
                            return Json(middleware::EditResponseBody {
                                kind: middleware::EditRequest::Error,
                                description: None,
                                image: None,
                                name: None,
                            });
                        }
                    }
                    Err(err) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
        },
        middleware::EditRequest::ChangeName => match request_data.edit_type {
            middleware::EditType::Game => todo!(),
            middleware::EditType::Location => {
                match db::change_location(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Mob => {
                match db::change_mob(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Loot => {
                match db::change_loot(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
        },
        middleware::EditRequest::ChangeDescription => match request_data.edit_type {
            middleware::EditType::Game => todo!(),
            middleware::EditType::Location => {
                match db::change_location(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Mob => {
                match db::change_mob(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Loot => {
                match db::change_loot(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
        },
        middleware::EditRequest::ChangePreview => match request_data.edit_type {
            middleware::EditType::Game => todo!(),
            middleware::EditType::Location => {
                match db::change_location(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Mob => {
                match db::change_mob(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
            middleware::EditType::Loot => {
                match db::change_loot(
                    &data.connection,
                    request_data.name.clone(),
                    request_data.description.clone(),
                    request_data.image.clone(),
                    request_data.original.clone(),
                )
                .await
                {
                    Ok(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Success,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                    Err(_) => {
                        return Json(middleware::EditResponseBody {
                            kind: middleware::EditRequest::Error,
                            description: None,
                            image: None,
                            name: None,
                        })
                    }
                }
            }
        },
        middleware::EditRequest::Success => todo!(),
        middleware::EditRequest::Error => todo!(),
    }
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
            .service(editor)
            .service(add)
            .default_service(get().to(index))
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}
