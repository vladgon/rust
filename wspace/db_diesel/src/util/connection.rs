use std::fmt::Debug;

use diesel::prelude::*;

use wg_util::common::config::app_config;

///
/// Establish Connection
///
/// # Examples
///
/// ```
/// use diesel::connection::SimpleConnection;
/// use db_diesel::util::connection::establish_connection;
/// assert!(establish_connection().unwrap().batch_execute("Select 1").is_ok(), "Cannot get Connection")
///```
///
///```
/// use wg_util::common::config::app_config;
/// assert!(app_config::settings?.db.url, "db_url")
/// ```

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    tracing::trace!("Got Connection {:?}", dotenv::var("DATABASE_URL").unwrap());
    app_config::settings().unwrap().db.url.establish_connection()
}

pub trait MySqlConnectionT {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection>;
}

impl MySqlConnectionT for String {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection> {
        MysqlConnection::establish(self)
    }
}

impl<E: Debug> MySqlConnectionT for Result<String, E> {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection> {
        MysqlConnection::establish(self.as_ref()
            .map_err(|e| ConnectionError::InvalidConnectionUrl(format!("{:?}", e)))?)
    }
}