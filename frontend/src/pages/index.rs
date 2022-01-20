use yew::prelude::*;

pub struct Index {}

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
          <div class="Content">
              <div class="Name">
                  <h1>{"Universal MMORPG Wikipedia"}</h1>
              </div>
              <div class="Inner_Content">
              </div>
          </div>
        }
    }
}
