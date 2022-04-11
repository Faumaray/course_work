mod db;
use actix_session::{CookieSession, Session};
use rand::Rng;
use std::env;

use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use db::*;
#[post("api/admin/delete")]
async fn delete(
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::GetterDeleteBlockList(list_type) => match list_type {
            middleware::GetterDeleteBlockListRequestTypes::Game => {
                match db::get_all_games(&data.connection).await {
                    Ok(vc) => {
                        return web::Json(middleware::Response::GetterDeleteBlockList(
                            middleware::GetterDeleteBlockListResponseTypes::Game(
                                vc.iter()
                                    .map(|game| (game.id, game.game_name.clone()))
                                    .collect::<Vec<(i32, String)>>(),
                            ),
                        ))
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterDeleteBlockListRequestTypes::Location => {
                match db::get_all_locations(&data.connection).await {
                    Ok(vc) => {
                        return web::Json(middleware::Response::GetterDeleteBlockList(
                            middleware::GetterDeleteBlockListResponseTypes::Location(
                                vc.iter()
                                    .map(|location| {
                                        (
                                            location.id,
                                            location.gameid.clone(),
                                            location.location_name.clone(),
                                        )
                                    })
                                    .collect::<Vec<(i32, Option<i32>, String)>>(),
                            ),
                        ))
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterDeleteBlockListRequestTypes::Mob => {
                match db::get_all_mobs(&data.connection).await {
                    Ok(vc) => {
                        return web::Json(middleware::Response::GetterDeleteBlockList(
                            middleware::GetterDeleteBlockListResponseTypes::Mob(
                                vc.iter()
                                    .map(|mob| {
                                        (
                                            mob.id,
                                            mob.game_id.clone(),
                                            mob.locationid.clone(),
                                            mob.mob_name.clone(),
                                        )
                                    })
                                    .collect::<Vec<(i32, Option<i32>, Option<i32>, String)>>(),
                            ),
                        ))
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterDeleteBlockListRequestTypes::Loot => {
                match db::get_all_loot(&data.connection).await {
                    Ok(vc) => {
                        return web::Json(middleware::Response::GetterDeleteBlockList(
                            middleware::GetterDeleteBlockListResponseTypes::Loot(
                                vc.iter()
                                    .map(|loot| (loot.id,loot.game_id.clone(),loot.locationid.clone(),loot.mobid.clone(), loot.loot_name.clone()))
                                    .collect::<Vec<(i32, Option<i32>,Option<i32>,Option<i32>,String)>>(),
                            ),
                        ))
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
        },
        middleware::Request::PageDelete(delete_type) => match delete_type {
            middleware::DeleteContentRequestBodyTypes::Game { id } => {
                match db::delete_game(&data.connection, id).await {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::DeleteContentRequestBodyTypes::Location { id } => {
                match db::delete_location(&data.connection, id).await {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::DeleteContentRequestBodyTypes::Mob { id } => {
                match db::delete_mob(&data.connection, id).await {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::DeleteContentRequestBodyTypes::Loot { id } => {
                match db::delete_loot(&data.connection, id).await {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Not yet implemented")),
                String::from("Basic"),
            ));
        }
    }
}
#[post("api/admin/add")]
async fn add(
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::Getter(bodytype) => match bodytype {
            middleware::GetterRequestBodyTypes::GameList => {
                match db::get_all_games(&data.connection).await {
                    Ok(games) => {
                        let vs: Vec<String> = games
                            .iter()
                            .map(|game| game.game_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::GameList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::MobListByGame(gamename) => {
                match db::get_all_mobs_by_game(&data.connection, gamename).await {
                    Ok(mobs) => {
                        let vs: Vec<String> = mobs
                            .iter()
                            .map(|mob| mob.mob_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::MobList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::LootListByGame(gamename) => {
                match db::get_all_loots_by_game(&data.connection, gamename).await {
                    Ok(loots) => {
                        let vs: Vec<String> = loots
                            .iter()
                            .map(|loot| loot.loot_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::LootList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::LocationListByGame(gamename) => {
                match db::get_all_locations_by_game(&data.connection, gamename).await {
                    Ok(locations) => {
                        let vs: Vec<String> = locations
                            .iter()
                            .map(|loc| loc.location_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::LocationList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }

            _ => {
                return web::Json(middleware::Response::Error(
                    middleware::ErrorType::Custom(String::from("Wrong type")),
                    String::from("Not supported getter type for this page"),
                ));
            }
        },
        middleware::Request::PageAdd(body_type) => match body_type {
            middleware::AddNewContentRequestBodyTypes::Game { info } => {
                match db::add_game(
                    &data.connection,
                    info.name,
                    info.informations_block,
                    info.preview,
                )
                .await
                {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::AddNewContentRequestBodyTypes::Location { info, game } => {
                match db::add_location(
                    &data.connection,
                    info.name,
                    game,
                    info.informations_block,
                    info.preview,
                )
                .await
                {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::AddNewContentRequestBodyTypes::Mob {
                info,
                game,
                location,
            } => {
                match db::add_mob(
                    &data.connection,
                    info.name,
                    game,
                    info.informations_block,
                    info.preview,
                    location,
                )
                .await
                {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::AddNewContentRequestBodyTypes::Loot {
                info,
                game,
                location,
                mob,
            } => {
                match db::add_loot(
                    &data.connection,
                    info.name,
                    game,
                    info.informations_block,
                    info.preview,
                    location,
                    mob,
                )
                .await
                {
                    Ok(_) => {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Custon("".to_string()),
                            "".to_string(),
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Not yet implemented")),
                String::from("Basic"),
            ));
        }
    }
}
#[post("api/admin/edit/{type}/{part}/{name}")]
async fn editor(
    path: web::Path<(String, String, String)>,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::PageEdit(innertype) => match innertype {
            middleware::EditContentRequestBodyTypes::Mob(part) => match part {
                middleware::EditContentPartTypes::Name { original, new } => {
                    match db::change_mob(
                        &data.connection,
                        Some(new.clone()),
                        None,
                        None,
                        Some(original.clone()),
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Description(new) => {
                    match db::change_mob(
                        &data.connection,
                        Some(path.2.clone()),
                        Some(new.clone()),
                        None,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Preview(new) => {
                    match db::change_mob(
                        &data.connection,
                        Some(path.2.clone()),
                        None,
                        Some(new.clone()),
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
            },
            middleware::EditContentRequestBodyTypes::Game(part) => match part {
                middleware::EditContentPartTypes::Name { original, new } => {
                    match db::change_game(
                        &data.connection,
                        Some(new.clone()),
                        None,
                        None,
                        Some(original.clone()),
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Description(new) => {
                    match db::change_game(
                        &data.connection,
                        Some(path.2.clone()),
                        Some(new.clone()),
                        None,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Preview(new) => {
                    match db::change_game(
                        &data.connection,
                        Some(path.2.clone()),
                        None,
                        Some(new.clone()),
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
            },
            middleware::EditContentRequestBodyTypes::Loot(part) => match part {
                middleware::EditContentPartTypes::Name { original, new } => {
                    match db::change_loot(
                        &data.connection,
                        Some(new.clone()),
                        None,
                        None,
                        Some(original.clone()),
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Description(new) => {
                    match db::change_loot(
                        &data.connection,
                        Some(path.2.clone()),
                        Some(new.clone()),
                        None,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Preview(new) => {
                    match db::change_loot(
                        &data.connection,
                        Some(path.2.clone()),
                        None,
                        Some(new.clone()),
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
            },
            middleware::EditContentRequestBodyTypes::Location(part) => match part {
                middleware::EditContentPartTypes::Name { original, new } => {
                    match db::change_location(
                        &data.connection,
                        Some(new.clone()),
                        None,
                        None,
                        Some(original.clone()),
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Description(new) => {
                    match db::change_location(
                        &data.connection,
                        Some(path.2.clone()),
                        Some(new.clone()),
                        None,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
                middleware::EditContentPartTypes::Preview(new) => {
                    match db::change_location(
                        &data.connection,
                        Some(path.2.clone()),
                        None,
                        Some(new.clone()),
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Custon("".to_string()),
                                "".to_string(),
                            ));
                        }
                        Err(error) => {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("{}", error),
                            ));
                        }
                    }
                }
            },
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Wrong type")),
                String::from("Not supported getter type for this page"),
            ));
        }
    }
}
#[post("api/")]
async fn viewer(
    session: Session,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::LogOut { username } => {
            session.remove("username");
            session.remove("admin");
            return web::Json(middleware::Response::Success(
                middleware::SuccessType::Custon("".to_string()),
                "".to_string(),
            ));
        }
        middleware::Request::PageShow(_) => todo!(),
        middleware::Request::Getter(bodytype) => match bodytype {
            middleware::GetterRequestBodyTypes::CurrentUser => {
                let username: Option<String> = session.get::<String>("username").unwrap();
                let privelege: Option<bool> = session.get::<bool>("admin").unwrap();
                if let Some(name) = username {
                    if let Some(status) = privelege {
                        if status {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Admin,
                                name.clone(),
                            ));
                        } else {
                            return web::Json(middleware::Response::Success(
                                middleware::SuccessType::Admin,
                                name.clone(),
                            ));
                        }
                    }
                }
                return web::Json(middleware::Response::Success(
                    middleware::SuccessType::Custon("".to_string()),
                    "".to_string(),
                ));
            }
            middleware::GetterRequestBodyTypes::GameList => {
                match db::get_all_games(&data.connection).await {
                    Ok(games) => {
                        let vs: Vec<String> = games
                            .iter()
                            .map(|game| game.game_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::GameList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::MobListByGame(gamename) => {
                match db::get_all_mobs_by_game(&data.connection, gamename).await {
                    Ok(mobs) => {
                        let vs: Vec<String> = mobs
                            .iter()
                            .map(|mob| mob.mob_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::MobList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::LootListByGame(gamename) => {
                match db::get_all_loots_by_game(&data.connection, gamename).await {
                    Ok(loots) => {
                        let vs: Vec<String> = loots
                            .iter()
                            .map(|loot| loot.loot_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::LootList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::GetterRequestBodyTypes::LocationListByGame(gamename) => {
                match db::get_all_locations_by_game(&data.connection, gamename).await {
                    Ok(locations) => {
                        let vs: Vec<String> = locations
                            .iter()
                            .map(|loc| loc.location_name.clone())
                            .collect::<Vec<String>>();
                        return web::Json(middleware::Response::Getter(
                            middleware::GetterResponseBodyTypes::LocationList,
                            vs,
                        ));
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }

            _ => {
                return web::Json(middleware::Response::Error(
                    middleware::ErrorType::Custom(String::from("Wrong type")),
                    String::from("Not supported getter type for this page"),
                ));
            }
        },
        middleware::Request::GetterDeleteBlockList(_) => todo!(),
        middleware::Request::PageAdd(_) => todo!(),
        middleware::Request::PageDelete(_) => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Not yet implemented")),
                String::from("Basic"),
            ));
        }
        middleware::Request::PageEdit(_) => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Not yet implemented")),
                String::from("Basic"),
            ));
        }
        _ => {
            todo!()
        }
    }
}
#[post("api/{game}")]
async fn game_index(
    path: web::Path<String>,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::PageShow(infotype) => match infotype {
            middleware::InfoRequestBodyTypes::Game { name } => {
                match get_game_info_by_name(&data.connection, name).await {
                    Ok(game) => {
                        if let Some(gm) = game {
                            return web::Json(middleware::Response::PageShow(
                                middleware::InfoResponseBodyTypes::Game {
                                    info: middleware::Info {
                                        name: gm.game_name.clone(),
                                        informations_block: gm.description.clone(),
                                        preview: gm.preview.clone(),
                                    },
                                    background: gm.background.clone(),
                                },
                            ));
                        } else {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("Not Found"),
                            ));
                        }
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            _ => {
                return web::Json(middleware::Response::Error(
                    middleware::ErrorType::Custom(String::from("Bad Request")),
                    String::from("Not supported"),
                ));
            }
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Bad Request")),
                String::from("Not supported"),
            ));
        }
    }
}
#[post("api/{game}/{related}")]
async fn related_index(
    path: web::Path<(String, String)>,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::PageShow(req_type) => match req_type {
            middleware::InfoRequestBodyTypes::Location { name, game } => {
                match db::get_location_by_name_and_by_game(&data.connection, name.clone(), game)
                    .await
                {
                    Ok(location) => {
                        if let Some(lc) = location {
                            return web::Json(middleware::Response::PageShow(
                                middleware::InfoResponseBodyTypes::Location {
                                    info: middleware::Info {
                                        name: lc.location_name.clone(),
                                        informations_block: lc.description.clone(),
                                        preview: lc.preview.clone(),
                                    },
                                },
                            ));
                        } else {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("Not Found"),
                            ));
                        }
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::InfoRequestBodyTypes::Mob {
                name,
                game,
                location,
            } => {
                match db::get_mob_by_name_and_by_other(
                    &data.connection,
                    name.clone(),
                    game,
                    location,
                )
                .await
                {
                    Ok(mob) => {
                        if let Some(mb) = mob {
                            return web::Json(middleware::Response::PageShow(
                                middleware::InfoResponseBodyTypes::Mob {
                                    info: middleware::Info {
                                        name: mb.mob_name.clone(),
                                        informations_block: mb.description.clone(),
                                        preview: mb.preview.clone(),
                                    },
                                },
                            ));
                        } else {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("Not Found"),
                            ));
                        }
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            middleware::InfoRequestBodyTypes::Loot {
                name,
                game,
                location,
                mob,
            } => {
                match db::get_loot_by_name_and_by_other(
                    &data.connection,
                    name.clone(),
                    game,
                    location,
                    mob,
                )
                .await
                {
                    Ok(loot) => {
                        if let Some(lt) = loot {
                            return web::Json(middleware::Response::PageShow(
                                middleware::InfoResponseBodyTypes::Loot {
                                    info: middleware::Info {
                                        name: lt.loot_name.clone(),
                                        informations_block: lt.description.clone(),
                                        preview: lt.preview.clone(),
                                    },
                                },
                            ));
                        } else {
                            return web::Json(middleware::Response::Error(
                                middleware::ErrorType::Custom(String::from(
                                    "Database error occured",
                                )),
                                format!("Not Found"),
                            ));
                        }
                    }
                    Err(error) => {
                        return web::Json(middleware::Response::Error(
                            middleware::ErrorType::Custom(String::from("Database error occured")),
                            format!("{}", error),
                        ));
                    }
                }
            }
            _ => {
                return web::Json(middleware::Response::Error(
                    middleware::ErrorType::Custom(String::from("Bad Request")),
                    format!("Not Supported"),
                ));
            }
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Not yet implemented")),
                String::from("Basic"),
            ));
        }
    }
}
#[post("api/login")]
async fn login(
    session: Session,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::Login { email, password } => {
            match db::check_user(&data.connection, email.clone(), password).await {
                Ok(admin) => {
                    session.insert("username", email.clone());
                    session.insert("admin", admin);
                    if admin {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::Admin,
                            email.clone(),
                        ));
                    } else {
                        return web::Json(middleware::Response::Success(
                            middleware::SuccessType::User,
                            email.clone(),
                        ));
                    }
                }
                Err(error) => {
                    return web::Json(middleware::Response::Error(
                        middleware::ErrorType::Custom(String::from("Not yet implemented")),
                        format!("{}", error),
                    ));
                }
            }
        }
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Bad Request")),
                format!("Not Supported"),
            ));
        }
    }
}
#[post("api/register")]
async fn register(
    session: Session,
    data: web::Data<DatabaseState>,
    request_data: web::Json<middleware::Request>,
) -> impl Responder {
    match request_data.0 {
        middleware::Request::Registration {
            username,
            email,
            password,
        } => match db::add_user(
            &data.connection,
            username.clone(),
            email.clone(),
            password,
            false,
        )
        .await
        {
            Ok(admin) => {
                session.insert("username", username.clone());
                session.insert("admin", false);
                return web::Json(middleware::Response::Success(
                    middleware::SuccessType::User,
                    username.clone(),
                ));
            }
            Err(error) => {
                return web::Json(middleware::Response::Error(
                    middleware::ErrorType::Custom(String::from("Not yet implemented")),
                    format!("{}", error),
                ));
            }
        },
        _ => {
            return web::Json(middleware::Response::Error(
                middleware::ErrorType::Custom(String::from("Bad Request")),
                format!("Not Supported"),
            ));
        }
    }
}
async fn react_index() -> actix_files::NamedFile {
    actix_files::NamedFile::open("/home/faumaray/Projects/Rust/course_work/static/index.html")
        .unwrap()
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
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(CookieSession::signed(&private_key).secure(false))
            .service(viewer)
            .service(login)
            .service(register)
            .service(editor)
            .service(add)
            .service(delete)
            .service(game_index)
            .service(related_index)
            .service(Files::new("/", "./static").index_file("index.html"))
            .default_service(web::get().to(react_index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
// For Heroku
/*
  ip 0.0.0.0
*/
