use std::collections::HashMap;

use gloo_file::{callbacks::FileReader, File};
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

pub struct AddNew {
    game_list: Vec<String>,
    location_list: Vec<String>,
    mob_list: Vec<String>,
    game_name: Option<String>,
    location_name: Vec<String>,
    mob_name: Vec<String>,
    loot_name: Option<String>,
    description: Option<String>,
    preview: Vec<u8>,
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    state: State,
}

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
    ReceiveResponse(Result<middleware::AddBody, reqwasm::Error>),
    AddGame,
    AddLocation,
    AddMob,
    AddLoot,
    ChangeState(State),
    ChangeGame(String),
}
enum State {
    Unknown,
    Game,
    Location,
    Mob,
    Loot,
}
pub async fn fetch(body: String) -> Result<middleware::AddBody, reqwasm::Error> {
    let res: Result<middleware::AddBody, reqwasm::Error> = reqwasm::http::Request::post("/add")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for AddNew {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            game_name: None,
            location_name: Vec::new(),
            mob_name: Vec::new(),
            loot_name: None,
            description: None,
            preview: Vec::new(),
            game_list: Vec::new(),
            location_list: Vec::new(),
            mob_list: Vec::new(),
            readers: HashMap::default(),
            files: vec![],
            state: State::Unknown,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                self.preview = data.clone();
                self.readers.remove(&file_name);
                return true;
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();
                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                return true;
            }
            Msg::ReceiveResponse(res) => match res {
                Ok(body) => match body.kind {
                    middleware::AddRequest::GetGameList => {
                        self.game_list = body.game_list.unwrap();
                    }
                    middleware::AddRequest::GetLocationList => {
                        self.location_list = body.location_list.unwrap();
                    }
                    middleware::AddRequest::GetMobList => {
                        self.mob_list = body.mob_list.unwrap();
                    }
                    middleware::AddRequest::Error => todo!(),
                    _ => {
                        let history = ctx.link().history().unwrap();
                        history.back();
                        return false;
                    }
                },
                Err(err) => return false,
            },
            Msg::ChangeGame(name) => self.game_name = Some(name),
            Msg::AddGame => {
                let body = serde_json::to_string(&middleware::AddBody {
                    kind: middleware::AddRequest::AddGame,
                    game_list: None,
                    location_list: None,
                    mob_list: None,
                    game_name: self.game_name.clone(),
                    location_name: None,
                    mob_name: None,
                    loot_name: None,
                    description: None,
                    preview: None,
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::AddLocation => todo!(),
            Msg::AddMob => todo!(),
            Msg::AddLoot => todo!(),
            Msg::ChangeState(state) => {
                match state {
                    State::Unknown => todo!(),
                    State::Game => {
                        self.game_name = None;
                        self.state = state;
                    }
                    State::Location => {
                        self.game_name = None;
                        self.location_name = Vec::new();
                        self.description = None;
                        self.preview = Vec::new();
                        self.game_list = Vec::new();
                        self.state = state;
                        let body = serde_json::to_string(&middleware::AddBody {
                            kind: middleware::AddRequest::GetGameList,
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
                        .unwrap();
                        ctx.link().send_future(async move {
                            let data = fetch(body).await;
                            Msg::ReceiveResponse(data)
                        });
                    }
                    State::Mob => {
                        self.game_name = None;
                        self.location_name = Vec::new();
                        self.mob_name = Vec::new();
                        self.description = None;
                        self.preview = Vec::new();
                        self.game_list = Vec::new();
                        self.location_list = Vec::new();
                        self.state = state;
                    }
                    State::Loot => {
                        self.game_name = None;
                        self.loot_name = None;
                        self.description = None;
                        self.preview = Vec::new();
                        self.game_list = Vec::new();
                        self.location_list = Vec::new();
                        self.mob_list = Vec::new();
                        self.state = state;
                    }
                }
                return true;
            }
        }
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inside = match self.state {
            State::Unknown => html! {},
            State::Game => {
                html! {
                    <div>
                        <label for="name">{"Edit Name:"} </label>
                        <input value={self.game_name.clone()} type="text" name="name" id="name" oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::ChangeGame(input.value())
                        })}/>
                        <button onclick={ctx.link().callback(|_|{
                            Msg::AddGame
                        })}>{"Submit"}</button>
                    </div>
                }
            }
            State::Location => {
                let games = self
                    .game_list
                    .iter()
                    .map(|game| {
                        html! {
                             <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                Msg::ChangeGame(input.inner_text())
                            })}>{game.clone()}</a>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <div>
                        <div id="dropdown" class="dropdown">
                        <button class="adapt_list">{ "Games List "}</button>
                        <div id="content" class="content">
                            {games}//добавить добавить
                        </div>
                        </div>
                    </div>
                }
            }
            State::Mob => todo!(),
            State::Loot => todo!(),
        };
        html! {
            <>
                <div id="main_div" class="main_div">
                    <center class={stylist::css!("margin-top: 10%;")}>
                        <p>{"Please select what your want to add:"}</p>
                        <input type="radio" id="games" value="Games" name="state" onclick={ctx.link().callback(|_|{
                            Msg::ChangeState(State::Game)
                        })}/>
                        <label for="games">{"Games"}</label>
                        <input type="radio" id="locations" value="Locations" name="state" onclick={ctx.link().callback(|_|{
                            Msg::ChangeState(State::Location)
                        })}/>
                        <label for="locations">{"Locations"}</label>
                        <input type="radio" id="mobs" value="Mobs" name="state" onclick={ctx.link().callback(|_|{
                            Msg::ChangeState(State::Mob)
                        })}/>
                        <label for="mobs">{"Mobs"}</label>
                        <input type="radio" id="loot" value="Loot" name="state" onclick={ctx.link().callback(|_|{
                            Msg::ChangeState(State::Loot)
                        })}/>
                        <label for="loot">{"Loot"}</label>
                        <br/><br/>
                        {inside}
                    </center>
                </div>
                <stylist::yew::Global css=r#"
                .main_div{
                    height:740px;
                    margin-top: 10%;
                    line-height: 1.5;
                    margin-left: 15%;
                    margin-right: 15%;
                    background: white;
                    border-radius: 12px;
                }
                .dropdown {
                    margin-left: 50px;
                    width: 40%;
                    margin-top: 50px;
                    margin-bottom: 50px;
                    display: inline-block;
                    position: relative;
                  }
                  .content {
                    display: none;
                    position: absolute;
                    width: 100%;
                    overflow: auto;
                    box-shadow: 0px 10px 10px 0px rgba(0, 0, 0, 0.4);
                  }
                  .dropdown:hover .content {
                    display: block;
                  }
                  .content a {
                    display: block;
                    color: #000000;
                    padding: 5px;
                    text-decoration: none;
                  }
                  .content a:hover {
                    color: #ffffff;
                    background-color: #00a4bd;
                  }
                "#/>
            </>
        }
    }
}
