use web_sys::HtmlElement;
use yew::services::storage::Area;
use yew::services::StorageService;
use yew::{
    html, Bridge, Bridged, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender,
};
use yew_router::agent::RouteRequest;
use yew_router::prelude::{Route, RouteAgent};
use yew_router::{router::Router, Switch};

use crate::app::catalogue::{self, CatalogueRoute};

use super::catalogue::Catalogue;
use super::chapter::Chapter;
use super::component::NavigationBar;
use super::detail::Detail;
use super::history::History;
use super::home::Home;
use super::login::Login;
use super::logout::Logout;
use super::settings::Settings;
use super::updates::Updates;

#[derive(Switch, Debug, Clone)]
pub enum BrowseRoute {
    #[to = "/catalogue/{source}/manga/{title}"]
    Detail(String, String),
    #[to = "/catalogue{*:rest}"]
    Catalogue(CatalogueRoute),
    #[to = "/updates"]
    Updates,
    #[to = "/history"]
    History,
    #[to = "/settings"]
    Settings,
    #[to = "/"]
    Home,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub source: Option<String>,
    pub title: Option<String>,
}

impl Into<Props> for BrowseRoute {
    fn into(self) -> Props {
        match self {
            BrowseRoute::Detail(source, title) => Props {
                source: Some(source),
                title: Some(title),
            },
            BrowseRoute::Catalogue(route) => Props {
                source: None,
                title: None,
            },
            BrowseRoute::Updates => Props {
                source: None,
                title: None,
            },
            BrowseRoute::History => Props {
                source: None,
                title: None,
            },
            BrowseRoute::Settings => Props {
                source: None,
                title: None,
            },
            BrowseRoute::Home => Props {
                source: None,
                title: None,
            },
        }
    }
}

pub struct Browse {
    props: Props,
    link: ComponentLink<Self>,
    storage: StorageService,
    router: Box<dyn Bridge<RouteAgent>>,
    route: String,
    refs: Vec<NodeRef>,
}

pub enum Msg {
    RouterCallback(Route),
}

impl Component for Browse {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let callback = link.callback(|route| Msg::RouterCallback(route));
        let router = RouteAgent::bridge(callback);
        Browse {
            props,
            link,
            storage,
            router,
            route: "/".to_string(),
            refs: vec![NodeRef::default(), NodeRef::default()],
        }
    }

    fn mounted(&mut self) -> ShouldRender {false}

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
                <>
                    <div ref=self.refs[0].clone() class="block fixed inset-x-0 top-0 z-50 bg-tachiyomi-blue safe-top z-50 shadow"></div>
                    <NavigationBar ref=self.refs[1].clone()/>
                    <Router<BrowseRoute, ()>
                    render = Router::render(|switch: BrowseRoute| {
                    match switch {
                        BrowseRoute::Detail(source, title) => html!{<Detail source=source title=title/>},
                        BrowseRoute::Catalogue(catalogue_route) => html!{<Catalogue route=catalogue_route/>},
                        BrowseRoute::Updates => html!{<Updates/>},
                        BrowseRoute::History => html!{<History/>},
                        BrowseRoute::Home => html!{<Home/>},
                        BrowseRoute::Settings => html!{<Settings />},
                    }}) / >
            < / >
        }
    }
}

impl Browse {
    fn hide(&self) {
        if let Some(top_bar) = self.refs[0].cast::<HtmlElement>() {
            top_bar.set_hidden(true);
        }
        if let Some(nav_bar) = self.refs[0].cast::<HtmlElement>() {
            nav_bar.set_hidden(true);
        }
    }
}
