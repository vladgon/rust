use std::fmt::Debug;

use diesel::prelude::*;

use crate::config;

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
/// ```
/// use db_diesel::config;
/// assert!(config::db_url().is_ok(), "db_url")
/// ```

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    tracing::trace!("Got Connection {:?}", dotenv::var("DATABASE_URL").unwrap());
    config::db_url().establish_connection()
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