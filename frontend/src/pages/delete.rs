use yew::prelude::*;

pub struct Delete;

pub enum Msg {}

impl Component for Delete {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {<>
            <div id="main_div" class="main_div">
            </div>
            <stylist::yew::Global css=r#"
            .main_div{
                display: flex;
                flex-direction: row;
                height:740px;
                line-height: 1.5;
                margin-top: 10%;
                margin-left: 15%;
                margin-right: 15%;
                background: white;
                border-radius: 12px;
            }
            
            "#/>
        </>}
    }
}
