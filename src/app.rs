use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, sub::Sub};
use crate::routes::AppRoutes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <Switch<AppRoutes> render={Switch::render(switch)} />
      </BrowserRouter>
    }
}

fn switch(app_routes: &AppRoutes) -> Html {
    match app_routes {
        AppRoutes::Home => html! {<Home/>},
        AppRoutes::Sub => html! {<Sub/>},
    }
}
