use yew::prelude::*;

pub struct Template {
    link: ComponentLink<Self>,
}

impl Component for Template {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                <div class="card-body">
                    <p>{"Card"}</p>
                </div>
            </div>
        }
    }
}
