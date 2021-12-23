use wasm_bindgen::JsCast;
use yew::prelude::*;

enum State {
    Game,
    Location,
    Mob,
    Loot,
    Unknown,
}
pub struct Delete {
    state: State,
    games: Vec<(i32, String)>,
    locations: Vec<(i32, i32, String)>,
    mobs: Vec<(i32, Option<i32>, String)>,
    loots: Vec<(i32, Option<i32>, Option<i32>, String)>,
}
pub async fn fetch(body: String) -> Result<middleware::DeleteBody, reqwasm::Error> {
    let res: Result<middleware::DeleteBody, reqwasm::Error> =
        reqwasm::http::Request::post("/delete")
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .json()
            .await;
    res
}
pub enum Msg {
    ReceiveResponse(Result<middleware::DeleteBody, reqwasm::Error>),
    DeleteGame(Vec<String>),
    DeleteLocation(Vec<String>),
    DeleteMob(Vec<String>),
    DeleteLoot(Vec<String>),
    ChangeState(State),
}

impl Component for Delete {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&middleware::DeleteBody {
            kind: middleware::DeleteRequest::Initial,
            games: None,
            locations: None,
            mobs: None,
            loots: None,
            id: None,
            locationid: None,
            mobid: None,
            name: None,
        })
        .unwrap();
        ctx.link().send_future(async move {
            let data = fetch(body).await;
            Msg::ReceiveResponse(data)
        });
        Self {
            state: State::Unknown,
            games: Vec::new(),
            locations: Vec::new(),
            mobs: Vec::new(),
            loots: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveResponse(re) => match re {
                Ok(body) => match body.kind {
                    middleware::DeleteRequest::Initial => {
                        self.games = body.games.unwrap().clone();
                        self.locations = body.locations.unwrap().clone();
                        self.mobs = body.mobs.unwrap().clone();
                        self.loots = body.loots.unwrap().clone();
                        return true;
                    }
                    middleware::DeleteRequest::Game => todo!(),
                    middleware::DeleteRequest::Location => todo!(),
                    middleware::DeleteRequest::Mob => todo!(),
                    middleware::DeleteRequest::Loot => todo!(),
                    middleware::DeleteRequest::Success => return true,
                    middleware::DeleteRequest::Error => return false,
                },
                Err(er) => todo!(),
            },
            Msg::DeleteGame(content) => {
                let mut new = Vec::new();
                for v in self.games.clone() {
                    if v.0 == content.clone()[0].parse::<i32>().unwrap_or_default()
                        && content.clone()[1].clone() == v.1
                    {
                    } else {
                        new.push(v);
                    }
                }
                self.games = new.clone();
                let body = serde_json::to_string(&middleware::DeleteBody {
                    kind: middleware::DeleteRequest::Game,
                    games: None,
                    locations: None,
                    mobs: None,
                    loots: None,
                    id: Some(content.clone()[0].parse().unwrap_or_default()),
                    locationid: None,
                    mobid: None,
                    name: Some(content.clone()[1].clone()),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteLocation(content) => {
                let mut new = Vec::new();
                for v in self.locations.clone() {
                    if v.0 == content.clone()[0].parse::<i32>().unwrap_or_default()
                        && content.clone()[2].clone() == v.2
                    {
                    } else {
                        new.push(v);
                    }
                }
                self.locations = new.clone();
                let body = serde_json::to_string(&middleware::DeleteBody {
                    kind: middleware::DeleteRequest::Location,
                    games: None,
                    locations: None,
                    mobs: None,
                    loots: None,
                    id: Some(content.clone()[0].parse::<i32>().unwrap_or_default()),
                    locationid: None,
                    mobid: None,
                    name: Some(content.clone()[2].clone()),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteMob(content) => {
                let mut new = Vec::new();
                for v in self.mobs.clone() {
                    if v.0 == content.clone()[0].parse::<i32>().unwrap_or_default()
                        && content.clone()[2].clone() == v.2
                    {
                    } else {
                        new.push(v);
                    }
                }
                self.mobs = new.clone();
                let body = serde_json::to_string(&middleware::DeleteBody {
                    kind: middleware::DeleteRequest::Mob,
                    games: None,
                    locations: None,
                    mobs: None,
                    loots: None,
                    id: Some(content.clone()[0].parse().unwrap_or_default()),
                    locationid: None,
                    mobid: None,
                    name: Some(content.clone()[2].clone()),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteLoot(content) => {
                let mut new = Vec::new();
                for v in self.loots.clone() {
                    if v.0 == content.clone()[0].parse::<i32>().unwrap_or_default()
                        && content.clone()[3].clone() == v.3
                    {
                    } else {
                        new.push(v);
                    }
                }
                self.loots = new.clone();
                let body = serde_json::to_string(&middleware::DeleteBody {
                    kind: middleware::DeleteRequest::Loot,
                    games: None,
                    locations: None,
                    mobs: None,
                    loots: None,
                    id: Some(content.clone()[0].parse().unwrap_or_default()),
                    locationid: None,
                    mobid: None,
                    name: Some(content.clone()[3].clone()),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::ChangeState(state) => {
                self.state = state;
            }
        }
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inside = match self.state {
            State::Game => {
                let games = self
                    .games
                    .iter()
                    .map(|game| {
                        html! {
                            <tr>
                                <td>{game.0}</td>
                                <td>{game.1.clone()}</td>
                                <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                    let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                    let element: &web_sys::Element = input.as_ref();
                                    let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                    let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                    let mut content = Vec::new();
                                    for i in 0..row.cells().length()
                                    {
                                        content.push(row.cells().item(i).unwrap().text_content().unwrap_or_default());
                                    }
                                    Msg::DeleteGame(content.clone())
                                })}>{"Delete"}</a></td>
                            </tr>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <table>
                        <tr>
                            <th scope="col">{"id"}</th>
                            <th scope="col">{"name"}</th>
                            <th scope="col">{""}</th>
                        </tr>
                        {games}
                    </table>
                }
            }
            State::Location => {
                let games = self
                .locations
                .iter()
                .map(|game| {
                    html! {
                        <tr>
                            <td>{game.0}</td>
                            <td>{game.1.clone()}</td>
                            <td>{game.2.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                let mut content = Vec::new();
                                for i in 0..row.cells().length()
                                {
                                    content.push(row.cells().item(i).unwrap().text_content().unwrap_or_default());
                                }
                                Msg::DeleteLocation(content.clone())
                            })}>{"Delete"}</a></td>
                        </tr>
                    }
                })
                .collect::<Html>();
                html! {
                    <table>
                        <tr>
                            <th scope="col">{"id"}</th>
                            <th scope="col">{"Gameid"}</th>
                            <th scope="col">{"name"}</th>
                            <th scope="col">{""}</th>
                        </tr>
                        {games}
                    </table>
                }
            }
            State::Mob => {
                let games = self
                .mobs
                .iter()
                .map(|game| {
                    html! {
                        <tr>
                            <td>{game.0}</td>
                            <td>{game.1.clone().unwrap_or_default()}</td>
                            <td>{game.2.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                let mut content = Vec::new();
                                for i in 0..row.cells().length()
                                {
                                    content.push(row.cells().item(i).unwrap().text_content().unwrap_or_default());
                                }
                                Msg::DeleteMob(content.clone())
                            })}>{"Delete"}</a></td>
                        </tr>
                    }
                })
                .collect::<Html>();
                html! {
                    <table>
                        <tr>
                            <th scope="col">{"id"}</th>
                            <th scope="col">{"Locationid"}</th>
                            <th scope="col">{"name"}</th>
                            <th scope="col">{""}</th>
                        </tr>
                        {games}
                    </table>
                }
            }
            State::Loot => {
                let games = self
                .loots
                .iter()
                .map(|game| {
                    html! {
                        <tr>
                            <td>{game.0}</td>
                            <td>{game.1.clone().unwrap_or_default()}</td>
                            <td>{game.2.clone().unwrap_or_default()}</td>
                            <td>{game.3.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                let mut content = Vec::new();
                                for i in 0..row.cells().length()
                                {
                                    content.push(row.cells().item(i).unwrap().text_content().unwrap_or_default());
                                }
                                Msg::DeleteLoot(content.clone())
                            })}>{"Delete"}</a></td>
                        </tr>
                    }
                })
                .collect::<Html>();
                html! {
                    <table>
                        <tr>
                            <th scope="col">{"id"}</th>
                            <th scope="col">{"Locationid"}</th>
                            <th scope="col">{"Mobid"}</th>
                            <th scope="col">{"name"}</th>
                            <th scope="col">{""}</th>
                        </tr>
                        {games}
                    </table>
                }
            }
            State::Unknown => html! {},
        };
        html! {<>
            <div id="main_div" class="main_div">
            <center class={stylist::css!("width: 100%; height: 100%;margin-top: 5%;")}>
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
            table, th, td {
                border: 1px solid black;
            }
            table {
                width: 100%;
            }
            .main_div{
                height:740px;
                line-height: 1.5;
                margin-top: 5%;
                margin-left: 15%;
                margin-right: 15%;
                background: white;
                border-radius: 12px;
            }
            
            "#/>
        </>}
    }
}
