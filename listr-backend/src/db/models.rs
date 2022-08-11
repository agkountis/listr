use crate::db::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub user_id: String,
}

#[derive(Insertable)]
#[table_name = "lists"]
pub struct NewList<'a> {
    pub name: &'a str,
    pub user_id: &'a str,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Item {
    pub id: i32,
    pub list_id: i32,
    pub data: String,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub list_id: i32,
    pub data: &'a str,
}
