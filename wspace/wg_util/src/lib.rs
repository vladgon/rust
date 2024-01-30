pub mod common;

pub type StdErrorBox = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, StdErrorBox>;

pub trait ResultExt<T> {
    fn into_std_error(self) -> Result<T>;
}

impl<T, E: Into<StdErrorBox>> ResultExt<T> for std::result::Result<T, E> {
    fn into_std_error(self) -> Result<T> { self.map_err(|e| e.into()) }
}

pub trait Tap<T, E> {
    fn do_on_ok<F: FnOnce(&T)>(self, op: F) -> Self;
    fn do_on_ok_ignore_result<V, F: FnOnce(&T) -> std::result::Result<V, E>>(self, op: F) -> Self;
    fn do_on_err<F: FnOnce(&E)>(self, op: F) -> Self;
    fn do_on_err_ignore_result<EE, F: FnOnce(&E) -> std::result::Result<T, EE>>(self, op: F) -> Self;
}

impl<T, E> Tap<T, E> for std::result::Result<T, E> {
    fn do_on_ok<F: FnOnce(&T)>(self, op: F) -> Self {
        match self {
            Ok(t) => {
                op(&t);
                Ok(t)
            }
            Err(e) => Err(e),
        }
    }

    fn do_on_ok_ignore_result<V, F: FnOnce(&T) -> std::result::Result<V, E>>(self, op: F) -> Self {
        match self {
            Ok(t) => {
                let res = op(&t);
                match res {
                    Ok(_) => Ok(t),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }


    fn do_on_err<F: FnOnce(&E)>(self, op: F) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                op(&e);
                Err(e)
            }
        }
    }

    fn do_on_err_ignore_result<EE, F: FnOnce(&E) -> std::result::Result<T, EE>>(self, op: F) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                _ = op(&e);
                Err(e)
            }
        }
    }
}