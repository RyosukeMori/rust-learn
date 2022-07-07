use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
    #[at("/")]
    Home,
    #[at("/sub")]
    Sub,
}
