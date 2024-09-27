use std::marker::PhantomData;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::guard::TokenDropGuard;
use crate::CANCEL_PANIC_MSG;

#[derive(Debug, Clone)]
pub struct CancellationToken {
    flag: Arc<AtomicBool>,
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
        }
    }

    pub fn drop_guard(&self) -> TokenDropGuard {
        TokenDropGuard {
            other_flag: Arc::clone(&self.flag),
            flag: true,
            _marker: PhantomData,
        }
    }

    pub fn cancel(&mut self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }

    pub fn panic_if_cancelled(&self) {
        if self.is_cancelled() {
            panic!("{CANCEL_PANIC_MSG}")
        }
    }

    pub fn cancel_and_panic(&mut self) -> ! {
        self.cancel();
        self.panic_if_cancelled();
        unreachable!()
    }
}
