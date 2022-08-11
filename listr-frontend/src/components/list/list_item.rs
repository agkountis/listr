use yew::prelude::*;
use yew_oauth2::prelude::*;
use listr_common::ListItemResponse;
use crate::requests::lists::delete_list_item;

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
