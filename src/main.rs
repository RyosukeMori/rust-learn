mod app;
mod pages;
mod routes;

use app::App;

fn main() {
    yew::start_app::<App>();
}
