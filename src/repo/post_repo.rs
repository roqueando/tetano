use models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expec("Error saving new post")
}

