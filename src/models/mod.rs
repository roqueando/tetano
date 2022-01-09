use crate::schema::*;
use diesel::Insertable;

pub mod post;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
