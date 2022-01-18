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
                  <h1>{"Header"}</h1>
              </div>
              <div class="Inner_Content">
                  <aside class="portable-infobox" role="region">
                      <img
                          class="content-img"
                          src="https://static.wikia.nocookie.net/gensin-impact/images/8/80/Emblem_Mondstadt.png/revision/latest/scale-to-width-down/256?cb=20201116194623"
                      />
                  </aside>
                  <h1>{"fsdfffffffffffffffffff"}</h1>
              </div>
          </div>
        }
    }
}
