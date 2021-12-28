use crate::switch::fetch_post as post;
use gloo_file::{callbacks::FileReader, File};
use middleware::GetterDeleteBlockListRequestTypes as State;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

pub struct AddNew {
    preview: Vec<u8>,
    readers: HashMap<String, FileReader>,
    state: State,
}

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
    ReceiveResponse(Result<middleware::Response, reqwasm::Error>),
    ChangeState(State),
}
impl Component for AddNew {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            preview: Vec::new(),
            readers: HashMap::default(),
            state: State::Game,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
                    middleware::Response::Error(_, _) => todo!(),
                    middleware::Response::Success(_) => todo!(),
                    middleware::Response::PageShow(_) => todo!(),
                    middleware::Response::Getter(_) => todo!(),
                    middleware::Response::GetterDeleteBlockList(_) => todo!(),
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
                html! {}
            }
            State::Location => {
                html! {}
            }
            State::Mob => {
                html! {}
            }
            State::Loot => {
                html! {}
            }
        };
        html! {
                <div id="main_div" class="main_div">
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
        }
    }
}
