use yew::services::storage::Area;
use yew::services::StorageService;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, NodeRef, ShouldRender};
use yew_router::agent::RouteRequest;
use yew_router::prelude::{Route, RouteAgent};
use yew_router::{router::Router, Switch};

use web_sys::HtmlElement;

use super::browse::{self, Browse, BrowseRoute};
use super::chapter::Chapter;
use super::login::Login;
use super::logout::Logout;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/catalogue/{source}/manga/{title}/chapter/{chapter}/page/{page}"]
    Chapter(String, String, String, usize),
    #[to = "/login"]
    Login,
    #[to = "/logout"]
    Logout,
    #[to = "{*:path}"]
    Browse(BrowseRoute),
}

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    router: Box<dyn Bridge<RouteAgent>>,
    route: String,
    refs: Vec<NodeRef>,
}

pub enum Msg {
    RouterCallback(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let callback = link.callback(|route| Msg::RouterCallback(route));
        let router = RouteAgent::bridge(callback);
        App {
            link,
            storage,
            router,
            route: "/".to_string(),
            refs: vec![NodeRef::default(), NodeRef::default()],
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Err(_) = self.storage.restore("token") {
            self.router
                .send(RouteRequest::ChangeRoute(Route::from("/login".to_string())));
        } else {
            self.router.send(RouteRequest::GetCurrentRoute);
        }
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouterCallback(route) => {
                self.route = route.route;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="w-full h-screen">
                <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::Chapter(source, title, chapter, page) => html!{<Chapter source=source title=title chapter=chapter page=page/>},
                    AppRoute::Login => html!{<Login />},
                    AppRoute::Logout => html!{<Logout />},
                    AppRoute::Browse(route) => {
                        let route: browse::Props = route.into();
                        html!{<Browse with route/>}
                    },
                }}) />
            </div>
        }
    }
}

impl App {
    fn hide(&self) {
        if let Some(top_bar) = self.refs[0].cast::<HtmlElement>() {
            top_bar.set_hidden(true);
        }
        if let Some(nav_bar) = self.refs[0].cast::<HtmlElement>() {
            nav_bar.set_hidden(true);
        }
    }
}
