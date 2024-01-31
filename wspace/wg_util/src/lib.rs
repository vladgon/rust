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
    fn tap_ok<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_ok_ignore_result<V, F: FnOnce(&T) -> std::result::Result<V, E>>(self, op: F) -> Self;
    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Self;
    fn tap_err_ignore_result<EE: Into<E>, F: FnOnce(&E) -> std::result::Result<T, EE>>(self, op: F) -> Self;
}

impl<T, E> Tap<T, E> for std::result::Result<T, E> {
    fn tap_ok<F: FnOnce(&T)>(self, op: F) -> Self {
        if let Ok(t) = self {
            op(&t);
            Ok(t)
        } else { self }
    }

    fn tap_ok_ignore_result<V, F: FnOnce(&T) -> std::result::Result<V, E>>(self, op: F) -> Self {
        self.map(|t| op(&t).map(|_| t))?
    }


    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Self {
        if let Err(e) = &self {
            op(e);
            self
        } else { self }
    }

    fn tap_err_ignore_result<EE: Into<E>, F: FnOnce(&E) -> std::result::Result<T, EE>>(self, op: F) -> Self {
        if let Err(e) = &self {
            if let Err(ee) = op(e) { Err(ee.into()) } else { self }
        } else { self }
    }
}