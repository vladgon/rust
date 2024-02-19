pub type StdErrorBox = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, StdErrorBox>;

pub trait ResultExt<T, E: Into<StdErrorBox>> {
    fn into_std_error(self) -> Result<T>;
}

impl<T, E: Into<StdErrorBox>> ResultExt<T, E> for std::result::Result<T, E> {
    fn into_std_error(self) -> Result<T> { self.map_err(|e| e.into()) }
}

pub trait ResultTap<T, E> {
    fn tap(self, op: impl FnOnce(&T)) -> Self;
    fn tap_ignore_result<TT, EE: Into<E>, F: FnOnce(&T) -> std::result::Result<TT, EE>>(self, op: F) -> Self;
    fn tap_err(self, op: impl FnOnce(&E)) -> Self;
    fn tap_err_ignore_result<TT, EE: Into<E>, F: FnOnce(&E) -> std::result::Result<TT, EE>>(self, op: F) -> Self;
}


impl<T, E> ResultTap<T, E> for std::result::Result<T, E> {
    fn tap(self, op: impl FnOnce(&T)) -> Self {
        if let Ok(t) = &self {
            op(t);
            self
        } else { self }
    }

    fn tap_ignore_result<TT, EE: Into<E>, F: FnOnce(&T) -> std::result::Result<TT, EE>>(self, op: F) -> Self {
        if let Ok(t) = &self {
            match op(t) {
                Ok(_) => self,
                Err(e) => { Err(e.into()) }
            }
        } else { self }
    }

    fn tap_err(self, op: impl FnOnce(&E)) -> Self {
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