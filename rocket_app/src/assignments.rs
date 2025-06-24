use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{Assignment, NewAssignment};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct AssignmentInput {
    pub user_id: i32,
    pub task_id: i32,
    pub task_status_id: i32,
}

#[get("/assignments")]
pub async fn get_assignments(pool: &State<DbPool>) -> Json<Vec<Assignment>> {
    let mut conn = pool.get().expect("db connection");
    let assignments = Assignment::read_all(&mut conn).unwrap_or_default();
    Json(assignments)
}

#[get("/assignments/<id>")]
pub async fn get_assignment(id: i32, pool: &State<DbPool>) -> Option<Json<Assignment>> {
    let mut conn = pool.get().ok()?;
    // For simplicity, treat id as user_id and get the first assignment for that user
    Assignment::read(&mut conn, id).ok().map(Json)
}

#[post("/assignments", data = "<assignment>")]
pub async fn create_assignment(pool: &State<DbPool>, assignment: Json<AssignmentInput>) -> Option<Json<Assignment>> {
    let mut conn = pool.get().ok()?;    let new_assignment = NewAssignment {
        user_id: assignment.user_id,
        task_id: assignment.task_id,
        task_status_id: assignment.task_status_id,
    };
    Assignment::create(&mut conn, new_assignment).ok().map(Json)
}

#[put("/assignments/<id>", data = "<assignment>")]
pub async fn update_assignment(id: i32, pool: &State<DbPool>, assignment: Json<AssignmentInput>) -> Option<Json<Assignment>> {
    let mut conn = pool.get().ok()?;
    let updated_assignment = NewAssignment {
        user_id: assignment.user_id,
        task_id: assignment.task_id,
        task_status_id: assignment.task_status_id,
    };
    Assignment::update(&mut conn, id, updated_assignment).ok().map(Json)
}

#[delete("/assignments/<id>")]
pub async fn delete_assignment(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    Assignment::delete(&mut conn, id).ok().map(Json)
}
