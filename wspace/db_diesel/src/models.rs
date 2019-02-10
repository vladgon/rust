use super::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: u64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}