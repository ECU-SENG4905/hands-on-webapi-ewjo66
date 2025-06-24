use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{Status, NewStatus};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct StatusInput {
    pub status_name: String,
}

#[get("/statuses")]
pub async fn get_statuses(pool: &State<DbPool>) -> Json<Vec<Status>> {
    let mut conn = pool.get().expect("db connection");
    let statuses = Status::read_all(&mut conn).unwrap_or_default();
    Json(statuses)
}

#[get("/statuses/<id>")]
pub async fn get_status(id: i32, pool: &State<DbPool>) -> Option<Json<Status>> {
    let mut conn = pool.get().ok()?;
    Status::read(&mut conn, id).ok().map(Json)
}

#[post("/statuses", data = "<status>")]
pub async fn create_status(pool: &State<DbPool>, status: Json<StatusInput>) -> Option<Json<Status>> {
    let mut conn = pool.get().ok()?;
    let new_status = NewStatus {
        status_name: &status.status_name,
    };
    Status::create(&mut conn, new_status).ok().map(Json)
}

#[put("/statuses/<id>", data = "<status>")]
pub async fn update_status(id: i32, pool: &State<DbPool>, status: Json<StatusInput>) -> Option<Json<Status>> {
    let mut conn = pool.get().ok()?;
    let updated_status = NewStatus {
        status_name: &status.status_name,
    };
    Status::update(&mut conn, id, updated_status).ok().map(Json)
}

#[delete("/statuses/<id>")]
pub async fn delete_status(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    Status::delete(&mut conn, id).ok().map(Json)
}