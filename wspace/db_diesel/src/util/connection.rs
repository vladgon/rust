use std::fmt::Debug;

use diesel::prelude::*;
use log::debug;

use wg_util::common::config::app_config;
use wg_util::ResultTap;

///
/// Establish Connection
///
/// # Examples
///
/// ```
/// use diesel::connection::SimpleConnection;
/// use db_diesel::util::connection::establish_connection;
/// assert!(establish_connection()?.batch_execute("Select 1").is_ok(), "Cannot get Connection")
///```
///
///```
/// use db_diesel::util::connection::db_url;
/// use wg_util::common::config::app_config;
/// assert_eq!(db_url().is_ok(), true)
/// ```

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    db_url()
        .tap_ok(|url| debug!("Connecting {:?}", url))
        .map(|s| s.establish_connection())?
}

pub fn db_url() -> ConnectionResult<String> {
    app_config::settings()
        .map(|s|
            s.db.url
             .replace("${user}", s.db.user.as_str())
             .replace("${password}", s.db.password.as_ref()
                                      .unwrap_or(&"".into())))
        .map_err(|e| ConnectionError::InvalidConnectionUrl(e.to_string()))
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