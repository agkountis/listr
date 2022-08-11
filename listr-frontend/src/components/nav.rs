use yew::prelude::*;
use yew_router::prelude::*;
use yew_oauth2::oauth2::Client;
use yew_oauth2::prelude::*;
use listr_common::User;
use crate::routes::*;
use crate::components::user::*;

#[function_component(AuthenticatedNav)]
pub fn authenticated_nav() -> Html {
    let user = use_context::<User>().expect("No user context present.");

    let logout = |_| OAuth2Dispatcher::<Client>::new().logout();

    html! {
        <>
            <nav>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>{"|"}
                <Link<Route> to={Route::Lists}>{"Lists"}</Link<Route>>{"|"}
                <span>{ format!("  Welcome {} ", user.username) }</span>
                // if let Some(user) = maybe_user {
                //
                // }
                <a onclick={logout.clone()}>{"Log Out"}</a>{"|"}
            </nav>
            <Switch<Route> render={Switch::render(switch)} />
        </>
    }
}

#[function_component(Nav)]
pub fn nav() -> Html {

    let login = |_| OAuth2Dispatcher::<Client>::new().start_login();

    html! {
        <>
            <Authenticated>
                <UserContextProvider>
                    <BrowserRouter>
                        <AuthenticatedNav />
                    </BrowserRouter>
                </UserContextProvider>
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
        </>
    }
}
