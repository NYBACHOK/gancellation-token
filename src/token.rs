use std::marker::PhantomData;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::guard::TokenDropGuard;
use crate::CANCEL_PANIC_MSG;

#[derive(Debug, Clone)]
pub struct CancellationToken {
    pub(crate) flag: Arc<AtomicBool>,
    pub(crate) source: Option<Arc<AtomicBool>>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
            source: None,
        }
    }

    pub fn drop_guard(&self) -> TokenDropGuard {
        TokenDropGuard {
            main: self.clone(),
            cancel_source: false,
            flag: true,
            _marker: PhantomData,
        }
    }

    pub fn drop_guard_with_source(&self) -> TokenDropGuard {
        TokenDropGuard {
            main: self.clone(),
            cancel_source: false,
            flag: true,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn cancel(&mut self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn is_cancelled(&self) -> bool {
        match &self.source {
            Some(var) => match var.load(Ordering::Relaxed) {
                true => true,
                false => self.flag.load(Ordering::Relaxed),
            },
            None => self.flag.load(Ordering::Relaxed),
        }
    }

    #[inline]
    pub fn cancel_source(&self) -> bool {
        match &self.source {
            Some(var) => {
                var.store(true, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }

    #[inline]
    pub fn panic_if_cancelled(&self) {
        if self.is_cancelled() {
            panic!("{CANCEL_PANIC_MSG}")
        }
    }

    #[inline]
    pub fn cancel_and_panic(&mut self) -> ! {
        self.cancel();
        self.panic_if_cancelled();
        unreachable!()
    }
}
