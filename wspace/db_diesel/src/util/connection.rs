use diesel::prelude::*;

use crate::config;

///
/// Establish Connection
///
/// # Examples
///
/// use diesel::connection::SimpleConnection;
/// use db_diesel::util::connection::establish_connection;
/// assert!(establish_connection().batch_execute("Select 1").is_ok(), "Cannot get Connection")
///
///
/// ```
/// use db_diesel::config;
/// assert!(config::db_url().is_ok(), "Cannot get Connection")
/// ```

pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
    tracing::trace!("Got Connection {:?}", dotenv::var("DATABASE_URL").unwrap());
    let database_url = config::db_url().unwrap();
    MysqlConnection::establish(&database_url)
}
