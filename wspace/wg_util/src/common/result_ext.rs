use std::fmt::Debug;

use crate::StdErrorBox;

pub type Result<T> = std::result::Result<T, StdErrorBox>;

pub trait ResultExt<T> {
    fn into_std_error(self) -> Result<T>;
}

impl<T, E: Into<StdErrorBox>> ResultExt<T> for std::result::Result<T, E> {
    fn into_std_error(self) -> Result<T> { self.map_err(|e| e.into()) }
}

pub trait ResultTap<T, E> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_ignore_result<TT, EE: Debug, F: FnOnce(&T) -> std::result::Result<TT, EE>>(self, op: F) -> Self;
    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Self;
    fn tap_err_ignore_result<TT, EE: Into<E>, F: FnOnce(&E) -> std::result::Result<TT, EE>>(self, op: F) -> Self;
}


impl<T, E> ResultTap<T, E> for std::result::Result<T, E> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self {
        if let Ok(t) = &self {
            op(t);
            self
        } else { self }
    }

    fn tap_ignore_result<TT, EE: Debug, F: FnOnce(&T) -> std::result::Result<TT, EE>>(self, op: F) -> Self {
        self.tap(|t| {
            op(t).unwrap_or_else(|ee| panic!("{:?}", ee));
        })
    }


    fn tap_err<F: FnOnce(&E)>(self, op: F) -> Self {
        if let Err(e) = &self {
            op(e);
            self
        } else { self }
    }

    fn tap_err_ignore_result<TT, EE: Into<E>, F: FnOnce(&E) -> std::result::Result<TT, EE>>(self, op: F) -> Self {
        if let Err(e) = &self {
            if let Err(ee) = op(e) { Err(ee.into()) } else { self }
        } else { self }
    }
}