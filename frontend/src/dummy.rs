use crate::app::Body;
use crate::pages::page_not_found::PageNotFound;
use crate::support::fetch_post as post;
use crate::switch::AppRoute;
use web_sys::HtmlLinkElement;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::PageNotFound => {
            html! { <PageNotFound/> }
        }
        AppRoute::App => {
            html! { <Body/>}
        }
    }
}
pub struct Dummy {}
impl Component for Dummy {
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
            <BrowserRouter>
                <Body/>
            </BrowserRouter>
        }
    }
}
