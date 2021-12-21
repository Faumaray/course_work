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
