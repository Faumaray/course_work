use std::collections::HashMap;

use gloo_file::{callbacks::FileReader, File};
use web_sys::console;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

#[derive(PartialEq, Properties)]
pub struct EditProps {
    pub edit_type: crate::pages::viewer::Edit,
    pub name: String,
    pub part: crate::pages::viewer::Edit,
}
pub struct EditContent {
    name: String,
    original: String,
    description: Option<String>,
    image: Vec<u8>,
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
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
    ReceiveResponse(Result<middleware::EditResponseBody, reqwasm::Error>),
}
pub async fn fetch(body: String) -> Result<middleware::EditResponseBody, reqwasm::Error> {
    let res: Result<middleware::EditResponseBody, reqwasm::Error> =
        reqwasm::http::Request::post("/edit")
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .json()
            .await;
    res
}
impl Component for EditContent {
    type Message = Msg;
    type Properties = EditProps;

    fn create(ctx: &Context<Self>) -> Self {
        let edit_type = match ctx.props().edit_type {
            super::viewer::Edit::location => middleware::EditType::Location,
            super::viewer::Edit::mob => middleware::EditType::Mob,
            super::viewer::Edit::loot => middleware::EditType::Loot,
            super::viewer::Edit::game => middleware::EditType::Game,
            _ => middleware::EditType::Game,
        };
        let body = serde_json::to_string(&middleware::EditRequestBody {
            kind: middleware::EditRequest::Initial,
            description: None,
            image: None,
            name: Some(ctx.props().name.clone().replace("%20", " ")),
            original: None,
            edit_type: edit_type.clone(),
        })
        .unwrap();
        ctx.link().send_future(async move {
            let data = fetch(body).await;
            Msg::ReceiveResponse(data)
        });
        Self {
            name: ctx.props().name.clone().replace("%20", " "),
            original: ctx.props().name.clone().replace("%20", " "),
            description: None,
            image: Vec::new(),
            readers: HashMap::default(),
            files: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let edit_type = match ctx.props().edit_type {
            super::viewer::Edit::location => middleware::EditType::Location,
            super::viewer::Edit::mob => middleware::EditType::Mob,
            super::viewer::Edit::loot => middleware::EditType::Loot,
            super::viewer::Edit::game => middleware::EditType::Game,
            _ => middleware::EditType::Game,
        };
        match msg {
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
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
                let body = serde_json::to_string(&middleware::EditRequestBody {
                    kind: middleware::EditRequest::ChangeName,
                    edit_type: edit_type,
                    description: None,
                    image: None,
                    name: Some(self.name.clone()),
                    original: Some(self.original.clone()),
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::EditDescriptionConfirm => {
                let body = serde_json::to_string(&middleware::EditRequestBody {
                    kind: middleware::EditRequest::ChangeDescription,
                    edit_type: edit_type,
                    description: self.description.clone(),
                    image: None,
                    name: Some(self.name.clone()),
                    original: None,
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::EditImageConfirm => {
                let body = serde_json::to_string(&middleware::EditRequestBody {
                    kind: middleware::EditRequest::ChangeDescription,
                    edit_type: edit_type,
                    description: None,
                    image: Some(self.image.clone()),
                    name: Some(self.name.clone()),
                    original: None,
                })
                .unwrap();
                ctx.link().send_future(async move {
                    let data = fetch(body).await;
                    Msg::ReceiveResponse(data)
                });
                true
            }
            Msg::ReceiveResponse(res) => match res {
                Ok(body) => match body.kind {
                    middleware::EditRequest::Initial => {
                        self.name = body.name.clone().unwrap_or_default();
                        self.description = body.description.clone();
                        self.image = body.image.clone().unwrap_or_default();
                        return true;
                    }
                    middleware::EditRequest::Success => {
                        let history = ctx.link().history().unwrap();
                        history.back();
                        false
                    }
                    _ => return false,
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
            crate::pages::viewer::Edit::name => {
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
            crate::pages::viewer::Edit::description => {
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
            crate::pages::viewer::Edit::image => {
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

        html! {<>
            <div id="main_div" class="main_div">
                {inner}
            </div>
            <stylist::yew::Global css=r#"
            .inner{
                margin-top: 5%;
                margin-left: 15%;
                margin-right: 15%;
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
