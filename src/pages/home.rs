use yew::prelude::*;

// mod counter;
use crate::components::counter::Counter;

pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
    SubtractOne,
    MultiplyByTwo,
    // Arithmetic { kind: _, num: i32 },
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::SubtractOne => self.value -= 1,
            Msg::MultiplyByTwo => self.value *= 2,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div id="carouselExampleSlidesOnly" class="carousel slide" data-bs-ride="carousel">
                <div class="carousel-inner">
                    <div class="carousel-item active">
                        <img src="./images/background-image.jpg" class="d-block w-100" alt="Background Image"/>
                    </div>
                </div>
                </div>
                <div class="container">
                    <button class="btn btn-primary" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <button class="btn btn-danger" onclick=self.link.callback( |_| Msg::SubtractOne )>{"-1"}</button>
                    <button class="btn btn-primary" onclick=self.link.callback(|_| Msg::MultiplyByTwo)>{"*2"}</button>
                    // <Counter count={self.value}/>
                    <Counter>
                        <p>{self.value}</p>
                    </Counter>
                </div>
            </div>
        }
    }
}
