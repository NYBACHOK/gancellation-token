use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Debug)]
pub struct TokenDropGuard {
    pub(crate) other_flag: Arc<AtomicBool>,
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
            self.other_flag.store(true, Ordering::SeqCst);
        }
    }
}
