use serde::{Deserialize, Serialize};
use jsonwebtokens::raw;

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

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CreateListRequest {
    pub name: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AddListItemRequest {
    pub value: String,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub sub: String,
}

pub fn extract_user_from_token(access_token: &str) -> User {
    let token_data = raw::decode_only(access_token).unwrap();
    let mut username = token_data.claims.get("username").unwrap().to_string();
    let mut sub = token_data.claims.get("sub").unwrap().to_string();

    username.retain(|c| c != '"');
    sub.retain(|c| c != '"');

    User {
        username,
        sub,
    }
}
