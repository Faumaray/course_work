use crate::support::fetch_post as post;
use gloo_file::{callbacks::FileReader, File};
use middleware::GetterDeleteBlockListRequestTypes as State;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

pub struct AddNew {
    preview: Vec<u8>,
    readers: HashMap<String, FileReader>,
    state: State,
    name: String,
    description: String,
    game: String,
    gamelist: Vec<String>,
    locations: Vec<String>,
    locations_list: Vec<String>,
    mobs: Vec<String>,
    mobs_list: Vec<String>,
}

pub enum Msg {
    GameCreate,
    LocationCreate,
    MobCreate,
    LootCreate,
    SetSelectedGame(String),
    SetSelectedLocations(Vec<String>),
    SetSelectedMobs(Vec<String>),
    ChangeName(String),
    ChangeDescription(String),
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
    ReceiveResponse(Result<middleware::Response, reqwasm::Error>),
    ChangeState(State),
}
impl Component for AddNew {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&middleware::Request::Getter(
            middleware::GetterRequestBodyTypes::GameList,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/admin/add").await;
            Msg::ReceiveResponse(data)
        });

        Self {
            preview: Vec::new(),
            readers: HashMap::default(),
            state: State::Game,
            name: String::new(),
            description: String::new(),
            game: String::new(),
            gamelist: Vec::new(),
            locations: Vec::new(),
            locations_list: Vec::new(),
            mobs: Vec::new(),
            mobs_list: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameCreate => {
                let mut inf = middleware::Info {
                    name: self.name.clone(),
                    informations_block: None,
                    preview: None,
                };
                if !self.description.is_empty() {
                    inf.informations_block = Some(self.description.clone());
                }
                if !self.preview.is_empty() {
                    inf.preview = Some(self.preview.clone());
                }
                let body_type = middleware::AddNewContentRequestBodyTypes::Game { info: inf };
                let body = serde_json::to_string(&middleware::Request::PageAdd(body_type)).unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::LocationCreate => {
                let mut inf = middleware::Info {
                    name: self.name.clone(),
                    informations_block: None,
                    preview: None,
                };
                if !self.description.is_empty() {
                    inf.informations_block = Some(self.description.clone());
                }
                if !self.preview.is_empty() {
                    inf.preview = Some(self.preview.clone());
                }
                let body_type = middleware::AddNewContentRequestBodyTypes::Location {
                    info: inf,
                    game: self.game.clone(),
                };
                let body = serde_json::to_string(&middleware::Request::PageAdd(body_type)).unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::MobCreate => {
                let mut inf = middleware::Info {
                    name: self.name.clone(),
                    informations_block: None,
                    preview: None,
                };
                if !self.description.is_empty() {
                    inf.informations_block = Some(self.description.clone());
                }
                if !self.preview.is_empty() {
                    inf.preview = Some(self.preview.clone());
                }
                let body_type = middleware::AddNewContentRequestBodyTypes::Mob {
                    info: inf,
                    game: self.game.clone(),
                    location: self.locations.clone(),
                };
                let body = serde_json::to_string(&middleware::Request::PageAdd(body_type)).unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::LootCreate => {
                let mut inf = middleware::Info {
                    name: self.name.clone(),
                    informations_block: None,
                    preview: None,
                };
                if !self.description.is_empty() {
                    inf.informations_block = Some(self.description.clone());
                }
                if !self.preview.is_empty() {
                    inf.preview = Some(self.preview.clone());
                }
                let body_type = middleware::AddNewContentRequestBodyTypes::Loot {
                    info: inf,
                    game: self.game.clone(),
                    location: self.locations.clone(),
                    mob: self.mobs.clone(),
                };
                let body = serde_json::to_string(&middleware::Request::PageAdd(body_type)).unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::SetSelectedGame(new) => {
                self.game = new.clone();
                let body = serde_json::to_string(&middleware::Request::Getter(
                    middleware::GetterRequestBodyTypes::MobListByGame(new.clone()),
                ))
                .unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });
                let body = serde_json::to_string(&middleware::Request::Getter(
                    middleware::GetterRequestBodyTypes::LocationListByGame(new.clone()),
                ))
                .unwrap();
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/add").await;
                    Msg::ReceiveResponse(data)
                });

                true
            }
            Msg::SetSelectedLocations(new) => {
                self.locations = new;
                true
            }
            Msg::SetSelectedMobs(new) => {
                self.mobs = new;
                true
            }

            Msg::ChangeName(new) => {
                self.name = new;
                true
            }
            Msg::ChangeDescription(new) => {
                self.description = new;
                true
            }
            Msg::LoadedBytes(file_name, data) => {
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
                Ok(body) => match body {
                    middleware::Response::Success(_, _) => {
                        let history = ctx.link().history().unwrap();
                        history.back();
                        true
                    }
                    middleware::Response::Getter(body_type, vc) => match body_type {
                        middleware::GetterResponseBodyTypes::GameList => {
                            self.gamelist = vc;
                            true
                        }
                        middleware::GetterResponseBodyTypes::LocationList => {
                            self.locations_list = vc;
                            true
                        }
                        middleware::GetterResponseBodyTypes::MobList => {
                            self.mobs_list = vc;
                            true
                        }
                        _ => false,
                    },
                    _ => {
                        return false;
                    }
                },
                Err(_) => todo!(),
            },
            Msg::ChangeState(state) => {
                self.state = state;
                return true;
            }
        }
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inside = match self.state {
            State::Game => {
                html! {
                    <form class="create_container" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::GameCreate
                        } ) }>
                        <div class="Name_Row">
                            <div class="Col_25"><label for="name">{"Name:"}</label></div>
                            <div class="Col-75"><input value={self.name.clone()} type="text" name="name" id="name" required=true oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::ChangeName(input.value())
                                    })}/></div>
                        </div>
                        <div class="Description_Row">
                            <div class="Col-Des_25"><label for="description">{"Description:"}</label></div>
                            <div class="Col-Des_75"><textarea class={stylist::css!("min-width: 100%; min-height: 100%;")} id="description" value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::ChangeDescription(input.value())
                            })} /></div>
                        </div>
                        <div class="Icon_Row">
                            <br/>
                            <br/>
                            <br/>
                            <div class="Col-Icon_25"></div>
                            <div class="Col-Icon_75"><img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
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
                            /></div>
                        </div>
                        <div class="Submit_Row">
                            <center><button type="submit">{"Create"}</button></center>
                        </div>
                    </form>
                }
            }
            State::Location => {
                html! {
                    <form class="create_container" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::LocationCreate
                        } ) }>
                        <div class="Name_Row">
                            <div class="Col_25"><label for="name">{"Name:"}</label></div>
                            <div class="Col-75"><input value={self.name.clone()} type="text" name="name" id="name" required=true oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::ChangeName(input.value())
                                    })}/></div>
                        </div>
                        <div class="Description_Row">
                            <div class="Col-Des_25"><label for="description">{"Description:"}</label></div>
                            <div class="Col-Des_75"><textarea class={stylist::css!("min-width: 100%; min-height: 100%;")} id="description" value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::ChangeDescription(input.value())
                            })} /></div>
                        </div>
                        <div class="Icon_Row">
                            <br/>
                            <br/>
                            <br/>

                            <div class="Col-Icon_25"></div>
                            <div class="Col-Icon_75"><img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
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
                            /></div>
                        </div>
                        <div class="Games_Row">
                            <br/>
                            <br/>
                            <br/>
                            <div class="Col-Games_25">{"Select related Game:"}</div>
                            <div class="Col-Games_75">
                                <div class="select select-multiple">
                                    <select id="multi-select" multiple=false onchange={
                            ctx.link().callback(|e: Event| {
                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                Msg::SetSelectedGame(select.selected_options().item(0).unwrap().text_content().unwrap())
                        })}>
                                {self.gamelist.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>
                            </div>
                        </div>
                        <div class="Submit_Row">
                            <center><button type="submit">{"Create"}</button></center>
                        </div>
                    </form>
                }
            }
            State::Mob => {
                html! {
                     <form class="create_container" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::MobCreate
                        } ) }>
                        <div class="Name_Row">
                            <div class="Col_25"><label for="name">{"Name:"}</label></div>
                            <div class="Col-75"><input value={self.name.clone()} type="text" name="name" id="name" required=true oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::ChangeName(input.value())
                                    })}/></div>
                        </div>
                        <div class="Description_Row">
                            <div class="Col-Des_25"><label for="description">{"Description:"}</label></div>
                            <div class="Col-Des_75"><textarea class={stylist::css!("min-width: 100%; min-height: 100%;")} id="description" value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::ChangeDescription(input.value())
                            })} /></div>
                        </div>
                        <div class="Icon_Row">
                            <br/>
                            <br/>
                            <br/>

                            <div class="Col-Icon_25"></div>
                            <div class="Col-Icon_75"><img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
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
                            /></div>
                        </div>
                        <div class="Games_Row">
                            <br/>
                            <br/>
                            <br/>
                            <div class="Col-Games_25">{"Select related Game:"}</div>
                            <div class="Col-Games_75">
                                <div class="select select-multiple">
                                <select id="multi-select" multiple=false required=true onchange={
                            ctx.link().callback(|e: Event| {
                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                Msg::SetSelectedGame(select.selected_options().item(0).unwrap().text_content().unwrap())})}>
                                {self.gamelist.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>
                            </div>
                        </div>
                         <div class="Locations_Row">
                            <div class="Col-Locs_25">{"Select Related Locations(optional):"}</div>
                            <div class="Col-Locs_75">
                                 <div class="select select-multiple">
                                <select id="multi-select" multiple=true required=false onchange={
                            ctx.link().callback(|e: Event| {
                               let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                let mut selected = Vec::new();
                                for index in 0..select.selected_options().length()
                                {
                                    let input: web_sys::Element = select.item(index).unwrap();
                                    selected.push(input.text_content().unwrap_or_default());
                                };
                                Msg::SetSelectedLocations(selected.clone())})}>
                                {self.locations_list.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>
                            </div>
                        </div>
                        <div class="Submit_Row">
                            <center><button type="submit">{"Create"}</button></center>
                        </div>
                    </form>

                }
            }
            State::Loot => {
                html! {
                    <form class="create_container" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::LootCreate
                        } ) }>
                        <div class="Name_Row">
                            <div class="Col_25"><label for="name">{"Name:"}</label></div>
                            <div class="Col-75"><input value={self.name.clone()} type="text" name="name" id="name" required=true oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::ChangeName(input.value())
                                    })}/></div>
                        </div>
                        <div class="Description_Row">
                            <div class="Col-Des_25"><label for="description">{"Description:"}</label></div>
                            <div class="Col-Des_75"><textarea class={stylist::css!("min-width: 100%; min-height: 100%;")} id="description" value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::ChangeDescription(input.value())
                            })} /></div>
                        </div>
                        <div class="Icon_Row">
                            <br/>
                            <br/>
                            <br/>

                            <div class="Col-Icon_25"></div>
                            <div class="Col-Icon_75"><img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.preview.clone()))}/>
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
                            /></div>
                        </div>
                        <div class="Games_Row">
                            <br/>
                            <br/>
                            <br/>
                            <div class="Col-Games_25">{"Select related Game:"}</div>
                            <div class="Col-Games_75">
                                <div class="select select-multiple">
                                <select id="multi-select" multiple=false required=true onchange={
                            ctx.link().callback(|e: Event| {
                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                Msg::SetSelectedGame(select.selected_options().item(0).unwrap().text_content().unwrap())})}>
                                {self.gamelist.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>
                            </div>
                        </div>
                         <div class="Locations_Row">
                            <div class="Col-Locs_25">{"Select Related Locations(optional):"}</div>
                            <div class="Col-Locs_75">
                                <div class="select select-multiple">
                                <select id="multi-select" multiple=true required=false onchange={
                            ctx.link().callback(|e: Event| {
                               let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                let mut selected = Vec::new();
                                for index in 0..select.selected_options().length()
                                {
                                    let input: web_sys::Element = select.item(index).unwrap();
                                    selected.push(input.text_content().unwrap_or_default());
                                };
                                Msg::SetSelectedLocations(selected.clone())})}>
                                {self.locations_list.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>
                            </div>
                        </div>
                         <div class="Mobs_Row">
                            <div class="Col-Mobs_25">{"Select Related Mobs(Optional):"}</div>
                            <div class="Col-Mobs_75">
                                <div class="select select-multiple">
                                <select id="multi-select" multiple=true required=false onchange={
                            ctx.link().callback(|e: Event| {
                               let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                let mut selected = Vec::new();
                                for index in 0..select.selected_options().length()
                                {
                                    let input: web_sys::Element = select.item(index).unwrap();
                                    selected.push(input.text_content().unwrap_or_default());
                                };
                                Msg::SetSelectedMobs(selected.clone())})}>
                                {self.mobs_list.iter()
                                    .map(|location| {
                                        html! {
                                            <option value={location.clone()}>{location.clone()}</option>
                                        }}).collect::<Html>()}
                                    </select>
                                    <span class="focus"></span>
                                </div>

                            </div>
                        </div>
                        <div class="Submit_Row">
                            <center><button type="submit">{"Create"}</button></center>
                        </div>
                    </form>
                }
            }
        };
        html! {
                <div class="Content">
                    <div class="Name">
                        <center class={stylist::css!("width:100%;height:100%;margin-top: 5%;")}>
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
                        </center>
                    </div>
                    <div class="Inner_Content">
                        {inside}
                    </div>
                </div>
        }
    }
}
