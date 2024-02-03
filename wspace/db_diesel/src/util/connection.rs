use std::error::Error;

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
/// use wg_util::common::config::rust_app;
/// /// use wg_util::common::config::rust_app;
/// use wg_util::common::config::rust_app::Options;
/// rust_app::init(Options::Default).unwrap();
/// assert!(establish_connection().unwrap().batch_execute("Select 1").is_ok(), "Cannot get Connection");
///
///```
///
///```
/// use db_diesel::util::connection::db_url;
/// use wg_util::common::config::rust_app;
/// use wg_util::common::config::log::LogConfig;
/// use wg_util::common::config::rust_app::Options;
/// rust_app::init(Options::Default).unwrap();
/// assert_eq!(db_url().is_ok(), true)
/// ```

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    db_url()
        .tap(|url| debug!("Connecting {:?}", url))
        .establish_connection()
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


pub trait MySqlConnectionExt {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection>;
}

impl MySqlConnectionExt for String {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection> {
        MysqlConnection::establish(self)
    }
}

impl<E: Error> MySqlConnectionExt for Result<String, E> {
    fn establish_connection(&self) -> ConnectionResult<MysqlConnection> {
        self.as_ref().map(|s| s.establish_connection())
            .map_err(|e| ConnectionError::InvalidConnectionUrl(format!("{:?}", e)))?
    }
}