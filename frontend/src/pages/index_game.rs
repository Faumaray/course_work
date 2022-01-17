use crate::support::fetch_post as fetch;
use stylist::css;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlLinkElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};
pub struct IndexGame {}
#[derive(PartialEq, Properties)]
pub struct Props {
    pub game_name: String,
}

pub enum Msg {
    EditIcon,
    EditName,
    EditDescription,
    ReceiveResponse(Result<middleware::Response, reqwasm::Error>),
}
impl Component for IndexGame {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (data, _) = ctx
            .link()
            .context::<crate::support::InfoProp>(Callback::noop())
            .unwrap();
        match msg {
            Msg::ReceiveResponse(response) => match response {
                Ok(body) => match body {
                    middleware::Response::Error(_, _) => todo!(),
                    middleware::Response::Success(_, _) => todo!(),
                    middleware::Response::PageShow(_) => todo!(),
                    middleware::Response::Getter(_, _) => todo!(),
                    middleware::Response::GetterDeleteBlockList(_) => todo!(),
                },
                Err(error) => true,
            },
            Msg::EditIcon => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().game_name.clone(),
                    part: crate::support::EditTypes::Image,
                });
                return false;
            }
            Msg::EditName => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().game_name.clone(),
                    part: crate::support::EditTypes::Name,
                });
                return false;
            }
            Msg::EditDescription => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().game_name.clone(),
                    part: crate::support::EditTypes::Description,
                });
                return false;
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (data, _) = ctx
            .link()
            .context::<crate::support::InfoProp>(Callback::noop())
            .unwrap();
        let (header, content, icon) = match data.user_type {
            crate::support::UserType::Administrator(_) => (
                html!(
                    <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                    let input: HtmlLinkElement = e.target_unchecked_into();
                    Msg::EditName
                    })}>{".Edit"}</a>),
                html!(
                    <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                    let input: HtmlLinkElement = e.target_unchecked_into();
                    Msg::EditDescription
                    })}>{".Edit"}</a>),
                html!(
                    <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                    let input: HtmlLinkElement = e.target_unchecked_into();
                    Msg::EditIcon
                    })}>{".Edit"}</a>),
            ),
            _ => (html!(), html!(), html!()),
        };

        html! {
          <div class="Content">
              <div class="Name">
                  <h1>{ctx.props().game_name.clone().replace("%20", " ")}</h1>{header}
              </div>
              <div class="Inner_Content">
                  <aside class="portable-infobox" role="region">
                      <img
                          class="content-img"
                          src={format!("data:image/png;base64,{}", base64::encode(data.preview.clone().unwrap_or_default()))}
                      />{icon}
                  </aside>
                  <h3>{data.description.clone().unwrap_or_default()}</h3>{content}
              </div>
          </div>

        }
    }
}
