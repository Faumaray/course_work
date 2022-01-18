use serde::Serialize;
use yew::prelude::*;

use yew_router::{history::History, prelude::RouterScopeExt};
#[derive(PartialEq, Properties, Serialize)]
pub struct Props {
    pub game_name: String,
    pub related_name: String,
}
pub enum Msg {
    EditIcon,
    EditName,
    EditDescription,
}

pub struct IndexRelated {}

impl Component for IndexRelated {
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
            Msg::EditIcon => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().related_name.clone(),
                    part: crate::support::EditTypes::Image,
                });
                return false;
            }
            Msg::EditName => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().related_name.clone(),
                    part: crate::support::EditTypes::Name,
                });
                return false;
            }
            Msg::EditDescription => {
                let history = ctx.link().history().unwrap();
                history.push(crate::switch::SecondRoute::EditContent {
                    typ: data.content_type.clone().unwrap(),
                    name_t: ctx.props().related_name.clone(),
                    part: crate::support::EditTypes::Description,
                });
                return false;
            }
        }
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
                    Msg::EditName
                    })}>{".Edit"}</a>),
                html!(
                    <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                    Msg::EditDescription
                    })}>{".Edit"}</a>),
                html!(
                    <a class="edit" onclick={ctx.link().callback(|e: MouseEvent| {
                    Msg::EditIcon
                    })}>{".Edit"}</a>),
            ),
            _ => (html!(), html!(), html!()),
        };

        html! {
            <div class="Content">
              <div class="Name">
                  <h1>{ctx.props().related_name.clone().replace("%20", " ")}</h1>{header}
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
