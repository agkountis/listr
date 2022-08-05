mod components;
mod pages;

use components::lists::*;
use pages::home::*;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/lists")]
    Lists,
    #[at("/lists/:id")]
    List { id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Lists => html! { <Lists/> },
        Route::List { id } => html! { <List id={*id} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let login = |_| OAuth2Dispatcher::<Client>::new().start_login();

    let logout = |_| OAuth2Dispatcher::<Client>::new().logout();

    let config = Config {
        client_id: "2k4mvc8v79o3b1vnvvmtgl36g6".to_string(),
        auth_url: "https://listr.auth.us-east-1.amazoncognito.com/login".to_string(),
        token_url: "https://localhost:80/auth/token".to_string(),
    };

    html! {
        <>
            <OAuth2 {config}>
                <Authenticated>
                    <BrowserRouter>
                        <nav>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>{"|"}
                            <Link<Route> to={Route::Lists}>{"Lists"}</Link<Route>>{"|"}
                            <a onclick={logout.clone()}>{"Log Out"}</a>{"|"}
                        </nav>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </Authenticated>
                <NotAuthenticated>
                    <BrowserRouter>
                        <nav>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>{"|"}
                            <a onclick={login.clone()}>{"Log In"}</a>{"|"}
                        </nav>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </NotAuthenticated>
            </OAuth2>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
