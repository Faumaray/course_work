use serde::Serialize;
use yew::html::ImplicitClone;
pub async fn fetch_post(body: String, path: &str) -> Result<middleware::Response, reqwasm::Error> {
    let res: Result<middleware::Response, reqwasm::Error> =
        reqwasm::http::Request::post(format!("api{}", path).as_str())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .json()
            .await;
    res
}

#[derive(Clone, PartialEq, Serialize)]
pub enum UserType {
    Administrator(String),
    User(String),
    Unknown,
}
#[derive(Clone, PartialEq)]
pub struct InfoProp {
    pub user_type: crate::support::UserType,
    pub content_type: Option<EditTypes>,
    pub description: Option<String>,
    pub preview: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Serialize)]
pub enum EditTypes {
    Location = 1,
    Mob = 2,
    Loot = 3,
    Name = 10,
    Description = 11,
    Image = 12,
    Game = 0,
}
impl std::fmt::Display for EditTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditTypes::Location => write!(f, "location"),
            EditTypes::Mob => write!(f, "mob"),
            EditTypes::Loot => write!(f, "loot"),
            EditTypes::Game => write!(f, "game"),
            EditTypes::Name => write!(f, "name"),
            EditTypes::Description => write!(f, "description"),
            EditTypes::Image => write!(f, "image"),
        }
    }
}
impl ImplicitClone for EditTypes {}
impl std::str::FromStr for EditTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "location" => Ok(EditTypes::Location),
            "mob" => Ok(EditTypes::Mob),
            "loot" => Ok(EditTypes::Loot),
            "game" => Ok(EditTypes::Game),
            "name" => Ok(EditTypes::Name),
            "description" => Ok(EditTypes::Description),
            "image" => Ok(EditTypes::Image),
            _ => Err(String::from("Provided Not valid String")),
        }
    }
}
