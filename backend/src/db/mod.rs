pub mod prelude;

pub mod games;
pub mod locations;
pub mod loot;
pub mod mobs;
pub mod sqlx_migrations;
use dotenv::dotenv;
use sea_orm::{sea_query::Expr, *};
use std::env;

pub async fn estabilish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url).await.unwrap()
}

//Игры
pub async fn get_all_games(
    connection: &DatabaseConnection, /*Входные для поиска */
) -> Result<Vec<games::Model>, DbErr> {
    games::Entity::find().all(connection).await
}

pub async fn add_game(connection: &DatabaseConnection, name: String) -> Result<(), DbErr> {
    let game = games::ActiveModel {
        game_name: Set(name.clone().to_owned()),
        ..Default::default()
    };
    games::Entity::insert(game).exec(connection).await?;
    Ok(())
}

//Локации

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
pub async fn add_location(
    connection: &DatabaseConnection,
    game_name: String,
    description: Option<String>,
    location_name: String,
    on_map: Vec<u8>,
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
                gameid: Set(game.id.to_owned()),
                location_name: Set(location_name.to_owned()),
                descr: Set(description.to_owned()),
                on_map: Set(Some(on_map.to_owned())),
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

pub async fn add_mob(
    connection: &DatabaseConnection,
    loot_names: Option<Vec<String>>,
    mob_name: String,
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
                    desct: Set(description.clone().to_owned()),
                    preview: Set(preview.clone().to_owned()),
                    ..Default::default()
                };
                mobs.push(mob);
            }
        }
        mobs::Entity::insert_many(mobs).exec(connection).await?;
        if let Some(names_vec) = loot_names {
            for lname in names_vec {
                let loot: Option<loot::Model> = loot::Entity::find()
                    .filter(loot::Column::LootName.like(&lname))
                    .one(connection)
                    .await?;

                // Into ActiveModel
                let mut loot: loot::ActiveModel = loot.unwrap().into();
                let mob = mobs::Entity::find()
                    .filter(mobs::Column::MobName.like(&mob_name))
                    .one(connection)
                    .await?
                    .unwrap();

                loot.mobid = Set(Some(mob.id.to_owned()));

                // Update corresponding row in database using primary key value
                loot.update(connection).await?;
            }
        }
    }
    Ok(())
}
pub async fn get_all_mobs_by_location(
    connection: &DatabaseConnection,
    location: String, /*Входные для поиска */
) -> Result<Vec<mobs::Model>, DbErr> {
    if location.is_empty() {
        println!(
            "{:?}",
            mobs::Entity::find()
                .filter(Expr::col(mobs::Column::Locationid).is_null())
                .build(DatabaseBackend::Postgres)
                .to_string()
        );
        mobs::Entity::find()
            .filter(Expr::col(mobs::Column::Locationid).is_null())
            .all(connection)
            .await
    } else {
        let location = locations::Entity::find()
            .filter(locations::Column::LocationName.like(&location))
            .one(connection)
            .await?
            .unwrap();
        mobs::Entity::find()
            .filter(mobs::Column::Locationid.eq(Some(location.id)))
            .all(connection)
            .await
    }
}

//Лут

pub async fn get_all_loot_by_location(
    connection: &DatabaseConnection,
    location: String, /*Входные для поиска */
) -> Result<Vec<loot::Model>, DbErr> {
    if location.is_empty() {
        loot::Entity::find()
            .filter(Expr::col(loot::Column::Locationid).is_null())
            .all(connection)
            .await
    } else {
        let location = locations::Entity::find()
            .filter(locations::Column::LocationName.like(&location))
            .one(connection)
            .await?
            .unwrap();
        loot::Entity::find()
            .filter(loot::Column::Locationid.eq(Some(location.id)))
            .all(connection)
            .await
    }
}
