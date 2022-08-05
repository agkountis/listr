use crate::db;
use crate::models::{Item, List, NewItem};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{delete, get, post, web, Error, HttpResponse};
use diesel::{insert_into, EqAll, QueryDsl, RunQueryDsl};
use listr_common::{AddListItemRequest, ListItemResponse, ListWithItemsResponse};
use log::{error, info};

#[get("/lists")]
pub async fn get_lists() -> Result<HttpResponse, Error> {
    use crate::schema::lists::dsl::*;
    let connection = db::establish_connection();

    let res_lists: Vec<List> = lists
        .load(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(res_lists))
}

#[get("/lists/{id}")]
pub async fn get_items(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    use crate::schema::items;
    use crate::schema::lists;
    let connection = db::establish_connection();

    let id = id.into_inner();

    let list_name: String = lists::table
        .select(lists::name)
        .find(id)
        .first(&connection)
        .map_err(|e| ErrorBadRequest(e))?;

    let list_items: Vec<Item> = items::table
        .filter(items::list_id.eq_all(id))
        .load(&connection)
        .map_err(|e| ErrorBadRequest(e))?;

    let list_items_response = list_items
        .into_iter()
        .map(|i| ListItemResponse {
            id: i.id,
            value: i.data,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(ListWithItemsResponse {
        name: list_name,
        items: list_items_response,
    }))
}

#[post("/lists/{id}/add")]
async fn add_list_item(
    id: web::Path<i32>,
    req: web::Json<AddListItemRequest>,
) -> Result<HttpResponse, Error> {
    use crate::schema::items;

    let list_id = id.into_inner();
    let connection = db::establish_connection();

    insert_into(items::table)
        .values(&NewItem {
            list_id,
            data: req.value.as_str(),
        })
        .execute(&connection)
        .map_err(|e| ErrorBadRequest(e))?;

    Ok(HttpResponse::Ok().json(req.into_inner()))
}

#[delete("/lists/item/{id}/delete")]
async fn remove_list_item(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    use crate::schema::items;

    let item_id = id.into_inner();

    let connection = db::establish_connection();

    diesel::delete(items::table.find(item_id))
        .execute(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(("id", format!("{}", item_id))))
}
