use serde::{Deserialize, Serialize};

/// Primitives Represents Information Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub informations_block: Option<String>,
    pub preview: Option<Vec<u8>>,
}

/// Enum Various Types of Body for View content Page by Types of Content[Response Type]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfoResponseBodyTypes {
    Game { info: Info },
    Location { info: Info },
    Mob { info: Info },
    Loot { info: Info },
}

/// Enum Various Types of Body for View content Page by Types of Content[Request Type]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfoRequestBodyTypes {
    Game {
        name: String,
    },
    Location {
        name: String,
        game: Option<String>,
    },
    Mob {
        name: String,
        game: Option<String>,
        location: Option<String>,
    },
    Loot {
        name: String,
        game: Option<String>,
        location: Option<String>,
        mob: Option<String>,
    },
}
/// For Various Types Of Lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetterRequestBodyTypes {
    GameList,
    LocationListByGame(String),
    LocationListByMob(String),
    LocationListByLoot(String),
    MobListByGame(String),
    MobListByLocation(String),
    MobListByLoot(String),
    LootListByGame(String),
    LootListByLocation(String),
    LootListByMob(String),
}

/// Enum Various Types of Body for AddNew content Page by Types of Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddNewContentRequestBodyTypes {
    Game {
        info: Info,
    },
    Location {
        info: Info,
        game: Option<String>,
    },
    Mob {
        info: Info,
        game: Option<String>,
        location: Vec<String>,
    },
    Loot {
        infp: Info,
        game: Option<String>,
        location: Vec<String>,
        mob: Vec<String>,
    },
}

/// Enum Various Types of Body for Delete content Page by Types of Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteContentRequestBodyTypes {
    Game { id: String },
    Location { id: String },
    Mob { id: String },
    Loot { id: String },
}

/// Enum Various Types of Body for Edit content Page by Types of Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditContentRequestBodyTypes {
    Game(EditContentPartTypes),
    Location(EditContentPartTypes),
    Mob(EditContentPartTypes),
    Loot(EditContentPartTypes),
}

/// Enum Various Part of Editable Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditContentPartTypes {
    Name { original: String, new: String },
    Description(String),
    Preview(Vec<u8>),
}

///Getter for Delete Page Response Body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetterDeleteBlockListResponseTypes {
    /// 1)Id: i32, 2)Name: String
    Game(Vec<(i32, String)>),
    /// 1)Id: i32, 2)GameId: String[i32 nullable], 3)Name: String
    Location(Vec<(i32, String, String)>),
    /// 1)Id: i32, 2)GameId: String[i32 nullable], 3)LocationId: String[i32 nullable] , 4)Name: String
    Mob(Vec<(i32, String, String, String)>),
    /// 1)Id: i32, 2)GameId: String[i32 nullable], 3)LocationId: String[i32 nullable], 4)MobId: String[i32 nullable] , 5)Name: String
    Loot(Vec<(i32, String, String, String, String)>),
}
/// Enum of Various Types Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetterDeleteBlockListRequestTypes {
    Game,
    Location,
    Mob,
    Loot,
}

/// Enum Various Types of Responses from Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Error(String, String),
    Success(String),
    //Registration(todo!("Add struct")),
    PageShow(InfoResponseBodyTypes),
    Getter(Vec<String>),
    GetterDeleteBlockList(GetterDeleteBlockListResponseTypes),
}

/// Enum Various Types of Requests from Frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    //Registration(todo!("Add struct")),
    PageShow(InfoRequestBodyTypes),
    Getter(GetterRequestBodyTypes),
    GetterDeleteBlockList(GetterDeleteBlockListRequestTypes),
    PageAdd(AddNewContentRequestBodyTypes),
    PageDelete(DeleteContentRequestBodyTypes),
    PadeEdit(EditContentRequestBodyTypes),
}
