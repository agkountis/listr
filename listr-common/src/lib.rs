use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetListsResponse {
    pub lists: Vec<ListResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetListItemsResponse {
    pub list_items: Vec<ListItemResponse>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct ListWithItemsResponse {
    pub name: String,
    pub items: Vec<ListItemResponse>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ListItemResponse {
    pub id: i32,
    pub value: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AddListItemRequest {
    pub value: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    fname: String,
    passwd: String,
}
