pub trait OptionExt<T> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_ignore_result<_T, F: FnOnce(&T) -> Option<_T>>(self, op: F) -> Self;
}


impl<T> OptionExt<T> for Option<T> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self {
        if let Some(t) = &self {
            op(t);
            self
        } else { self }
    }

    fn tap_ignore_result<_T, F: FnOnce(&T) -> Option<_T>>(self, op: F) -> Self {
        self.tap(|t| {
            _ = op(t);
        })
    }
}