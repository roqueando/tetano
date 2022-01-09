use diesel::Queryable;

#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

