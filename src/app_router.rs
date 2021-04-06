use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]

pub enum AppRoute {
    #[to = "/template"]
    Template,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|route: AppRoute| match route {
            AppRoute::Index => html! { <pages::Home/> },
            AppRoute::Template => html! { <pages::Template/> },
            _ => html! {<pages::Home/>},
        });
        html! {
            <div>
                <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container-fluid">
                    <span class="navbar-brand">{"Navbar"}</span>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <Link route=AppRoute::Index><span class="nav-link active">{"Home"}</span></Link>
                        </li>
                        <li class="nav-item">
                            <Link route=AppRoute::Template><span class="nav-link active">{"Template"}</span></Link>
                        </li>
                    </ul>
                    <form class="d-flex">
                        <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                        <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                    </form>
                    </div>
                </div>
                </nav>
                    <Router<AppRoute, ()> render=render />
            </div>
        }
    }
}
