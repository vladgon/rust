pub fn connect(user: &str, pwd: &str, port: u32) -> mysql::Result<mysql::Pool> {
    mysql::Pool::new(format!("mysql://{}:{}@localhost:{}/webgrid", user, pwd, port))
}
