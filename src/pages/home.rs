use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <main>
        <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
        <h1>{ "Hello World!" }</h1>
        <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        <a href="/sub">{"sub link"}</a>
      </main>
    }
}
