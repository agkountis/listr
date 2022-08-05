use crate::Route;
use gloo_net::http::*;
use listr_common::{AddListItemRequest, ListItemResponse, ListResponse, ListWithItemsResponse};
use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

const BACKEND_URL: &str = "https://localhost:80";

async fn fetch_lists(token: &str) -> Vec<ListResponse> {
    Request::get(&format!("{}/api/v1/lists", BACKEND_URL))
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn fetch_list_items(list_id: i32, token: &str) -> ListWithItemsResponse {
    Request::get(&format!("{}/api/v1/lists/{}", BACKEND_URL, list_id))
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn add_list_item(list_id: i32, item_data: String, token: &str) {
    info!("Submitting POST request for list_id:{} item_data:{} with token:{}", list_id, item_data, token);
    Request::post(&format!("{}/api/v1/lists/{}/add", BACKEND_URL, list_id))
        .header("Authorization", token)
        .json(&AddListItemRequest { value: item_data })
        .unwrap()
        .send()
        .await
        .unwrap();
}

async fn delete_list_item(item_id: i32, token: &str) {
    info!("Submitting DELETE request for item_id:{} with token:{}", item_id, token);
    Request::delete(&format!("{}/api/v1/lists/item/{}/delete", BACKEND_URL,item_id))
        .header("Authorization", token)
        .send()
        .await
        .unwrap();
}

#[function_component(Lists)]
pub fn lists() -> Html {
    let lists = use_state(|| vec![]);
    let oauth_context = use_context::<OAuth2Context>();

    {
        let lists = lists.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let credentials = oauth_context.unwrap();
                    let token = credentials.access_token().unwrap();
                    lists.set(fetch_lists(token).await);
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

#[derive(Properties, PartialEq, Clone)]
pub struct ListItemProps {
    pub item: ListItemResponse,
    pub refresh: Callback<()>,
}

#[function_component(ListItem)]
pub fn list_item(props: &ListItemProps) -> Html {
    let item_id = props.item.id;
    let refresh = props.refresh.clone();
    let oauth_context = use_context::<OAuth2Context>();

    let onclick = {
        let refresh = refresh.clone();

        move |_| {
            let refresh = refresh.clone();
            let oauth_context = oauth_context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let credentials = oauth_context.unwrap();
                let token = credentials.access_token().unwrap();
                delete_list_item(item_id, token).await;
                refresh.emit(())
            });
        }
    };

    html! {
        <>
            <tr>
                <td>
                    <button {onclick}>{"x"}</button>
                    <label>{format!(" - {}", props.item.value)}</label>
                </td>
            </tr>
        </>
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct ListItemsProps {
    pub items: Vec<ListItemResponse>,
    pub refresh: Callback<()>,
}

#[function_component(ListItems)]
pub fn list_items(props: &ListItemsProps) -> Html {
    html! {
        <>
            <table>
                {
                    props.items.iter()
                    .map(|item| html!{ <ListItem item={item.clone()} refresh={props.refresh.clone()}/> })
                    .collect::<Html>()
                }
            </table>
        </>
    }
}

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
    let token = use_ref(|| oauth_context.unwrap().access_token().unwrap().to_string());

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
                    wasm_bindgen_futures::spawn_local(async move {
                        list_with_items.set(fetch_list_items(list_id, &token).await);
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

    let add_item = {
        let input_ref = input_ref.clone();
        let do_refresh = do_refresh.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let token = token.clone();
            // Cast DOM node ref to HtmlInputElement to access the input's value
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let item_data = input.value();
                let do_refresh = do_refresh.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    // let credentials = oauth_context.unwrap();
                    // let token = credentials.access_token().unwrap();
                    add_list_item(list_id, item_data, &token).await;
                    do_refresh.emit(());
                });

                // Remove text from the input after successful item creation.
                input.set_value("")
            }
        })
    };

    html! {
        <>
            <Authenticated>
                <h1>{ format!("{}", list_with_items.name) }</h1>
                <input type="text" ref={input_ref}/>
                <button onclick={add_item}>{"Add Item"}</button>
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
