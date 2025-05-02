mod backend;
mod components;

use components::{DogView, NavBar, Favorites};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

static CSS: Asset = asset!("/assets/main.css");

// Create a new wrapper type
#[derive(Clone)]
struct TitleState(String);

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        // 📣 delete Title and DogView and replace it with the Router component.
        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)] // <---- add the #[layout] attribute
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites, // <------ add this new variant
}
