use crate::support::fetch_post as post;
use gloo_file::{callbacks::FileReader, File};
use middleware::GetterDeleteBlockListRequestTypes as State;
use std::collections::HashMap;
use web_sys::console;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

#[derive(PartialEq, Properties)]
pub struct EditProps {
    pub edit_type: crate::support::EditTypes,
    pub name: String,
    pub part: crate::support::EditTypes,
}
pub struct EditContent {
    name: String,
    original: String,
    description: Option<String>,
    image: Vec<u8>,
    readers: HashMap<String, FileReader>,
}

pub enum Msg {
    Initial,
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
    Change(String),
    DescChange(String),
    EditNameConfirm,
    EditDescriptionConfirm,
    EditImageConfirm,
    ReceiveResponse(Result<middleware::Response, reqwasm::Error>),
}
impl Component for EditContent {
    type Message = Msg;
    type Properties = EditProps;

    fn create(ctx: &Context<Self>) -> Self {
        let edit_type = match ctx.props().edit_type {
            crate::support::EditTypes::Location => State::Location,
            crate::support::EditTypes::Mob => State::Mob,
            crate::support::EditTypes::Loot => State::Loot,
            crate::support::EditTypes::Game => State::Game,
            _ => State::Game,
        };
        let (data, _) = ctx
            .link()
            .context::<crate::support::InfoProp>(Callback::noop())
            .unwrap();

        Self {
            name: ctx.props().name.clone().replace("%20", " "),
            original: ctx.props().name.clone().replace("%20", " "),
            description: data.description,
            image: data.preview.unwrap_or_default(),
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let edit_type = match ctx.props().edit_type {
            crate::support::EditTypes::Location => State::Location,
            crate::support::EditTypes::Mob => State::Mob,
            crate::support::EditTypes::Loot => State::Loot,
            crate::support::EditTypes::Game => State::Game,
            _ => State::Game,
        };
        match msg {
            Msg::LoadedBytes(file_name, data) => {
                self.image = data.clone();
                self.readers.remove(&file_name);
                true
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
                true
            }
            Msg::DescChange(new_desc) => {
                self.description = Some(new_desc.clone());
                true
            }
            Msg::Change(new_name) => {
                self.name = new_name.clone();
                true
            }
            Msg::EditNameConfirm => {
                let body = match ctx.props().edit_type {
                    crate::support::EditTypes::Location => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Location(
                                middleware::EditContentPartTypes::Name {
                                    original: self.original.clone(),
                                    new: self.name.clone(),
                                },
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Mob => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Mob(
                                middleware::EditContentPartTypes::Name {
                                    original: self.original.clone(),
                                    new: self.name.clone(),
                                },
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Loot => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Loot(
                                middleware::EditContentPartTypes::Name {
                                    original: self.original.clone(),
                                    new: self.name.clone(),
                                },
                            ),
                        ))
                        .unwrap()
                    }
                    _ => serde_json::to_string(&middleware::Request::PageEdit(
                        middleware::EditContentRequestBodyTypes::Game(
                            middleware::EditContentPartTypes::Name {
                                original: self.original.clone(),
                                new: self.name.clone(),
                            },
                        ),
                    ))
                    .unwrap(),
                };
                let path = format!(
                    "/admin/edit/{}/{}/{}",
                    ctx.props().edit_type.clone(),
                    ctx.props().part.clone(),
                    ctx.props().name.clone()
                );
                ctx.link().send_future(async move {
                    let data = post(body, path.clone().as_str()).await;
                    Msg::ReceiveResponse(data)
                });
                false
            }
            Msg::EditDescriptionConfirm => {
                let body = match ctx.props().edit_type {
                    crate::support::EditTypes::Location => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Location(
                                middleware::EditContentPartTypes::Description(
                                    self.description.clone().unwrap_or_default(),
                                ),
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Mob => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Mob(
                                middleware::EditContentPartTypes::Description(
                                    self.description.clone().unwrap_or_default(),
                                ),
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Loot => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Loot(
                                middleware::EditContentPartTypes::Description(
                                    self.description.clone().unwrap_or_default(),
                                ),
                            ),
                        ))
                        .unwrap()
                    }
                    _ => serde_json::to_string(&middleware::Request::PageEdit(
                        middleware::EditContentRequestBodyTypes::Game(
                            middleware::EditContentPartTypes::Description(
                                self.description.clone().unwrap_or_default(),
                            ),
                        ),
                    ))
                    .unwrap(),
                };
                let path = format!(
                    "/admin/edit/{}/{}/{}",
                    ctx.props().edit_type.clone(),
                    ctx.props().part.clone(),
                    ctx.props().name.clone()
                );
                ctx.link().send_future(async move {
                    let data = post(body, path.clone().as_str()).await;
                    Msg::ReceiveResponse(data)
                });
                false
            }
            Msg::EditImageConfirm => {
                let body = match ctx.props().edit_type {
                    crate::support::EditTypes::Location => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Location(
                                middleware::EditContentPartTypes::Preview(self.image.clone()),
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Mob => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Mob(
                                middleware::EditContentPartTypes::Preview(self.image.clone()),
                            ),
                        ))
                        .unwrap()
                    }
                    crate::support::EditTypes::Loot => {
                        serde_json::to_string(&middleware::Request::PageEdit(
                            middleware::EditContentRequestBodyTypes::Loot(
                                middleware::EditContentPartTypes::Preview(self.image.clone()),
                            ),
                        ))
                        .unwrap()
                    }
                    _ => serde_json::to_string(&middleware::Request::PageEdit(
                        middleware::EditContentRequestBodyTypes::Game(
                            middleware::EditContentPartTypes::Preview(self.image.clone()),
                        ),
                    ))
                    .unwrap(),
                };
                let path = format!(
                    "/admin/edit/{}/{}/{}",
                    ctx.props().edit_type.clone(),
                    ctx.props().part.clone(),
                    ctx.props().name.clone()
                );
                ctx.link().send_future(async move {
                    let data = post(body, path.clone().as_str()).await;
                    Msg::ReceiveResponse(data)
                });
                false
            }
            Msg::ReceiveResponse(res) => match res {
                Ok(body) => match body {
                    middleware::Response::Error(_, _) => todo!(),
                    middleware::Response::Success(_, _) => {
                        let history = ctx.link().history().unwrap();
                        history.back();
                        true
                    }
                    middleware::Response::PageShow(_) => todo!(),
                    middleware::Response::Getter(_, _) => todo!(),
                    middleware::Response::GetterDeleteBlockList(_) => todo!(),
                },
                Err(err_msg) => return false,
            },
            _ => return false,
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inner = match ctx.props().part {
            crate::support::EditTypes::Name => {
                html! {
                    <div class="inner">
                    <center class={stylist::css!("margin-top: 20px;")}>
                                    <label for="name">{"Edit Name:"} </label>
                                    <input value={self.name.clone()} type="text" name="name" id="name" oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::Change(input.value())
                                    })}/>
                                    <br/>
                                    <button onclick={ctx.link().callback(|_|{
                                        Msg::EditNameConfirm
                                    })}>{"Submit"}</button>
                        </center>
                    </div>
                }
            }
            crate::support::EditTypes::Description => {
                html! {
                    <div class="inner">
                    <center class={stylist::css!("margin-top: 5%;")}>
                            <textarea class={stylist::css!("width: 200px; height:200px;margin-top:20px;")} value={self.description.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                Msg::DescChange(input.value())
                            })} />
                            <br/>
                            <button onclick={ctx.link().callback(|_|{
                                Msg::EditDescriptionConfirm
                            })}>{"Submit"}</button>
                        </center>
                    </div>
                }
            }
            crate::support::EditTypes::Image => {
                html! {
                    <div class={stylist::css!("width:90%, height: 90%;margin-left: 15%, margin-top:5%;")}>
                        <center class={stylist::css!("margin-top: 20px;")}>
                            <img class={stylist::css!("width: 55%; height: 55%;bject-fit: cover;")} src={format!("data:image/png;base64,{}", base64::encode(self.image.clone()))}/>
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
                            <button onclick={ctx.link().callback(|_|{
                                Msg::EditImageConfirm
                            })}>{"Submit"}</button>
                        </center>
                    </div>
                }
            }
            _ => {
                html! {}
            }
        };

        html! {
                <div class="Content">
                    <div class="Inner_Content">
                    {inner}
                    </div>
                </div>
        }
    }
}
