use crate::support::fetch_post as post;
use crate::switch::SecondRoute;
use crate::{
    pages::{
        admin::{add_new::AddNew, delete::Delete, edit_content::EditContent},
        index::Index,
        index_game::IndexGame,
        index_related::IndexRelated,
        page_not_found::PageNotFound,
    },
    support,
};
use serde::Serialize;
use web_sys::HtmlLinkElement;
use yew::prelude::*;
use yew_router::prelude::*;

fn second_switch(routes: &SecondRoute) -> Html {
    match routes {
        SecondRoute::Index => {
            html! { <Index/> }
        }
        SecondRoute::AddNew => {
            html! { <AddNew/> }
        }
        SecondRoute::Delete => {
            html! { <Delete/> }
        }
        SecondRoute::IndexGame { game } => {
            html! { <IndexGame game_name={ game.clone()} /> }
        }
        SecondRoute::IndexRelated { game, related_name } => {
            html! { <IndexRelated game_name={ game.clone() } related_name={ related_name.clone() }/>}
        }
        SecondRoute::EditContent { typ, name_t, part } => {
            html! { <EditContent part={part} edit_type={typ} name={name_t.clone()}/> }
        }
    }
}
pub struct Body {
    username: String,
    password: String,
    confirm: String,
    email: String,
    error: bool,
    user: crate::support::UserType,
    game: String,
    game_list: Vec<String>,
    location_list: Vec<String>,
    mob_list: Vec<String>,
    loot_list: Vec<String>,
    context: crate::support::InfoProp,
}
#[derive(Serialize)]
pub enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    EmailChanged(String),
    RegistrationSubmit,
    LoginSubmit,
    LogOut,
    Open,
    UserChange(crate::support::UserType),
    GameClick(String),
    AddNew,
    Delete,
    RelatedClick(String, crate::support::EditTypes),
    Response(middleware::Response),
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&middleware::Request::Getter(
            middleware::GetterRequestBodyTypes::GameList,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/").await.unwrap();
            Msg::Response(data)
        });
        let body = serde_json::to_string(&middleware::Request::Getter(
            middleware::GetterRequestBodyTypes::CurrentUser,
        ))
        .unwrap();
        ctx.link().send_future(async move {
            let data = post(body, "/").await.unwrap();
            Msg::Response(data)
        });
        Self {
            user: crate::support::UserType::Unknown,
            game: String::new(),
            context: crate::support::InfoProp {
                user_type: crate::support::UserType::Unknown,
                content_type: None,
                description: None,
                preview: None,
            },
            username: String::new(),
            password: String::new(),
            email: String::new(),
            confirm: String::new(),
            error: false,
            game_list: Vec::new(),
            location_list: Vec::new(),
            mob_list: Vec::new(),
            loot_list: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddNew => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::AddNew);
                true
            }
            Msg::Delete => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::Delete);
                true
            }
            Msg::Open => {
                self.username = String::new();
                true
            }
            Msg::UsernameChanged(new) => {
                self.username = new;
                true
            }
            Msg::EmailChanged(new) => {
                self.email = new;
                true
            }
            Msg::PasswordChanged(new) => {
                self.password = new;
                true
            }
            Msg::PasswordConfirmChanged(new) => {
                self.confirm = new;
                true
            }
            Msg::RegistrationSubmit => {
                if self.password != self.confirm {
                    self.error = true;
                } else {
                    self.error = false;
                    let body = serde_json::to_string(&middleware::Request::Registration {
                        username: self.username.clone(),
                        email: self.email.clone(),
                        password: self.password.clone(),
                    })
                    .unwrap();

                    ctx.link().send_future(async move {
                        let data = post(body, "/register").await.unwrap();
                        Msg::Response(data)
                    });
                }
                true
            }
            Msg::LoginSubmit => {
                let body = serde_json::to_string(&middleware::Request::Login {
                    email: self.username.clone(),
                    password: self.password.clone(),
                })
                .unwrap();

                ctx.link().send_future(async move {
                    let data = post(body, "/login").await.unwrap();
                    Msg::Response(data)
                });
                true
            }
            Msg::LogOut => {
                let body = serde_json::to_string(&middleware::Request::LogOut {
                    username: self.username.clone(),
                })
                .unwrap();

                ctx.link().send_future(async move {
                    let data = post(body, "/").await.unwrap();
                    Msg::UserChange(crate::support::UserType::Unknown)
                });
                true
            }
            Msg::UserChange(user_type) => {
                self.user = user_type.clone();
                true
            }
            Msg::GameClick(name) => {
                self.game = name.clone();
                let body = serde_json::to_string(&middleware::Request::PageShow(
                    middleware::InfoRequestBodyTypes::Game { name: name.clone() },
                ))
                .unwrap();

                ctx.link().send_future(async move {
                    let data = post(body, format!("/{}", name.clone()).as_str())
                        .await
                        .unwrap();
                    Msg::Response(data)
                });
                true
            }
            Msg::RelatedClick(name, related_type) => match related_type {
                crate::support::EditTypes::Mob => {
                    let body = serde_json::to_string(&middleware::Request::PageShow(
                        middleware::InfoRequestBodyTypes::Mob {
                            name: name.clone(),
                            game: Some(self.game.clone()),
                            location: None,
                        },
                    ))
                    .unwrap();
                    let game = self.game.clone();
                    ctx.link().send_future(async move {
                        let data = post(body, format!("/{}/{}", game, name.clone()).as_str())
                            .await
                            .unwrap();
                        Msg::Response(data)
                    });
                    true
                }
                crate::support::EditTypes::Location => {
                    let body = serde_json::to_string(&middleware::Request::PageShow(
                        middleware::InfoRequestBodyTypes::Location {
                            name: name.clone(),
                            game: Some(self.game.clone()),
                        },
                    ))
                    .unwrap();

                    let game = self.game.clone();
                    ctx.link().send_future(async move {
                        let data = post(body, format!("/{}/{}", game, name.clone()).as_str())
                            .await
                            .unwrap();
                        Msg::Response(data)
                    });
                    true
                }
                crate::support::EditTypes::Loot => {
                    let body = serde_json::to_string(&middleware::Request::PageShow(
                        middleware::InfoRequestBodyTypes::Loot {
                            name: name.clone(),
                            game: Some(self.game.clone()),
                            location: None,
                            mob: None,
                        },
                    ))
                    .unwrap();

                    let game = self.game.clone();
                    ctx.link().send_future(async move {
                        let data = post(body, format!("/{}/{}", game, name.clone()).as_str())
                            .await
                            .unwrap();
                        Msg::Response(data)
                    });
                    true
                }
                _ => false,
            },
            Msg::Response(response) => match response {
                middleware::Response::Error(error_type, message) => false,
                middleware::Response::Success(success_type, message) => match success_type {
                    middleware::SuccessType::Custon(_) => false,
                    middleware::SuccessType::User => {
                        self.email = String::new();
                        self.password = String::new();
                        self.user = support::UserType::User(message);
                        let document = gloo_utils::document();
                        let modal = document.get_element_by_id("id01").unwrap();
                        let modal2 = document.get_element_by_id("id02").unwrap();
                        let style = modal.attributes().get_named_item("style").unwrap();
                        style
                            .set_value(&style.value().replace("display: block;", "display: none;"));
                        let style2 = modal2.attributes().get_named_item("style").unwrap();
                        style2.set_value(
                            &style2.value().replace("display: block;", "display: none;"),
                        );

                        self.context.user_type = self.user.clone();
                        true
                    }
                    middleware::SuccessType::Admin => {
                        self.email = String::new();
                        self.password = String::new();
                        self.confirm = String::new();
                        self.user = support::UserType::Administrator(message);
                        let document = gloo_utils::document();
                        let modal = document.get_element_by_id("id01").unwrap();
                        let modal2 = document.get_element_by_id("id02").unwrap();
                        let style = modal.attributes().get_named_item("style").unwrap();
                        style
                            .set_value(&style.value().replace("display: block;", "display: none;"));
                        let style2 = modal2.attributes().get_named_item("style").unwrap();
                        style2.set_value(
                            &style2.value().replace("display: block;", "display: none;"),
                        );
                        self.context.user_type = self.user.clone();
                        true
                    }
                },
                middleware::Response::PageShow(info) => match info {
                    middleware::InfoResponseBodyTypes::Game { info, background } => {
                        let body = serde_json::to_string(&middleware::Request::Getter(
                            middleware::GetterRequestBodyTypes::LocationListByGame(
                                self.game.clone(),
                            ),
                        ))
                        .unwrap();

                        ctx.link().send_future(async move {
                            let data = post(body, "/").await.unwrap();
                            Msg::Response(data)
                        });
                        let body = serde_json::to_string(&middleware::Request::Getter(
                            middleware::GetterRequestBodyTypes::MobListByGame(self.game.clone()),
                        ))
                        .unwrap();

                        ctx.link().send_future(async move {
                            let data = post(body, "/").await.unwrap();
                            Msg::Response(data)
                        });
                        let body = serde_json::to_string(&middleware::Request::Getter(
                            middleware::GetterRequestBodyTypes::LootListByGame(self.game.clone()),
                        ))
                        .unwrap();

                        ctx.link().send_future(async move {
                            let data = post(body, "/").await.unwrap();
                            Msg::Response(data)
                        });

                        let history = ctx.link().history().unwrap();
                        self.context = crate::support::InfoProp {
                            user_type: self.user.clone(),
                            content_type: Some(crate::support::EditTypes::Game),
                            description: info.informations_block.clone(),
                            preview: info.preview.clone(),
                        }
                        .clone();
                        history.push(crate::switch::SecondRoute::IndexGame {
                            game: info.name.clone(),
                        });
                        true
                    }
                    middleware::InfoResponseBodyTypes::Mob { info } => {
                        let history = ctx.link().history().unwrap();
                        self.context = crate::support::InfoProp {
                            user_type: self.user.clone(),
                            content_type: Some(crate::support::EditTypes::Mob),
                            description: info.informations_block.clone(),
                            preview: info.preview.clone(),
                        }
                        .clone();
                        history.push(crate::switch::SecondRoute::IndexRelated {
                            game: self.game.clone(),
                            related_name: info.name.clone(),
                        });

                        true
                    }
                    middleware::InfoResponseBodyTypes::Location { info } => {
                        let history = ctx.link().history().unwrap();
                        self.context = crate::support::InfoProp {
                            user_type: self.user.clone(),
                            content_type: Some(crate::support::EditTypes::Location),
                            description: info.informations_block.clone(),
                            preview: info.preview.clone(),
                        }
                        .clone();
                        history.push(crate::switch::SecondRoute::IndexRelated {
                            game: self.game.clone(),
                            related_name: info.name.clone(),
                        });

                        true
                    }
                    middleware::InfoResponseBodyTypes::Loot { info } => {
                        let history = ctx.link().history().unwrap();
                        self.context = crate::support::InfoProp {
                            user_type: self.user.clone(),
                            content_type: Some(crate::support::EditTypes::Loot),
                            description: info.informations_block.clone(),
                            preview: info.preview.clone(),
                        }
                        .clone();
                        history.push(crate::switch::SecondRoute::IndexRelated {
                            game: self.game.clone(),
                            related_name: info.name.clone(),
                        });

                        true
                    }
                },
                middleware::Response::Getter(list_type, list) => {
                    match list_type {
                        middleware::GetterResponseBodyTypes::GameList => {
                            self.game_list = list.clone();
                        }
                        middleware::GetterResponseBodyTypes::LocationList => {
                            self.location_list = list.clone();
                        }
                        middleware::GetterResponseBodyTypes::MobList => {
                            self.mob_list = list.clone();
                        }
                        middleware::GetterResponseBodyTypes::LootList => {
                            self.loot_list = list.clone();
                        }
                    }
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let footer = match self.user.clone() {
            crate::support::UserType::Administrator(_) => {
                html!( <footer class="foot">
                        <nav class="navigation">
                            <a onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: HtmlLinkElement = e.target_unchecked_into();
                                Msg::AddNew
                            })}>{"Add new"}</a>
                            <a onclick={ctx.link().callback(|e: MouseEvent| {
                                let input: HtmlLinkElement = e.target_unchecked_into();
                                Msg::Delete
                            })}>{"Delete"}</a>
                        </nav>
                </footer>)
            }
            _ => {
                html!()
            }
        };
        let user_header = match self.user.clone() {
            super::support::UserType::Unknown => {
                html! {
                    <ul>
                        <li>
                            <a onclick={ ctx.link().callback(|e: MouseEvent| {
                                    let document = gloo_utils::document();
                                    let modal = document.get_element_by_id("id02").unwrap();
                                    let style = modal.attributes().get_named_item("style").unwrap();
                                    style.set_value(&style.value().replace("display: none;","display: block;"));
                                    Msg::Open
                                })}>{"Login"}</a>

                        </li>
                        <li>
                            <a onclick={ ctx.link().callback(|e: MouseEvent| {
                                    let document = gloo_utils::document();
                                    let modal = document.get_element_by_id("id01").unwrap();
                                    let style = modal.attributes().get_named_item("style").unwrap();
                                    style.set_value(&style.value().replace("display: none;","display: block;"));
                                    Msg::Open
                                })}>{"Sign-up"}</a>
                        </li>
                    </ul>
                }
            }
            crate::support::UserType::User(username) => {
                html! {
                    <ul>
                        <li>
                            <label>{format!("{} ",username.clone())}</label>
                            <a onclick={ ctx.link().callback(|e: MouseEvent| {
                                Msg::LogOut
                            })}>{"LogOut"}</a>
                        </li>
                    </ul>
                }
            }
            crate::support::UserType::Administrator(username) => {
                html! {
                    <ul>
                        <li>
                            <label>{format!("Adm:{} ",username.clone())}</label>
                            <a onclick={ ctx.link().callback(|e: MouseEvent| {
                                Msg::LogOut
                            })}>{"LogOut"}</a>
                        </li>
                    </ul>
                }
            }
        };
        let games = self
            .game_list
            .iter()
            .map(|game| {
                html! {
                    <li>
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::GameClick(input.inner_text())
                        })}>{game.clone()}</a>
                    </li>
                }
            })
            .collect::<Html>();
        let locations = self
            .location_list
            .iter()
            .map(|game| {
                html! {
                    <li>
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::RelatedClick(input.inner_text(),crate::support::EditTypes::Location)
                        })}>{game.clone()}</a>
                    </li>
                }
            })
            .collect::<Html>();

        let mobs = self
            .mob_list
            .iter()
            .map(|game| {
                html! {
                    <li>
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::RelatedClick(input.inner_text(), crate::support::EditTypes::Mob)
                        })}>{game.clone()}</a>
                    </li>
                }
            })
            .collect::<Html>();

        let loots = self
            .loot_list
            .iter()
            .map(|game| {
                html! {
                    <li>
                        <a onclick={ctx.link().callback(|e: MouseEvent| {
                            let input: HtmlLinkElement = e.target_unchecked_into();
                            Msg::RelatedClick(input.inner_text(), crate::support::EditTypes::Loot)
                        })}>{game.clone()}</a>
                    </li>
                }
            })
            .collect::<Html>();

        html! {
            <div class="container">
                <header class="Header">
                    <nav class="menu-container">
                        <div class="menu">
                            <ul class="items">
                                {games}
                            </ul>
                            {user_header}
                        </div>
                    </nav>
                </header>
                <div class="Main-Area"></div>
                <nav class="Navigation">
                    <ul class="content_menu">
                        <li>{"Locations"}
                            <ul class="content_sub_menu">{locations}</ul>
                        </li>
                        <li>{"Mobs"}
                            <ul class="content_sub_menu">{mobs}</ul>
                        </li>
                        <li>{"Items"}
                            <ul class="content_sub_menu">{loots}</ul>
                        </li>
                    </ul>
                </nav>
                <ContextProvider<crate::support::InfoProp> context={self.context.clone()}>
                    <BrowserRouter>
                        <Switch<SecondRoute> render={Switch::render(second_switch)} />
                    </BrowserRouter>
                </ContextProvider<crate::support::InfoProp>>
                <div id="id02" class="modal" style="display: none;position: fixed; z-index: 1; left: 0; top: 0; width: 100%; height: 100%; overflow: auto; background-color: #474e5d; padding-top: 50px;">
                        <form class="modal-content" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::LoginSubmit
                        } ) }>
                            <div class="container-sign">
                                <label for="username"><b>{"Username"}</b></label>
                                <input type="text" value={self.username.clone()} placeholder="Enter Your Username" name="username" id="username" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::UsernameChanged(input.value())})}/>

                                <label for="psw"><b>{"Password"}</b></label>
                                <input type="password" value={self.password.clone()} placeholder="Enter Password" name="psw" id="psw" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::PasswordChanged(input.value())})}/>

                                <div class="clearfix">
                                <button type="button" onclick={ ctx.link().callback(|e: MouseEvent| {
                                    let document = gloo_utils::document();
                                    let modal = document.get_element_by_id("id02").unwrap();
                                    let style = modal.attributes().get_named_item("style").unwrap();
                                    style.set_value(&style.value().replace("display: block;","display: none;"));
                                    Msg::Open                                })} class="cancelbtn">{"Cancel"}</button>
                                <button type="submit" class="signupbtn">{"Sign Up"}</button>
                                </div>
                            </div>
                        </form>
                    </div>

                    <div id="id01" class="modal" style="display: none;position: fixed; z-index: 1; left: 0; top: 0; width: 100%; height: 100%; overflow: auto; background-color: #474e5d; padding-top: 50px;">
                        <form class="modal-content" onsubmit={ ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::RegistrationSubmit
                        } ) }>
                            <div class="container-sign">
                                <h1>{"Sign Up"}</h1>
                                <p>{"Please fill in this form to create an account."}</p>
                                <hr/>
                                <label for="username"><b>{"Username"}</b></label>
                                <input type="text" value={self.username.clone()} placeholder="Enter Your Username" name="username" id="username" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::UsernameChanged(input.value())})}/>


                                <label for="email"><b>{"Email"}</b></label>
                                <input type="text" value={self.email.clone()} placeholder="Enter Email" name="email" id="email" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::EmailChanged(input.value())})}/>

                                <label for="psw"><b>{"Password"}</b></label>
                                <input type="password" value={self.password.clone()} placeholder="Enter Password" name="psw" id="psw" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::PasswordChanged(input.value())})}/>

                                if self.error{
                                    <label for="psw-repeat" class={stylist::css!("color: red;")}><b>{"Passwords must be indentical"}</b></label>
                                }
                                else{
                                    <label for="psw-repeat"><b>{"Repeat Password"}</b></label>
                                }
                                <input type="password" value={self.confirm.clone()} placeholder="Repeat Password" name="psw-repeat" id="psw-repeat" required=true
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::PasswordConfirmChanged(input.value())})}/>

                                <p>{"By creating an account you agree to our "}<a href="#" style="color:dodgerblue">{"Terms & Privacy"}</a>{"."}</p>

                                <div class="clearfix">
                                <button type="button" onclick={ ctx.link().callback(|e: MouseEvent| {
                                    let document = gloo_utils::document();
                                    let modal = document.get_element_by_id("id01").unwrap();
                                    let style = modal.attributes().get_named_item("style").unwrap();
                                    style.set_value(&style.value().replace("display: block;","display: none;"));
                                    Msg::Open                                })} class="cancelbtn">{"Cancel"}</button>
                                <button type="submit" class="signupbtn">{"Sign Up"}</button>
                                </div>
                            </div>
                        </form>
                    </div>

                <stylist::yew::Global css=r#"
                .container {
                  display: grid;
                  grid-template-columns: repeat(6, 1fr);
                  grid-template-rows: 1.5fr 0.5fr 4fr 0.5fr;
                  gap: 0px 0px;
                  grid-auto-flow: row;
                }

                .Header {
                  grid-area: 1 / 2 / 2 / 6;
                  color: invert(100%);
                }

                .Main-Area {
                  justify-self: stretch;
                  grid-area: 2 / 2 / 4 / 6;
                }

                .Navigation {
                  grid-area: 2 / 2 / 3 / 6;
                  text-align: center;
                }

                .Content {
                  background-color: whitesmoke;
                  display: grid;
                  grid-template-columns: repeat(3, 1fr);
                  grid-template-rows: 0.3fr 1.7fr;
                  gap: 0px 0px;
                  grid-auto-flow: row;
                  grid-template-areas:
                    "Name Name Name"
                    "Inner_Content Inner_Content Inner_Content";
                  grid-area: 3 / 2 / 4 / 6;
                }
                .content_menu {
                    justify-content: space-around;
                    display: flex;
                    max-height: 50px;
                    list-style-type: none;
                }
                .content_menu > li {
                    float: left;
                    color: white;
                    line-height: 40px;
                    background: transparent;
                    cursor: pointer;
                }
                .conent_sub_menu {
                    list-style-type: none;
                    transform: scale(0);
                    transform-origin: top center;
                    transition: all 300ms ease-in-out;
                }
                .content_sub_menu li {
                    background: gray;
                    padding: 8px 0;
                    color: white;
                    transform: scale(0);
                    transform-origin: top center;
                    transition: all 300ms ease-in-out;
                }
                .content_sub_menu li:last-child {
                    border-bottom: 0;
                }
                    .content_sub_menu li:hover {
                    color: purple;
                }
                .content_menu > li:hover .content_sub_menu li {
                    transform: scale(1);
                }
                    .content_menu > li:hover .content_sub_menu {
                    transform: scale(1);
                }
                .Name {
                  grid-area: Name;
                  margin-left: 20px;
                  margin-bottom: 20px;
                }

                .Inner_Content {
                  margin-left: 20px;
                  margin-right: 20px;
                  margin-bottom: 20px;
                  grid-area: Inner_Content;
                }

                .Footer {
                  grid-area: 4 / 1 / 5 / 7;
                }

                html,
                body,
                .container {
                  height: 100%;
                  margin: 0;
                }
                .portable-infobox {
                  border-radius: 3px;
                  border-style: solid;
                  border-width: 1px;
                  clear: right;
                  float: right;
                  margin: 0 0 18px 18px;
                  width: 270px;
                }
                .content-img {
                  max-width: 100%;
                  height: auto;
                }
                .menu-container {
                  position: relative;
                  display: flex;
                  align-items: center;
                  margin-bottom: 20px;
                  background: transparent;
                  color: invert(100%);
                  padding: 20px;
                  z-index: 1;
                  -webkit-user-select: none;
                  user-select: none;
                  box-sizing: border-box;
                }

                .menu-container a {
                  text-decoration: none;
                  color: white;
                  transition: color 0.3s ease;
                }

                .menu-container a:hover {
                  background-color: invert(100%);
                }
                .menu .items {
                  overflow-x: auto;
                }
                .menu ul {
                  list-style: none;
                  display: flex;
                  padding: 0;
                }

                .menu li {
                  padding: 10px 0;
                  font-size: 22px;
                  padding: 0 20px;
                }
                .menu {
                  position: relative;
                  width: 100%;
                  display: flex;
                  justify-content: space-between;
                }
                /* Add padding to containers */
/* Full-width input fields */
input[type=text], input[type=password] {
  width: 100%;
  padding: 15px;
  margin: 5px 0 22px 0;
  display: inline-block;
  border: none;
  background: #f1f1f1;
}

/* Add a background color when the inputs get focus */
input[type=text]:focus, input[type=password]:focus {
  background-color: #ddd;
  outline: none;
}

/* Set a style for all buttons */
button {
  background-color: #04AA6D;
  color: white;
  padding: 14px 20px;
  margin: 8px 0;
  border: none;
  cursor: pointer;
  width: 100%;
  opacity: 0.9;
}

button:hover {
  opacity:1;
}

/* Extra styles for the cancel button */
.cancelbtn {
  padding: 14px 20px;
  background-color: #f44336;
}

/* Float cancel and signup buttons and add an equal width */
.cancelbtn, .signupbtn {
  float: left;
  width: 50%;
}

/* Add padding to container elements */
.container-sign {
  padding: 16px;
}


/* Modal Content/Box */
.modal-content {
  background-color: #fefefe;
  margin: 5% auto 15% auto; /* 5% from the top, 15% from the bottom and centered */
  border: 1px solid #888;
  width: 80%; /* Could be more or less, depending on screen size */
}

/* Style the horizontal ruler */
hr {
  border: 1px solid #f1f1f1;
  margin-bottom: 25px;
}
 
/* The Close Button (x) */
.close {
  position: absolute;
  right: 35px;
  top: 15px;
  font-size: 40px;
  font-weight: bold;
  color: #f1f1f1;
}

.close:hover,
.close:focus {
  color: #f44336;
  cursor: pointer;
}

/* Clear floats */
.clearfix::after {
  content: "";
  clear: both;
  display: table;
}

/* Change styles for cancel button and signup button on extra small screens */
@media screen and (max-width: 300px) {
  .cancelbtn, .signupbtn {
     width: 100%;
  }
}
select {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  background-color: transparent;
  border: none;
  padding: 0 1em 0 0;
  margin: 0;
  width: 100%;
  font-family: inherit;
  font-size: inherit;
  cursor: inherit;
  line-height: inherit;
  z-index: 1;
  outline: none;
}

.select {
  display: grid;
  grid-template-areas: "select";
  align-items: center;
  position: relative;
  min-width: 15ch;
  max-width: 30ch;
  border: 1px solid var(--select-border);
  border-radius: 0.25em;
  padding: 0.25em 0.5em;
  font-size: 1.25rem;
  cursor: pointer;
  line-height: 1.1;
}
.create_container {  display: grid;
  grid-template-columns: 0.2fr 1fr repeat(2, 1.5fr) 1.6fr 0.2fr;
  grid-template-rows: 0.2fr 1fr 1.2fr 1.7fr repeat(3, 1.4fr) 0.5fr 0.2fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    ". . . . . ."
    ". Name_Row Name_Row Name_Row Name_Row ."
    ". Description_Row Description_Row Description_Row Description_Row ."
    ". Icon_Row Icon_Row Icon_Row Icon_Row ."
    ". Games_Row Games_Row Games_Row Games_Row ."
    ". Locations_Row Locations_Row Locations_Row Locations_Row ."
    ". Mobs_Row Mobs_Row Mobs_Row Mobs_Row ."
    ". Submit_Row Submit_Row Submit_Row Submit_Row ."
    ". . . . . .";
}

.Name_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col_25 Col-75";
  grid-area: Name_Row;
}

.Col_25 { grid-area: Col_25; }

.Col-75 { grid-area: Col-75; }

.Description_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col-Des_25 Col-Des_75";
  grid-area: Description_Row;
}

.Col-Des_25 { grid-area: Col-Des_25; }

.Col-Des_75 { grid-area: Col-Des_75; }

.Icon_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col-Icon_25 Col-Icon_75";
  grid-area: Icon_Row;
}

.Col-Icon_25 { grid-area: Col-Icon_25; }

.Col-Icon_75 { grid-area: Col-Icon_75; }

.Games_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col-Games_25 Col-Games_75";
  grid-area: Games_Row;
}

.Col-Games_25 { grid-area: Col-Games_25; }

.Col-Games_75 { grid-area: Col-Games_75; }

.Locations_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col-Locs_25 Col-Locs_75";
  grid-area: Locations_Row;
}

.Col-Locs_25 { grid-area: Col-Locs_25; }

.Col-Locs_75 { grid-area: Col-Locs_75; }

.Mobs_Row {  display: grid;
  grid-template-columns: 0.25fr 1.75fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-auto-flow: row;
  grid-template-areas:
    "Col-Mobs_25 Col-Mobs_75";
  grid-area: Mobs_Row;
}

.Col-Mobs_25 { grid-area: Col-Mobs_25; }

.Col-Mobs_75 { grid-area: Col-Mobs_75; }

.Submit_Row { grid-area: Submit_Row; }

                "#/>

            {footer}
            </div>
        }
    }
}
