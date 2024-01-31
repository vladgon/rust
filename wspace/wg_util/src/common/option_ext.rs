pub trait OptionTap<T> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self;
    fn tap_ignore_result<TT, F: FnOnce(&T) -> Option<TT>>(self, op: F) -> Self;
}


impl<T> OptionTap<T> for Option<T> {
    fn tap<F: FnOnce(&T)>(self, op: F) -> Self {
        if let Some(t) = &self {
            op(t);
            self
        } else { self }
    }

    fn tap_ignore_result<TT, F: FnOnce(&T) -> Option<TT>>(self, op: F) -> Self {
        self.tap(|t| {
            _ = op(t);
        })
    }
}