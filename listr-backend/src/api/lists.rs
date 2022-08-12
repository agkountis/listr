use crate::db;
use crate::db::models::{Item, List, NewItem, NewList};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{delete, get, post, web, Error, HttpResponse, HttpRequest};
use diesel::{insert_into, delete, EqAll, QueryDsl, RunQueryDsl};
use listr_common::{AddListItemRequest, CreateListRequest, extract_user_from_bearer_token, ListItemResponse, ListWithItemsResponse};

#[get("/lists")]
pub async fn get_lists(request: HttpRequest) -> Result<HttpResponse, Error> {
    use crate::db::schema::lists;
    let connection = db::establish_connection();

    // Safe to unwrap here. The token verifier middleware has already validated the existence of this
    // header key/value.
    let token = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let user = extract_user_from_bearer_token(token);

    let res_lists: Vec<List> = lists::table.filter(lists::user_id.eq_all(user.sub))
        .load(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(res_lists))
}

#[get("/lists/{id}")]
pub async fn get_items(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    use crate::db::schema::items;
    use crate::db::schema::lists;
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

#[post("/lists/create")]
async fn create_list(request: HttpRequest, payload: web::Json<CreateListRequest>) -> Result<HttpResponse, Error> {
    use crate::db::schema::lists;

    let connection = db::establish_connection();

    // Safe to unwrap here. The token verifier middleware has already validated the existence of this
    // header key/value.
    let token = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let user = extract_user_from_bearer_token(token);
    let list_name = &payload.name;

    insert_into(lists::table)
        .values(&NewList {
            name: list_name,
            user_id: &user.sub,
        })
        .execute(&connection)
        .map_err(|e| ErrorBadRequest(e))?;

    Ok(HttpResponse::Ok().json(payload.into_inner()))
}

#[delete("/lists/delete/{id}")]
async fn delete_list(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    use crate::db::schema::lists;
    use crate::db::schema::items;

    let list_id = id.into_inner();
    let connection = db::establish_connection();

    //Delete all items that belong to the list with the specified id.
    delete(items::table.filter(items::list_id.eq_all(list_id)))
        .execute(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    //Delete the list with the specified id.
    delete(lists::table.filter(lists::id.eq_all(list_id)))
        .execute(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(("id", format!("{}", list_id))))
}

#[post("/lists/{id}/add")]
async fn add_list_item(
    id: web::Path<i32>,
    req: web::Json<AddListItemRequest>,
) -> Result<HttpResponse, Error> {
    use crate::db::schema::items;

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
    use crate::db::schema::items;

    let item_id = id.into_inner();

    let connection = db::establish_connection();

    diesel::delete(items::table.find(item_id))
        .execute(&connection)
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(("id", format!("{}", item_id))))
}
