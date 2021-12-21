use stylist::css;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlLinkElement, HtmlTextAreaElement};
use yew::{html::ImplicitClone, prelude::*};
use yew_router::{history::History, prelude::RouterScopeExt};
pub struct Viewer {
    games: Vec<String>,
    locations: Vec<(String, Vec<u8>, String)>,
    mobs: Vec<(String, Vec<u8>, String)>,
    loot: Vec<(String, Vec<u8>, String)>,
    content: Option<(Vec<u8>, String, String)>,
    edit_type: Edit,
}

pub enum Msg {
    EditIcon,
    EditName,
    EditDescription,
    AddNew,
    Delete,
    GameChange(String),
    LocationChange(String),
    MobChange(String),
    LootChange(String),
    ReceiveResponse(Result<middleware::ResponseBody, reqwasm::Error>),
}
#[derive(Clone, PartialEq)]
pub enum Edit {
    location = 1,
    mob = 2,
    loot = 3,
    name = 10,
    description = 11,
    image = 12,
    game = 0,
}
impl std::fmt::Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edit::location => write!(f, "location"),
            Edit::mob => write!(f, "mob"),
            Edit::loot => write!(f, "loot"),
            Edit::game => write!(f, "game"),
            Edit::name => write!(f, "name"),
            Edit::description => write!(f, "description"),
            Edit::image => write!(f, "image"),
        }
    }
}
impl ImplicitClone for Edit {}
impl std::str::FromStr for Edit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "location" => Ok(Edit::location),
            "mob" => Ok(Edit::mob),
            "loot" => Ok(Edit::loot),
            "game" => Ok(Edit::game),
            "name" => Ok(Edit::name),
            "description" => Ok(Edit::description),
            "image" => Ok(Edit::image),
            _ => Err(String::from("Provided Not valid String")),
        }
    }
}
pub async fn fetch(body: String) -> Result<middleware::ResponseBody, reqwasm::Error> {
    let res: Result<middleware::ResponseBody, reqwasm::Error> = reqwasm::http::Request::post("/")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for Viewer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&middleware::RequestBody {
            kind: middleware::DatabaseRequest::Initial,
            name: String::new(),
        })
        .unwrap();
        ctx.link().send_future(async move {
            let data = fetch(body).await;
            Msg::ReceiveResponse(data)
        });
        Self {
            games: Vec::new(),
            locations: Vec::new(),
            mobs: Vec::new(),
            loot: Vec::new(),
            content: None,
            edit_type: Edit::game,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameChange(game) => {
                self.edit_type = Edit::game;
                let body = serde_json::to_string(&middleware::RequestBody {
                    kind: middleware::DatabaseRequest::LocationsByGame,
                    name: game.clone(),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });

                false
            }
            Msg::LocationChange(location) => {
                if location.contains("Undefined") {
                    self.content = Some((
                        include_bytes!("D:\\Projects\\Rust\\course_work\\frontend\\unknown.png")
                            .to_vec(),
                        String::from("Undefined"),
                        String::from("Just a placeholder"),
                    ));
                    self.edit_type = Edit::location;
                    let body = serde_json::to_string(&middleware::RequestBody {
                        kind: middleware::DatabaseRequest::ListsByLocation,
                        name: String::new(),
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                } else {
                    for tuple in &self.locations {
                        if tuple.0 == location {
                            self.content =
                                Some((tuple.1.clone(), tuple.0.clone(), tuple.2.clone()));
                            self.edit_type = Edit::location;
                            break;
                        }
                    }
                    let body = serde_json::to_string(&middleware::RequestBody {
                        kind: middleware::DatabaseRequest::ListsByLocation,
                        name: location.clone(),
                    })
                    .unwrap();
                    ctx.link().send_future(async move {
                        let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                    });
                }
                false
            }
            Msg::MobChange(name) => {
                for tuple in &self.mobs {
                    if tuple.0 == name {
                        self.content = Some((tuple.1.clone(), tuple.0.clone(), tuple.2.clone()));
                        self.edit_type = Edit::mob;
                        break;
                    }
                }
                true
            }
            Msg::LootChange(name) => {
                for tuple in &self.loot {
                    if tuple.0 == name {
                        self.content = Some((tuple.1.clone(), tuple.0.clone(), tuple.2.clone()));
                        self.edit_type = Edit::loot;
                        break;
                    }
                }
                true
            }
            Msg::ReceiveResponse(response) => match response {
                Ok(body) => match body.kind {
                    middleware::DatabaseRequest::Initial => {
                        if let Some(data) = &body.games {
                            self.games = data.clone();
                        } else {
                            self.games = Vec::new();
                        }
                        ctx.link()
                            .send_message(Msg::GameChange(body.clone().games.unwrap()[0].clone()));
                        true
                    }
                    middleware::DatabaseRequest::LocationsByGame => {
                        if let Some(data) = body.locations {
                            self.locations = data.clone();
                        } else {
                            self.locations = Vec::new();
                            self.mobs = Vec::new();
                            self.loot = Vec::new();
                        }
                        true
                    }
                    middleware::DatabaseRequest::Success => todo!(),
                    middleware::DatabaseRequest::Error => false,

                    middleware::DatabaseRequest::ListsByLocation => {
                        if let Some(data) = body.mobs {
                            self.mobs = data.clone();
                        } else {
                            self.mobs = Vec::new();
                        }
                        if let Some(data) = body.loot {
                            self.loot = data.clone();
                        } else {
                            self.loot = Vec::new();
                        }
                        true
                    }
                },
                Err(error) => {
                    self.games = vec![error.to_string()];
                    true
                }
            },
            Msg::EditIcon => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::AppRoute::EditContent {
                    typ: self.edit_type.clone(),
                    name_t: self.content.clone().unwrap().1.clone(),
                    part: Edit::image,
                });
                return false;
            }
            Msg::EditName => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::AppRoute::EditContent {
                    typ: self.edit_type.clone(),
                    name_t: self.content.clone().unwrap().1.clone(),
                    part: Edit::name,
                });
                return false;
            }
            Msg::EditDescription => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::AppRoute::EditContent {
                    typ: self.edit_type.clone(),
                    name_t: self.content.clone().unwrap().1.clone(),
                    part: Edit::description,
                });
                return false;
            }
            Msg::AddNew => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::AppRoute::AddNew);
                return false;
            }
            Msg::Delete => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::AppRoute::Delete);
                return false;
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let games = self
            .games
            .iter()
            .map(|game| {
                html! {
                     <input type="button" value={game.clone()} class="games_button"  onclick={ctx.link().callback(|e: MouseEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::GameChange(input.value())
                    })}/>
                }
            })
            .collect::<Html>();
        let locations = self
            .locations
            .iter()
            .map(|location| {
                html! {
                    <li class="li_element">
                        <button class="locatons_button" value={location.0.clone()}  onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlButtonElement = e.target_unchecked_into();
                            Msg::LocationChange(input.value())
                        })}>{location.0.clone()}</button>
                    </li>
                }
            })
            .collect::<Html>();
        let mobs = self
            .mobs
            .iter()
            .map(|mob| {
                html! {
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::MobChange(input.inner_text())
                        })}>{mob.0.clone()}</a>
                }
            })
            .collect::<Html>();
        let loot = self
            .loot
            .iter()
            .map(|loot| {
                html! {

                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::LootChange(input.inner_text())
                        })}>{loot.0.clone()}</a>
                }
            })
            .collect::<Html>();

        let mut content = html! {};
        if let Some(desc) = &self.content {
            content = html! {
                <div class={css!("display: flex;flex-direction: row;")}>
                    <div class={css!("margin-right: 5%; width: 45%;")}>
                        <h1 class="desc_name">{desc.1.clone()}
                        <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::EditName
                        })}>{".Edit"}</a></h1>
                        //добавить изменить
                        <p class="desc_content">{desc.2.clone()}
                        <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::EditDescription
                        })}>{".Edit"}</a></p>
                        //добавить изменить
                    </div>
                    <div class={css!("width:45%;")}>
                        <img class={css!("width: 100%; height: 100%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(desc.0.clone()))}/>
                        <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::EditIcon
                        })}>{".Edit"}</a>
                        //добавить изменить
                    </div>
                </div>
            };
        }

        html! {
            <div class={css!("margin-top: 10%;")}>
                <header class={css!("margin-bottom: 10px;")}>
                    <center>
                        {games}
                    </center>
                </header>

                <div id="main_div" class="main_div">
                            <div class="locations_div">
                                <ul class="locations_list">
                                    {locations}//добавить добавить
                                    <li class="li_element">
                                    <button class="locatons_button" value={"Undefined"}  onclick={ctx.link().callback(|e: MouseEvent| {
                                        let input: HtmlButtonElement = e.target_unchecked_into();
                                        Msg::LocationChange(input.value())
                                    })}>{"Undefined"}</button>
                                </li>
                                </ul>
                            </div>
                            <div class={css!("width:100%;")}>
                                    <div id="dropdown" class="dropdown">
                                        <button class="adapt_list">{ "Mobs List "}</button>
                                        <div id="content" class="content">
                                            {mobs}//добавить добавить
                                        </div>
                                    </div>
                                    <div class="dropdown">
                                        <button class="adapt_list">{ "Loot List "}</button>
                                        <div class="content">
                                            {loot}
                                        </div>
                                    </div>
                                <br/>
                                <div class={css!("margin-left: 5%; margin-right: 5%;")}>
                                    {content}
                                </div>
                            </div>
                </div>
                <footer>
                    <center>
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::AddNew
                        })}>{"Add new"}</a><a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::Delete
                        })}>{"Delete"}</a>
                    </center>
                </footer>
                <stylist::yew::Global css=r#"
                .edit{
                    font-size: 14px;
                    color: blue;
                }
                .locations_button{
                    background-color: none;
                    border: none;
                    color: black;
                    border-radius: 6px;
                    padding: 7px 10px;
                    text-align: center;
                    text-decoration: none;
                    font-size: 14px;
                    
                }
                .li_element{
                    margin-right: 40px; 
                    margin-bottom: 10px;
                }
                .locations_div{
                    text-align: center;
                    background-color: blue;
                }
                .locations_list{
                    margin-top: 20px;
                    text-align: center;
                    list-style-type: none;
                    height: 100%;
                    width: 150px;
                }
                .main_div{
                    display: flex;
                    flex-direction: row;
                    height:740px;
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
            </div>
        }
    }
}
