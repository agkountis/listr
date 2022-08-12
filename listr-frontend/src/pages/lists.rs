use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;
use crate::requests::lists::{create_list, fetch_lists};
use crate::routes::Route;
use crate::components::input::*;

#[function_component(Lists)]
pub fn lists() -> Html {
    let lists = use_state(|| vec![]);
    let refresh = use_state(|| true);
    let oauth_context = use_context::<OAuth2Context>().unwrap();
    let oauth_context_ref = use_ref(|| oauth_context);

    {
        let lists = lists.clone();
        let refresh = refresh.clone();
        let oauth_context = oauth_context_ref.clone();

        use_effect_with_deps(
            move |refresh| {
                let refresh = refresh.clone();
                if *refresh {
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(token) = oauth_context.access_token() {
                            lists.set(fetch_lists(token).await);
                        }
                        refresh.set(false);
                    });
                }

                || {}
            },
            refresh,
        );
    }

    let do_refresh = Callback::from(move |_| {
        let refresh = refresh.clone();
        refresh.set(true);
    });

    let add_list = {
        let oauth_context = oauth_context_ref.clone();
        let do_refresh = do_refresh.clone();

        Callback::from(move |input_value: String| {

            let oauth_context = oauth_context.clone();
            let do_refresh = do_refresh.clone();
            wasm_bindgen_futures::spawn_local(async move {

                let do_refresh = do_refresh.clone();
                if let Some(token) = oauth_context.access_token() {
                    create_list(&input_value, token).await;
                    do_refresh.emit(())
                }
            })
        })
    };

    html! {
        <>
            <Authenticated>
                <h1>{"Lists"}</h1>
                <ActionInput input_type="text" action_name="Create List" action={add_list} />
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
