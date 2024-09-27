use std::marker::PhantomData;
use std::rc::Rc;

use crate::CancellationToken;

#[derive(Debug)]
pub struct TokenDropGuard {
    pub(crate) main: CancellationToken,
    pub(crate) cancel_source: bool,
    pub(crate) flag: bool,
    /// I don't want that someone have ability to send token to other thread or save it to use somewhere else.
    /// So this marker prevents user from doing so by making type !Send & !Sync
    pub(crate) _marker: PhantomData<Rc<()>>,
}

impl TokenDropGuard {
    pub fn disarm(mut self) {
        self.flag = false;
    }
}

impl Drop for TokenDropGuard {
    fn drop(&mut self) {
        // Other way is to catch panic and cancel in such case,
        // but I think it slightly unclear but cleaner
        if self.flag {
            self.main.cancel();
            if self.cancel_source {
                self.main.cancel_source();
            }
        }
    }
}
