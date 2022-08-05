use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct List {
    pub id: i32,
    pub name: String,
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
