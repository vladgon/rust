extern crate db_diesel as db;

#[cfg(test)]
mod test {
    #[test]
    fn db_integration() {}

    #[test]
    fn insert_post() {
        println!("Inserted {:?}", db::create_post(&db::establish_connection(), "vlad", "Post body"));
    }
}