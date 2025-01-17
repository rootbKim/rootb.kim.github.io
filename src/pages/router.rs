use super::{
    about::About, archive::Archive, home::Home, navbar::Navbar, portfolio::Portfolio, post::Post,
    search::Search,
};
use crate::component::page::Page;
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <HashRouter>
                <Navbar />
                <div class="main-wrapper">
                    <div class="main-content">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </HashRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Post => {
            html! { <Post /> }
        }
        Route::Portfolio => {
            html! { <Portfolio /> }
        }
        Route::Archive => {
            html! { <Archive /> }
        }
        Route::Page { class, filename } => {
            html! { <Page class={class.clone()} filename={filename.clone()} /> }
        }
        Route::About => {
            html! { <About /> }
        }
        Route::Search => {
            html! { <Search /> }
        }
        Route::NotFound => {
            html! {
                <>
                    <div class="not-found">
                        {"Page Not Found"}
                    </div>
                    <div class="not-found-contents">
                        { "The page you visited has an invalid or deleted address." }
                    </div>
                </>
            }
        }
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post/")]
    Post,
    #[at("/portfolio/")]
    Portfolio,
    #[at("/archive/")]
    Archive,
    #[at("/:class/:filename")]
    Page { class: String, filename: String },
    #[at("/about/")]
    About,
    #[at("/search/")]
    Search,
    #[not_found]
    #[at("/404")]
    NotFound,
}
