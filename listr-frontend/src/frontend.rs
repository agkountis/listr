mod components;
mod pages;
mod requests;
mod routes;

use components::nav::*;

use yew::prelude::*;
use yew_oauth2::oauth2::*;

#[function_component(App)]
fn app() -> Html {

    let config = Config {
        client_id: "2k4mvc8v79o3b1vnvvmtgl36g6".to_string(),
        auth_url: "https://listr.auth.us-east-1.amazoncognito.com/login".to_string(),
        token_url: "https://localhost:80/auth/token".to_string(),
    };

    html! {
        <>
            <OAuth2 {config}>
                    <Nav />
            </OAuth2>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
