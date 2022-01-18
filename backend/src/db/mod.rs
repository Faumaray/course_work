pub mod prelude;

pub mod games;
pub mod locations;
pub mod loot;
pub mod mobs;
pub mod sqlx_migrations;
pub mod users;
use dotenv::dotenv;
use sea_orm::{sea_query::Expr, *};
use std::env;

pub async fn estabilish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url).await.unwrap()
}
pub async fn get_all_games(
    connection: &DatabaseConnection, /*Входные для поиска */
) -> Result<Vec<games::Model>, DbErr> {
    games::Entity::find().all(connection).await
}
pub async fn change_game(
    connection: &DatabaseConnection,
    name: Option<String>,
    description: Option<String>,
    image: Option<Vec<u8>>,
    original: Option<String>,
) -> Result<(), DbErr> {
    if let Some(na) = name {
        if let Some(orig) = original {
            let loc = games::Entity::find()
                .filter(games::Column::GameName.like(&orig))
                .one(connection)
                .await?;
            let mut loc: games::ActiveModel = loc.unwrap().into();
            loc.game_name = Set(na.to_owned());
            loc.update(connection).await?;
            return Ok(());
        }
        if let Some(desc) = description {
            let loc = games::Entity::find()
                .filter(games::Column::GameName.like(&na))
                .one(connection)
                .await?;
            let mut loc: games::ActiveModel = loc.unwrap().into();
            loc.description = Set(Some(desc.to_owned()));
            loc.update(connection).await?;
            return Ok(());
        }
        if let Some(img) = image {
            let loc = games::Entity::find()
                .filter(games::Column::GameName.like(&na))
                .one(connection)
                .await?;
            let mut loc: games::ActiveModel = loc.unwrap().into();
            loc.preview = Set(Some(img.to_owned()));
            loc.update(connection).await?;
            return Ok(());
        }
    }
    Err(DbErr::Custom(String::from("Name not provided")))
}

pub async fn get_game_info_by_name(
    connection: &DatabaseConnection,
    name: String,
) -> Result<Option<games::Model>, DbErr> {
    games::Entity::find()
        .filter(games::Column::GameName.like(&name))
        .one(connection)
        .await
}
pub async fn get_all_locations_by_game(
    connection: &DatabaseConnection,
    game_name: String, /*Входные для поиска */
) -> Result<Vec<locations::Model>, DbErr> {
    let game = games::Entity::find()
        .filter(games::Column::GameName.like(&game_name))
        .one(connection)
        .await?
        .unwrap();
    locations::Entity::find()
        .filter(locations::Column::Gameid.eq(game.id))
        .all(connection)
        .await
}
pub async fn get_location_by_name_and_by_game(
    connection: &DatabaseConnection,
    name: String,
    game_name: Option<String>, /*Входные для поиска */
) -> Result<Option<locations::Model>, DbErr> {
    if name.is_empty() {
        return Err(DbErr::RecordNotFound("Name not provided".to_string()));
    }
    if let Some(gname) = game_name {
        let game = games::Entity::find()
            .filter(games::Column::GameName.like(&gname))
            .one(connection)
            .await?
            .unwrap();
        locations::Entity::find()
            .filter(locations::Column::Gameid.eq(game.id))
            .filter(locations::Column::LocationName.eq(name))
            .one(connection)
            .await
    } else {
        locations::Entity::find()
            .filter(locations::Column::LocationName.eq(name))
            .one(connection)
            .await
    }
}

pub async fn get_all_mobs_by_game(
    connection: &DatabaseConnection,
    game_name: String, /*Входные для поиска */
) -> Result<Vec<mobs::Model>, DbErr> {
    let game = games::Entity::find()
        .filter(games::Column::GameName.like(&game_name))
        .one(connection)
        .await?
        .unwrap();
    mobs::Entity::find()
        .filter(mobs::Column::GameId.eq(game.id))
        .all(connection)
        .await
}
pub async fn get_mob_by_name_and_by_other(
    connection: &DatabaseConnection,
    name: String,
    game_name: Option<String>,
    location_name: Option<String>, /*Входные для поиска */
) -> Result<Option<mobs::Model>, DbErr> {
    if name.is_empty() {
        return Err(DbErr::RecordNotFound("Name not provided".to_string()));
    }
    let mut game = None;
    let mut location = None;
    if let Some(gname) = game_name {
        game = games::Entity::find()
            .filter(games::Column::GameName.like(&gname))
            .one(connection)
            .await?;
    }
    if let Some(lname) = location_name {
        location = locations::Entity::find()
            .filter(locations::Column::LocationName.like(&lname))
            .one(connection)
            .await?;
    }
    if game.is_some() && location.is_some() {
        mobs::Entity::find()
            .filter(mobs::Column::GameId.eq(game.unwrap().id))
            .filter(mobs::Column::Locationid.eq(location.unwrap().id))
            .filter(mobs::Column::MobName.eq(name))
            .one(connection)
            .await
    } else if game.is_some() {
        mobs::Entity::find()
            .filter(mobs::Column::GameId.eq(game.unwrap().id))
            .filter(mobs::Column::MobName.eq(name))
            .one(connection)
            .await
    } else if location.is_some() {
        mobs::Entity::find()
            .filter(mobs::Column::Locationid.eq(location.unwrap().id))
            .filter(mobs::Column::MobName.eq(name))
            .one(connection)
            .await
    } else {
        mobs::Entity::find()
            .filter(mobs::Column::MobName.eq(name))
            .one(connection)
            .await
    }
}

pub async fn get_all_loots_by_game(
    connection: &DatabaseConnection,
    game_name: String, /*Входные для поиска */
) -> Result<Vec<loot::Model>, DbErr> {
    let game = games::Entity::find()
        .filter(games::Column::GameName.like(&game_name))
        .one(connection)
        .await?
        .unwrap();
    loot::Entity::find()
        .filter(loot::Column::GameId.eq(game.id))
        .all(connection)
        .await
}
pub async fn get_loot_by_name_and_by_other(
    connection: &DatabaseConnection,
    name: String,
    game_name: Option<String>,
    mob_name: Option<String>,
    location_name: Option<String>, /*Входные для поиска */
) -> Result<Option<loot::Model>, DbErr> {
    if name.is_empty() {
        return Err(DbErr::RecordNotFound("Name not provided".to_string()));
    }
    let mut game = None;
    let mut location = None;
    let mut mob = None;
    if let Some(gname) = game_name {
        game = games::Entity::find()
            .filter(games::Column::GameName.like(&gname))
            .one(connection)
            .await?;
    }
    if let Some(lname) = location_name {
        location = locations::Entity::find()
            .filter(locations::Column::LocationName.like(&lname))
            .one(connection)
            .await?;
    }
    if let Some(mname) = mob_name {
        mob = mobs::Entity::find()
            .filter(mobs::Column::MobName.like(&mname))
            .one(connection)
            .await?;
    }

    if game.is_some() && location.is_some() && mob.is_some() {
        loot::Entity::find()
            .filter(loot::Column::GameId.eq(game.unwrap().id))
            .filter(loot::Column::Locationid.eq(location.unwrap().id))
            .filter(loot::Column::Mobid.eq(mob.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if game.is_some() && location.is_some() {
        loot::Entity::find()
            .filter(loot::Column::GameId.eq(game.unwrap().id))
            .filter(loot::Column::Locationid.eq(location.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if game.is_some() && mob.is_some() {
        loot::Entity::find()
            .filter(loot::Column::GameId.eq(game.unwrap().id))
            .filter(loot::Column::Mobid.eq(mob.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if mob.is_some() && location.is_some() {
        loot::Entity::find()
            .filter(loot::Column::Mobid.eq(mob.unwrap().id))
            .filter(loot::Column::Locationid.eq(location.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if location.is_some() {
        loot::Entity::find()
            .filter(loot::Column::Locationid.eq(location.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if game.is_some() {
        loot::Entity::find()
            .filter(loot::Column::GameId.eq(game.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else if mob.is_some() {
        loot::Entity::find()
            .filter(loot::Column::Mobid.eq(mob.unwrap().id))
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    } else {
        loot::Entity::find()
            .filter(loot::Column::LootName.eq(name))
            .one(connection)
            .await
    }
}

pub async fn check_user_privelege(
    connection: &DatabaseConnection,
    username: String,
) -> Result<bool, DbErr> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(connection)
        .await?;
    if let Some(usr) = user {
        return Ok(usr.admin);
    } else {
        return Ok(false);
    }
}
pub async fn check_user(
    connection: &DatabaseConnection,
    username: String,
    password: String,
) -> Result<bool, DbErr> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(connection)
        .await?;
    if let Some(usr) = user {
        if usr.password.eq(&password) {
            return Ok(usr.admin);
        } else {
            Err(DbErr::Custom("Wrong password".to_string()))
        }
    } else {
        return Err(DbErr::RecordNotFound("Username not found".to_string()));
    }
}

pub async fn add_user(
    connection: &DatabaseConnection,
    name: String,
    email: String,
    password: String,
    admin: bool,
) -> Result<(), DbErr> {
    let user = users::ActiveModel {
        username: Set(name.clone().to_owned()),
        password: Set(password.clone().to_owned()),
        email: Set(email.clone().to_owned()),
        admin: Set(admin.to_owned()),
        ..Default::default()
    };
    users::Entity::insert(user).exec(connection).await?;
    Ok(())
}
//Игры
pub async fn delete_game(connection: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let game: Option<games::Model> = games::Entity::find_by_id(id).one(connection).await?;
    let game: games::ActiveModel = game.unwrap().into();
    game.delete(connection).await
}

pub async fn add_game(
    connection: &DatabaseConnection,
    name: String,
    description: Option<String>,
    preview: Option<Vec<u8>>,
) -> Result<(), DbErr> {
    let game = games::ActiveModel {
        game_name: Set(name.clone().to_owned()),
        description: Set(description.to_owned()),
        preview: Set(preview.to_owned()),
        ..Default::default()
    };
    games::Entity::insert(game).exec(connection).await?;
    Ok(())
}

//Локации
pub async fn delete_location(
    connection: &DatabaseConnection,
    id: i32,
) -> Result<DeleteResult, DbErr> {
    let game: Option<locations::Model> = locations::Entity::find_by_id(id).one(connection).await?;
    let game: locations::ActiveModel = game.unwrap().into();
    game.delete(connection).await
}
pub async fn get_all_locations(
    connection: &DatabaseConnection,
) -> Result<Vec<locations::Model>, DbErr> {
    locations::Entity::find().all(connection).await
}
pub async fn change_location(
    connection: &DatabaseConnection,
    name: Option<String>,
    description: Option<String>,
    image: Option<Vec<u8>>,
    original: Option<String>,
) -> Result<(), DbErr> {
    if let Some(na) = name {
        if let Some(orig) = original {
            let loc = locations::Entity::find()
                .filter(locations::Column::LocationName.like(&orig))
                .one(connection)
                .await?;
            let mut loc: locations::ActiveModel = loc.unwrap().into();
            loc.location_name = Set(na.to_owned());
            loc.update(connection).await?;
            return Ok(());
        }
        if let Some(desc) = description {
            let loc = locations::Entity::find()
                .filter(locations::Column::LocationName.like(&na))
                .one(connection)
                .await?;
            let mut loc: locations::ActiveModel = loc.unwrap().into();
            loc.description = Set(Some(desc.to_owned()));
            loc.update(connection).await?;
            return Ok(());
        }
        if let Some(img) = image {
            let loc = locations::Entity::find()
                .filter(locations::Column::LocationName.like(&na))
                .one(connection)
                .await?;
            let mut loc: locations::ActiveModel = loc.unwrap().into();
            loc.preview = Set(Some(img.to_owned()));
            loc.update(connection).await?;
            return Ok(());
        }
    }
    Err(DbErr::Custom(String::from("Name not provided")))
}
pub async fn add_location(
    connection: &DatabaseConnection,
    location_name: String,
    game_name: String,
    description: Option<String>,
    on_map: Option<Vec<u8>>,
) -> Result<(), DbErr> {
    if let Some(game) = games::Entity::find()
        .filter(games::Column::GameName.like(&game_name))
        .one(connection)
        .await?
    {
        if locations::Entity::find()
            .filter(locations::Column::LocationName.like(&location_name))
            .one(connection)
            .await?
            .is_none()
        {
            let location = locations::ActiveModel {
                gameid: Set(Some(game.id.to_owned())),
                location_name: Set(location_name.to_owned()),
                description: Set(description.to_owned()),
                preview: Set(on_map.to_owned()),
                ..Default::default()
            };
            locations::Entity::insert(location).exec(connection).await?;
            return Ok(());
        }
        return Err(DbErr::Custom("Данная запись уже есть".to_string()));
    }

    Err(DbErr::RecordNotFound("Нет такой игры".to_string()))
}

//Мобы
pub async fn delete_mob(connection: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let game: Option<mobs::Model> = mobs::Entity::find_by_id(id).one(connection).await?;
    let game: mobs::ActiveModel = game.unwrap().into();
    loot::Entity::update_many()
        .col_expr(loot::Column::Mobid, Expr::value(Value::Int(None)))
        .filter(loot::Column::Mobid.eq(id))
        .exec(connection)
        .await?;
    game.delete(connection).await
}
pub async fn get_all_mobs(connection: &DatabaseConnection) -> Result<Vec<mobs::Model>, DbErr> {
    mobs::Entity::find().all(connection).await
}
pub async fn change_mob(
    connection: &DatabaseConnection,
    name: Option<String>,
    description: Option<String>,
    image: Option<Vec<u8>>,
    original: Option<String>,
) -> Result<(), DbErr> {
    if let Some(na) = name {
        if let Some(orig) = original {
            let mob = mobs::Entity::find()
                .filter(mobs::Column::MobName.like(&orig))
                .one(connection)
                .await?;
            let mut mob: mobs::ActiveModel = mob.unwrap().into();
            mob.mob_name = Set(na.to_owned());
            mob.update(connection).await?;
            return Ok(());
        }
        if let Some(desc) = description {
            let mob = mobs::Entity::find()
                .filter(mobs::Column::MobName.like(&na))
                .one(connection)
                .await?;
            let mut mob: mobs::ActiveModel = mob.unwrap().into();
            mob.description = Set(Some(desc.to_owned()));
            mob.update(connection).await?;
            return Ok(());
        }
        if let Some(img) = image {
            let mob = mobs::Entity::find()
                .filter(mobs::Column::MobName.like(&na))
                .one(connection)
                .await?;
            let mut mob: mobs::ActiveModel = mob.unwrap().into();
            mob.preview = Set(Some(img.to_owned()));
            mob.update(connection).await?;
            return Ok(());
        }
    }
    Err(DbErr::Custom(String::from("Name not provided")))
}
pub async fn add_mob(
    connection: &DatabaseConnection,
    mob_name: String,
    game_name: String,
    description: Option<String>,
    preview: Option<Vec<u8>>,
    locations: Vec<String>,
) -> Result<(), DbErr> {
    if locations.len() != 0 {
        let mut mobs = Vec::with_capacity(locations.len());
        for name in locations {
            if let Some(location) = locations::Entity::find()
                .filter(locations::Column::LocationName.like(&name))
                .one(connection)
                .await?
            {
                let mob = mobs::ActiveModel {
                    locationid: Set(Some(location.id.to_owned())),
                    mob_name: Set(mob_name.clone().to_owned()),
                    description: Set(description.clone().to_owned()),
                    preview: Set(preview.clone().to_owned()),
                    ..Default::default()
                };
                mobs.push(mob);
            }
        }
        mobs::Entity::insert_many(mobs).exec(connection).await?;
    } else {
        let mut mob = mobs::ActiveModel {
            locationid: Set(None),
            mob_name: Set(mob_name.clone().to_owned()),
            description: Set(description.clone().to_owned()),
            preview: Set(preview.clone().to_owned()),
            ..Default::default()
        };
        if let Some(game) = games::Entity::find()
            .filter(games::Column::GameName.like(&game_name))
            .one(connection)
            .await?
        {
            mob.game_id = Set(Some(game.id));
        }
        mobs::Entity::insert(mob).exec(connection).await?;
    }
    Ok(())
}
//Лут
pub async fn delete_loot(connection: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let game: Option<loot::Model> = loot::Entity::find_by_id(id).one(connection).await?;
    let game: loot::ActiveModel = game.unwrap().into();
    game.delete(connection).await
}
pub async fn get_all_loot(connection: &DatabaseConnection) -> Result<Vec<loot::Model>, DbErr> {
    loot::Entity::find().all(connection).await
}
pub async fn add_loot(
    connection: &DatabaseConnection,
    loot_name: String,
    game_name: String,
    description: Option<String>,
    preview: Option<Vec<u8>>,
    locations: Vec<String>,
    mobs: Vec<String>,
) -> Result<(), DbErr> {
    if locations.len() != 0 {
        let mut loot_list = Vec::with_capacity(locations.len() * mobs.len());
        for name in locations {
            if let Some(location) = locations::Entity::find()
                .filter(locations::Column::LocationName.like(&name))
                .one(connection)
                .await?
            {
                for mob in &mobs {
                    if let Some(mb) = mobs::Entity::find()
                        .filter(mobs::Column::MobName.like(&mob))
                        .one(connection)
                        .await?
                    {
                        let mut lt = loot::ActiveModel {
                            mobid: Set(Some(mb.id.to_owned())),
                            locationid: Set(Some(location.id.to_owned())),
                            loot_name: Set(loot_name.clone().to_owned()),
                            description: Set(description.clone().to_owned()),
                            preview: Set(preview.clone().to_owned()),
                            ..Default::default()
                        };
                        if let Some(game) = games::Entity::find()
                            .filter(games::Column::GameName.like(&game_name))
                            .one(connection)
                            .await?
                        {
                            lt.game_id = Set(Some(game.id));
                        }
                        loot_list.push(lt);
                    } else {
                        let mut lt = loot::ActiveModel {
                            locationid: Set(Some(location.id.to_owned())),
                            loot_name: Set(loot_name.clone().to_owned()),
                            description: Set(description.clone().to_owned()),
                            preview: Set(preview.clone().to_owned()),
                            ..Default::default()
                        };
                        if let Some(game) = games::Entity::find()
                            .filter(games::Column::GameName.like(&game_name))
                            .one(connection)
                            .await?
                        {
                            lt.game_id = Set(Some(game.id));
                        }
                        loot_list.push(lt);
                    }
                }
            }
        }
        loot::Entity::insert_many(loot_list.clone())
            .exec(connection)
            .await?;
    } else {
        let mut lt = loot::ActiveModel {
            loot_name: Set(loot_name.clone().to_owned()),
            description: Set(description.clone().to_owned()),
            preview: Set(preview.clone().to_owned()),
            ..Default::default()
        };
        if let Some(game) = games::Entity::find()
            .filter(games::Column::GameName.like(&game_name))
            .one(connection)
            .await?
        {
            lt.game_id = Set(Some(game.id));
        }
        loot::Entity::insert(lt).exec(connection).await?;
    }
    Ok(())
}
pub async fn change_loot(
    connection: &DatabaseConnection,
    name: Option<String>,
    description: Option<String>,
    image: Option<Vec<u8>>,
    original: Option<String>,
) -> Result<(), DbErr> {
    if let Some(na) = name {
        if let Some(orig) = original {
            let loot_one = loot::Entity::find()
                .filter(loot::Column::LootName.like(&orig))
                .one(connection)
                .await?;
            let mut loot_one: loot::ActiveModel = loot_one.unwrap().into();
            loot_one.loot_name = Set(na.to_owned());
            loot_one.update(connection).await?;
            return Ok(());
        }
        if let Some(desc) = description {
            let loot_one = loot::Entity::find()
                .filter(loot::Column::LootName.like(&na))
                .one(connection)
                .await?;
            let mut loot_one: loot::ActiveModel = loot_one.unwrap().into();
            loot_one.description = Set(Some(desc.to_owned()));
            loot_one.update(connection).await?;
            return Ok(());
        }
        if let Some(img) = image {
            let loot_one = loot::Entity::find()
                .filter(loot::Column::LootName.like(&na))
                .one(connection)
                .await?;
            let mut loot_one: loot::ActiveModel = loot_one.unwrap().into();
            loot_one.preview = Set(Some(img.to_owned()));
            loot_one.update(connection).await?;
            return Ok(());
        }
    }
    Err(DbErr::Custom(String::from("Name not provided")))
}
