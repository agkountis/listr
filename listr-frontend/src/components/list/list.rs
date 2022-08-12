use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use listr_common::ListWithItemsResponse;
use crate::components::list::list_item::*;
use crate::components::input::*;
use crate::requests::lists::{add_list_item, fetch_list_items};

#[derive(PartialEq, Clone, Copy, Properties)]
pub struct ListProps {
    pub id: i32,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let list_with_items = use_state(|| ListWithItemsResponse::default());
    let refresh = use_state(|| true);
    let input_ref = use_node_ref();

    let oauth_context = use_context::<OAuth2Context>();
    let token = use_ref(|| oauth_context.unwrap().access_token().map(|v| v.to_string()));

    let list_id = props.id;

    {
        let refresh = refresh.clone();
        let list_with_items = list_with_items.clone();
        let token = token.clone();
        // Use effect is a hook that runs every time the component gets re-rendered
        // (a re-render is triggered when state gets altered). all variables created through the use_state
        // hook are considered state.
        //
        // use_effect_with_deps is a a variant of the use_effect hook.
        // The difference is that use_effect_with_deps does not get invoked for every state change
        // but only if the dependency passed into it is altered.
        // We are leveraging this behaviour here to trigger a re-render/re-fetch of the list items
        // After a new one has been created through the onclick callback by altering the refresh state passed as a dependency.
        // See bellow.
        use_effect_with_deps(
            move |refresh| {
                let refresh = refresh.clone();
                if *refresh {
                    // let credentials = oauth_context.unwrap();
                    // let token = credentials.access_token().unwrap();
                    info!("Fetching Lists!");

                    if let Some(token) = (&*token).clone() {
                        wasm_bindgen_futures::spawn_local(async move {
                            list_with_items.set(fetch_list_items(list_id, &token).await);
                            refresh.set(false);
                        });
                    }
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

    let add_item = {
        let do_refresh = do_refresh.clone();
        let token = token.clone();
        Callback::from(move |item_data| {
            let token = token.clone();
            let do_refresh = do_refresh.clone();

            if let Some(token) = (&*token).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    // let credentials = oauth_context.unwrap();
                    // let token = credentials.access_token().unwrap();
                    add_list_item(list_id, item_data, &token).await;
                    do_refresh.emit(());
                });
            }
        })
    };

    html! {
        <>
            <Authenticated>
                <h1>{ format!("{}", list_with_items.name) }</h1>
                <ActionInput input_type="text" action_name="Add Item" action={add_item} />
                <br/><br/>
                <ListItems items={list_with_items.items.clone()} refresh={do_refresh}/><br/>
                <Link<Route> to={Route::Lists}>{"Back"}</Link<Route>>
            </Authenticated>
            <NotAuthenticated>
                <Redirect<Route> to={Route::Home} />
            </NotAuthenticated>
        </>
    }
}
