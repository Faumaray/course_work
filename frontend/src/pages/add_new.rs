use std::collections::HashMap;

use gloo_file::{callbacks::FileReader, File};
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

pub struct AddNew {
    game_list: Vec<String>,
    location_list: Vec<String>,
    mob_list: Vec<String>,
    game_name: Option<String>,
    locations_name: Vec<String>,
    location_name: String,
    selected_locations: Vec<String>,
    selected_mobs: Vec<String>,
    mob_name: String,
    mobs_name: Vec<String>,
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
    GetLocationList,
    SetSelectedLocations(Vec<String>),
    SetSelectedMobs(Vec<String>),
    ChangeState(State),
    ChangeGame(String),
    ChangeLocation(String),
    ChangeMob(String),
    ChangeLoot(String),
    ChangeDescription(String),
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
            locations_name: Vec::new(),
            location_name: String::new(),
            mob_name: String::new(),
            mobs_name: Vec::new(),
            loot_name: None,
            description: None,
            preview: Vec::new(),
            selected_locations: Vec::new(),
            selected_mobs: Vec::new(),
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
                        self.mob_list = Vec::new();
                        self.selected_mobs = Vec::new();
                        self.selected_locations = Vec::new();
                    }
                    middleware::AddRequest::GetMobList => {
                        self.mob_list = body.mob_list.unwrap();
                        self.selected_mobs = Vec::new();
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
            Msg::ChangeGame(name) => {
                self.game_name = Some(name);
                match self.state {
                    State::Game => return true,
                    State::Location => return true,
                    State::Unknown => return true,
                    _ => {
                        let body = serde_json::to_string(&middleware::AddBody {
                            kind: middleware::AddRequest::GetLocationList,
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
                        return true;
                    }
                }
            }
            Msg::ChangeLocation(name) => {
                self.location_name = name.clone();
            }
            Msg::ChangeDescription(desc) => {
                self.description = Some(desc.clone());
            }
            Msg::ChangeMob(name) => {
                self.mob_name = name.clone();
            }
            Msg::ChangeLoot(name) => {
                self.loot_name = Some(name.clone());
            }
            Msg::SetSelectedLocations(vc) => {
                self.selected_locations = vc.clone();
                match self.state {
                    State::Game => return true,
                    State::Location => return true,
                    State::Mob => return true,
                    State::Unknown => return true,
                    _ => {
                        let body = serde_json::to_string(&middleware::AddBody {
                            kind: middleware::AddRequest::GetMobList,
                            game_list: None,
                            location_list: None,
                            mob_list: None,
                            game_name: None,
                            location_name: Some(vc.clone()),
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
                }
            }
            Msg::SetSelectedMobs(vc) => {
                self.selected_mobs = vc.clone();
                return true;
            }
            Msg::GetLocationList => {
                if self.game_name.is_some() {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::GetLocationList,
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
            }
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
            Msg::AddLocation => {
                if self.preview.len() > 0 {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddLocation,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: self.game_name.clone(),
                        location_name: Some(vec![self.location_name.clone()]),
                        mob_name: None,
                        loot_name: None,
                        description: self.description.clone(),
                        preview: Some(self.preview.clone()),
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                } else {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddLocation,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: self.game_name.clone(),
                        location_name: Some(vec![self.location_name.clone()]),
                        mob_name: None,
                        loot_name: None,
                        description: self.description.clone(),
                        preview: None,
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                }
            }
            Msg::AddMob => {
                if self.preview.len() > 0 {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddMob,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: Some(self.selected_locations.clone()),
                        mob_name: Some(vec![self.mob_name.clone()]),
                        loot_name: None,
                        description: self.description.clone(),
                        preview: Some(self.preview.clone()),
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                } else {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddMob,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: Some(self.selected_locations.clone()),
                        mob_name: Some(vec![self.mob_name.clone()]),
                        loot_name: None,
                        description: self.description.clone(),
                        preview: None,
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                }
            }
            Msg::AddLoot => {
                if self.preview.len() > 0 {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddLoot,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: Some(self.selected_locations.clone()),
                        mob_name: Some(self.selected_mobs.clone()),
                        loot_name: self.loot_name.clone(),
                        description: self.description.clone(),
                        preview: Some(self.preview.clone()),
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                } else {
                    let body = serde_json::to_string(&middleware::AddBody {
                        kind: middleware::AddRequest::AddLoot,
                        game_list: None,
                        location_list: None,
                        mob_list: None,
                        game_name: None,
                        location_name: Some(self.selected_locations.clone()),
                        mob_name: Some(self.selected_mobs.clone()),
                        loot_name: self.loot_name.clone(),
                        description: self.description.clone(),
                        preview: None,
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                }
            }
            Msg::ChangeState(state) => {
                match state {
                    State::Unknown => todo!(),
                    State::Game => {
                        self.game_name = None;
                        self.state = state;
                    }
                    State::Location => {
                        self.game_name = None;
                        self.locations_name = Vec::new();
                        self.location_name = String::new();
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
                        self.mob_name = String::new();
                        self.locations_name = Vec::new();
                        self.mobs_name = Vec::new();
                        self.description = None;
                        self.preview = Vec::new();
                        self.game_list = Vec::new();
                        self.location_list = Vec::new();
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
                    State::Loot => {
                        self.game_name = None;
                        self.loot_name = None;
                        self.description = None;
                        self.preview = Vec::new();
                        self.game_list = Vec::new();
                        self.location_list = Vec::new();
                        self.mob_list = Vec::new();
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
                        <label for="name">{"Name:"} </label>
                            <input value={self.location_name.clone()} type="text" name="name" id="name" oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::ChangeLocation(input.value())
                        })}/>
                        <br/>
                        <div id="dropdown" class="dropdown">
                            <button class="adapt_list">{ "Games List "}</button>
                            <div id="content" class="content">
                                {games}//добавить добавить
                            </div>
                        </div>
                        <br/>
                        <textarea class={stylist::css!("witdh: 100px;height: 100px;")} value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::ChangeDescription(input.value())
                        })} />
                        <div class={stylist::css!("width: 250px; height: 250px; margin-left: 20px;")}>
                        <img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
                            <p>{ "Choose a file to upload" }</p>
                            <input type="file" multiple=false onchange={ctx.link().callback(move |e: Event| {
                                    let mut result = Vec::new();
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();

                                    if let Some(files) = input.files() {
                                        let files = js_sys::try_iter(&files)
                                            .unwrap()
                                            .unwrap()
                                            .map(|v| web_sys::File::from(v.unwrap()))
                                            .map(File::from);
                                        result.extend(files);
                                    }
                                    Msg::Files(result)
                                })}
                            />
                        </div>
                        <br/>


                            <button onclick={ctx.link().callback(|_|{
                                Msg::AddLocation
                            })}>{"Submit"}</button>
                    </div>
                }
            }
            State::Mob => {
                let locations = self
                    .location_list
                    .iter()
                    .map(|location| {
                        html! {
                             <option value={location.clone()}>{location.clone()}</option>
                        }
                    })
                    .collect::<Html>();
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
                        <label for="name">{"Name:"} </label>
                        <input value={self.mob_name.clone()} type="text" name="name" id="name" oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::ChangeMob(input.value())
                        })}/>
                        <br/>
                        <div id="dropdown" class="dropdown">
                            <button class="adapt_list">{ "Games List "}</button>
                            <div id="content" class="content">
                                {games}//добавить добавить
                            </div>
                        </div>
                        <br/>
                        <br/>
                        <select class={stylist::css!("width: 200px; height: 200px;")} multiple=true onchange={
                            ctx.link().callback(|e: Event| {
                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                let mut selected = Vec::new();
                                for index in 0..select.selected_options().length()
                                {
                                    let input: web_sys::Element = select.item(index).unwrap();
                                    selected.push(input.text_content().unwrap_or_default());
                                };
                                Msg::SetSelectedLocations(selected.clone())
                        })}>
                            {locations}
                        </select>
                        <br/>
                            <textarea class={stylist::css!("witdh: 100px;height: 100px;")} value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::ChangeDescription(input.value())
                            })} />
                            <div class={stylist::css!("width: 250px; height: 250px; margin-left: 20px;")}>
                                <img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
                                <p>{ "Choose a file to upload" }</p>
                                <input type="file" multiple=false onchange={ctx.link().callback(move |e: Event| {
                                        let mut result = Vec::new();
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();

                                        if let Some(files) = input.files() {
                                            let files = js_sys::try_iter(&files)
                                                .unwrap()
                                                .unwrap()
                                                .map(|v| web_sys::File::from(v.unwrap()))
                                                .map(File::from);
                                            result.extend(files);
                                        }
                                        Msg::Files(result)
                                    })}
                                />
                            </div>
                            <br/>
                            <button onclick={ctx.link().callback(|_|{
                                Msg::AddMob
                            })}>{"Submit"}</button>
                    </div>
                }
            }
            State::Loot => {
                let locations = self
                    .location_list
                    .iter()
                    .map(|location| {
                        html! {
                             <option value={location.clone()}>{location.clone()}</option>
                        }
                    })
                    .collect::<Html>();
                let mobs = self
                    .mob_list
                    .iter()
                    .map(|mob| {
                        html! {
                             <option value={mob.clone()}>{mob.clone()}</option>
                        }
                    })
                    .collect::<Html>();
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
                    <label for="name">{"Name:"} </label>
                        <input value={self.loot_name.clone()} type="text" name="name" id="name" oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::ChangeLoot(input.value())
                    })}/>
                    <br/>
                    <div id="dropdown" class="dropdown">
                        <button class="adapt_list">{ "Games List "}</button>
                        <div id="content" class="content">
                            {games}//добавить добавить
                        </div>
                    </div>
                    <br/>
                    <br/>
                    <select class={stylist::css!("witdh: 100px;height: 100px;")} multiple=true onchange={
                        ctx.link().callback(|e: Event| {
                            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                            let mut selected = Vec::new();
                            for index in 0..select.selected_options().length()
                            {
                                let input: web_sys::Element = select.item(index).unwrap();
                                selected.push(input.text_content().unwrap_or_default());
                            };
                            Msg::SetSelectedLocations(selected.clone())
                    })}>
                        {locations}
                    </select>
                    <select class={stylist::css!("witdh: 100px;height: 100px; margin-left: 50px;")} multiple=true onchange={
                        ctx.link().callback(|e: Event| {
                            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                            let mut selected = Vec::new();
                            for index in 0..select.selected_options().length()
                            {
                                let input: web_sys::Element = select.item(index).unwrap();
                                selected.push(input.text_content().unwrap_or_default());
                            };
                            Msg::SetSelectedMobs(selected.clone())
                    })}>
                        {mobs}
                    </select>
                        <br/>
                        <textarea class={stylist::css!("witdh: 100px;height: 100px;")} value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::ChangeDescription(input.value())
                        })} />
                        <div class={stylist::css!("width: 250px; height: 250px; margin-left: 20px;")}>
                            <img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
                            <p>{ "Choose a file to upload" }</p>
                            <input type="file" multiple=false onchange={ctx.link().callback(move |e: Event| {
                                    let mut result = Vec::new();
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();

                                    if let Some(files) = input.files() {
                                        let files = js_sys::try_iter(&files)
                                            .unwrap()
                                            .unwrap()
                                            .map(|v| web_sys::File::from(v.unwrap()))
                                            .map(File::from);
                                        result.extend(files);
                                    }
                                    Msg::Files(result)
                                })}
                            />
                        </div>
                        <br/>
                        <button onclick={ctx.link().callback(|_|{
                            Msg::AddLoot
                        })}>{"Submit"}</button>
                </div>
                    }
            }
        };
        html! {
            <>
                <div id="main_div" class="main_div">
                    <center class={stylist::css!("width:100%;height:100%;margin-top: 30%;")}>
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
