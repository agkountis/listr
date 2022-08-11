use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;
use crate::requests::lists::fetch_lists;
use crate::routes::Route;

#[function_component(Lists)]
pub fn lists() -> Html {
    let lists = use_state(|| vec![]);
    let oauth_context = use_context::<OAuth2Context>().unwrap();

    {
        let lists = lists.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(token) = oauth_context.access_token() {
                        lists.set(fetch_lists(token).await);
                    }
                });

                || {}
            },
            (),
        );
    }

    html! {
        <>
            <Authenticated>
                <h1>{"Lists"}</h1>
                <ul>
                    {
                        lists.iter().map(|list| html! {
                            <>
                                <li><Link<Route> to={ Route::List{ id: list.id } }>{ format!("{}", list.name) }</Link<Route>></li>
                            </>
                        }).collect::<Html>()
                    }
                </ul>
            </Authenticated>
            <NotAuthenticated>
                <Redirect<Route> to={Route::Home}/>
            </NotAuthenticated>
        </>
    }
}
