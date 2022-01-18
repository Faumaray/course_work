use crate::support::fetch_post as post;
use middleware::GetterDeleteBlockListRequestTypes as State;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct Delete {
    state: State,
    games: Vec<(i32, String)>,
    locations: Vec<(i32, Option<i32>, String)>,
    mobs: Vec<(i32, Option<i32>, Option<i32>, String)>,
    loots: Vec<(i32, Option<i32>, Option<i32>, Option<i32>, String)>,
}
pub enum Msg {
    DeleteGame(String),
    DeleteLocation(String),
    DeleteMob(String),
    DeleteLoot(String),
    ChangeState(State),
    ReceiveResponse(Result<middleware::Response, reqwasm::Error>),
}

impl Component for Delete {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&middleware::Request::GetterDeleteBlockList(
            middleware::GetterDeleteBlockListRequestTypes::Game,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/admin/delete").await;
            Msg::ReceiveResponse(data)
        });
        let body = serde_json::to_string(&middleware::Request::GetterDeleteBlockList(
            middleware::GetterDeleteBlockListRequestTypes::Location,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/admin/delete").await;
            Msg::ReceiveResponse(data)
        });
        let body = serde_json::to_string(&middleware::Request::GetterDeleteBlockList(
            middleware::GetterDeleteBlockListRequestTypes::Mob,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/admin/delete").await;
            Msg::ReceiveResponse(data)
        });
        let body = serde_json::to_string(&middleware::Request::GetterDeleteBlockList(
            middleware::GetterDeleteBlockListRequestTypes::Loot,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/admin/delete").await;
            Msg::ReceiveResponse(data)
        });

        Self {
            state: State::Game,
            games: Vec::new(),
            locations: Vec::new(),
            mobs: Vec::new(),
            loots: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveResponse(re) => match re {
                Ok(body) => match body {
                    middleware::Response::Error(_, _) => return false,
                    middleware::Response::Success(_, _) => return true,
                    middleware::Response::PageShow(_) => todo!(),
                    middleware::Response::Getter(_, _) => todo!(),
                    middleware::Response::GetterDeleteBlockList(list_type) => match list_type {
                        middleware::GetterDeleteBlockListResponseTypes::Mob(vc) => {
                            self.mobs = vc.clone();
                        }
                        middleware::GetterDeleteBlockListResponseTypes::Game(vc) => {
                            self.games = vc.clone()
                        }
                        middleware::GetterDeleteBlockListResponseTypes::Location(vc) => {
                            self.locations = vc.clone()
                        }
                        middleware::GetterDeleteBlockListResponseTypes::Loot(vc) => {
                            self.loots = vc.clone()
                        }
                    },
                },
                Err(er) => todo!(),
            },
            Msg::DeleteGame(id) => {
                let body = serde_json::to_string(&middleware::Request::PageDelete(
                    middleware::DeleteContentRequestBodyTypes::Game {
                        id: id.parse::<i32>().unwrap(),
                    },
                ))
                .unwrap();
                if let Some(pos) = self
                    .games
                    .clone()
                    .iter()
                    .position(|x| x.0 == id.parse::<i32>().unwrap())
                {
                    self.games.remove(pos);
                }
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/delete").await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteLocation(id) => {
                let body = serde_json::to_string(&middleware::Request::PageDelete(
                    middleware::DeleteContentRequestBodyTypes::Location {
                        id: id.parse::<i32>().unwrap(),
                    },
                ))
                .unwrap();
                if let Some(pos) = self
                    .locations
                    .clone()
                    .iter()
                    .position(|x| x.0 == id.parse::<i32>().unwrap())
                {
                    self.locations.remove(pos);
                }
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/delete").await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteMob(id) => {
                let body = serde_json::to_string(&middleware::Request::PageDelete(
                    middleware::DeleteContentRequestBodyTypes::Mob {
                        id: id.parse::<i32>().unwrap(),
                    },
                ))
                .unwrap();
                if let Some(pos) = self
                    .mobs
                    .clone()
                    .iter()
                    .position(|x| x.0 == id.parse::<i32>().unwrap())
                {
                    self.mobs.remove(pos);
                }
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/delete").await;
                    Msg::ReceiveResponse(data)
                });
            }
            Msg::DeleteLoot(id) => {
                let body = serde_json::to_string(&middleware::Request::PageDelete(
                    middleware::DeleteContentRequestBodyTypes::Loot {
                        id: id.parse::<i32>().unwrap(),
                    },
                ))
                .unwrap();
                if let Some(pos) = self
                    .loots
                    .clone()
                    .iter()
                    .position(|x| x.0 == id.parse::<i32>().unwrap())
                {
                    self.loots.remove(pos);
                }
                ctx.link().send_future(async move {
                    let data = post(body, "/admin/delete").await;
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
                                    Msg::DeleteGame(row.cells().item(0).unwrap().text_content().unwrap_or_default())
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
                            <td>{format!("{:?}",game.1.clone())}</td>
                            <td>{game.2.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                Msg::DeleteLocation(row.cells().item(0).unwrap().text_content().unwrap_or_default())
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
                            <td>{format!("{:?}",game.1.clone())}</td>
                            <td>{format!("{:?}",game.2.clone())}</td>
                            <td>{game.3.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                Msg::DeleteMob(row.cells().item(0).unwrap().text_content().unwrap_or_default())
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
                            <td>{format!("{:?}",game.1.clone())}</td>
                            <td>{format!("{:?}",game.2.clone())}</td>
                            <td>{format!("{:?}",game.3.clone())}</td>
                            <td>{game.4.clone()}</td>
                            <td> <a  onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: web_sys::HtmlLinkElement = e.target_unchecked_into();
                                let element: &web_sys::Element = input.as_ref();
                                let closest: web_sys::Element = element.closest("tr").unwrap().unwrap();
                                let row: web_sys::HtmlTableRowElement = closest.dyn_into().unwrap();
                                Msg::DeleteLoot(row.cells().item(0).unwrap().text_content().unwrap_or_default())
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
                            <th scope="col">{"Locationid"}</th>
                            <th scope="col">{"Mobid"}</th>
                            <th scope="col">{"name"}</th>
                            <th scope="col">{""}</th>
                        </tr>
                        {games}
                    </table>
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
                            {inside}
                        </center>
                    </div>
                    <div class="Inner_Content">

                    </div>
                </div>
        }
    }
}
