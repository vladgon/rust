extern crate db_diesel as db;

#[test]
fn db_integration() {}

#[test]
fn insert_post() {
    println!("Inserted {:?}", db::create_post(&db::establish_connection(), "vlad", "Post body"));
}
