use crate::pages::{
    add_new::AddNew, delete::Delete, edit_content::EditContent, page_not_found::PageNotFound,
    viewer::Viewer,
};
use crate::switch::AppRoute;
use stylist::css;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Index {}
fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::PageNotFound => {
            html! { <PageNotFound/> }
        }
        AppRoute::Viewer => {
            html! { <Viewer/> }
        }
        AppRoute::AddNew => {
            html! { <AddNew/> }
        }
        AppRoute::Delete => {
            html! { <Delete/> }
        }
        AppRoute::EditContent { typ, name_t, part } => {
            html! { <EditContent part={part} edit_type={typ} name={name_t.clone()}/> }
        }
    }
}
impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter >
                    <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
