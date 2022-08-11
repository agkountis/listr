use gloo_net::http::Request;
use log::{error, info};
use listr_common::{AddListItemRequest, ListResponse, ListWithItemsResponse};
use crate::requests::BACKEND_URL;

pub async fn fetch_lists(token: &str) -> Vec<ListResponse> {
    Request::get(&format!("{}/api/v1/lists", BACKEND_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| error!("{:?}", e))
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn fetch_list_items(list_id: i32, token: &str) -> ListWithItemsResponse {
    Request::get(&format!("{}/api/v1/lists/{}", BACKEND_URL, list_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| error!("{:?}", e))
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn add_list_item(list_id: i32, item_data: String, token: &str) {
    info!("Submitting POST request for list_id:{} item_data:{} with token:{}", list_id, item_data, token);
    Request::post(&format!("{}/api/v1/lists/{}/add", BACKEND_URL, list_id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&AddListItemRequest { value: item_data })
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn delete_list_item(item_id: i32, token: &str) {
    info!("Submitting DELETE request for item_id:{} with token:{}", item_id, token);
    Request::delete(&format!("{}/api/v1/lists/item/{}/delete", BACKEND_URL,item_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
}
