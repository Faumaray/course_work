use crate::switch::fetch_get as get;
use crate::switch::fetch_post as post;
use middleware::GetterDeleteBlockListRequestTypes as State;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct Delete {
    state: State,
    games: Vec<(i32, String)>,
    locations: Vec<(i32, i32, String)>,
    mobs: Vec<(i32, Option<i32>, String)>,
    loots: Vec<(i32, Option<i32>, Option<i32>, String)>,
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
                    middleware::Response::Error(_, _) => todo!(),
                    middleware::Response::Success(_) => todo!(),
                    middleware::Response::PageShow(_) => todo!(),
                    middleware::Response::Getter(_) => todo!(),
                    middleware::Response::GetterDeleteBlockList(_) => todo!(),
                },
                Err(er) => todo!(),
            },
            Msg::DeleteGame(_) => todo!(),
            Msg::DeleteLocation(_) => todo!(),
            Msg::DeleteMob(_) => todo!(),
            Msg::DeleteLoot(_) => todo!(),
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
                            <td>{game.1.clone()}</td>
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
                            <td>{game.1.clone().unwrap_or_default()}</td>
                            <td>{game.2.clone()}</td>
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
        }
    }
}
