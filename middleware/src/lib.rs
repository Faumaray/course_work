use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub kind: DatabaseRequest,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseRequest {
    Initial = 1,
    LocationsByGame = 2,
    ListsByLocation = 3,
    Success = 0,
    Error = -1,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody {
    pub kind: DatabaseRequest,
    pub err: Option<String>,
    pub games: Option<Vec<String>>,
    pub locations: Option<Vec<(String, Vec<u8>, String)>>,
    pub mobs: Option<Vec<(String, Vec<u8>, String)>>,
    pub loot: Option<Vec<(String, Vec<u8>, String)>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditRequestBody {
    pub kind: EditRequest,
    pub edit_type: EditType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub original: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditResponseBody {
    pub kind: EditRequest,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditRequest {
    Initial = 1,
    ChangeName = 2,
    ChangeDescription = 3,
    ChangePreview = 4,
    Success = 0,
    Error = -1,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditType {
    Game = 1,
    Location = 2,
    Mob = 3,
    Loot = 4,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBody {
    pub kind: AddRequest,
    pub game_list: Option<Vec<String>>,
    pub location_list: Option<Vec<String>>,
    pub mob_list: Option<Vec<String>>,
    pub game_name: Option<String>,
    pub location_name: Option<Vec<String>>,
    pub mob_name: Option<Vec<String>>,
    pub loot_name: Option<String>,
    pub description: Option<String>,
    pub preview: Option<Vec<u8>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRequest {
    AddGame = 1,
    AddLocation = 2,
    AddMob = 3,
    AddLoot = 4,
    GetGameList = 5,
    GetLocationList = 6,
    GetMobList = 7,
    Success = 0,
    Error = -1,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBody {
    pub kind: DeleteRequest,
    pub games: Option<Vec<(i32, String)>>,
    pub locations: Option<Vec<(i32, i32, String)>>,
    pub mobs: Option<Vec<(i32, Option<i32>, String)>>,
    pub loots: Option<Vec<(i32, Option<i32>, Option<i32>, String)>>,
    pub id: Option<i32>,
    pub locationid: Option<i32>,
    pub mobid: Option<i32>,
    pub name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRequest {
    Initial = 1,
    Game = 2,
    Location = 3,
    Mob = 4,
    Loot = 5,
    Success = 0,
    Error = -1,
}
