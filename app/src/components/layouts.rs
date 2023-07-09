use crate::{components::RequireAuth, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
    pub header_actions_left: Option<Children>,
    pub header_actions_right: Option<Children>,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    let header_left = html! {
        if let Some(header_actions_left) = props.header_actions_left.clone() {
            <div class="actions">
                {header_actions_left}
            </div>
        }
    };

    let header_right = html! {
        if let Some(header_actions_right) = props.header_actions_right.clone() {
            <div class="actions">
                {header_actions_right}
            </div>
        }
    };

    let header_empty =
        props.title.is_empty() && props.header_actions_left.is_none() && props.header_actions_right.is_none();

    html! {
        <RequireAuth>
            if !header_empty {
                <header>
                    {header_left}
                    <div class="space"/>
                    {header_right}
                </header>
            }
            <main id="content" class={props.class.clone()}>
                if !props.title.is_empty() {
                    <h1>{&props.title}</h1>
                }
                {props.children.clone()}
            </main>
        </RequireAuth>
    }
}

#[derive(Properties, PartialEq)]
pub struct RouteLinkProps {
    pub route: Route,
    pub label: AttrValue,
}

#[function_component(RouteLink)]
pub fn route_link(props: &RouteLinkProps) -> Html {
    let route = use_route::<Route>().unwrap();
    let active = route == props.route;
    let class = if active { "active" } else { "" };

    html! {
        <Link<Route> to={props.route.clone()} classes={classes!(class)}>
            { &props.label }
        </Link<Route>>
    }
}
