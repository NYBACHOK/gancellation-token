use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::vec::Vec;

use crate::token::CancellationToken;
use crate::CANCEL_PANIC_MSG;

/// Source of all tokens
#[derive(Debug)]
pub struct CancellationSource {
    flag: Arc<AtomicBool>,
    childs: Vec<CancellationToken>,
}

impl Drop for CancellationSource {
    fn drop(&mut self) {
        self.cancel_childs();
    }
}

impl Default for CancellationSource {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationSource {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
            childs: Vec::new(),
        }
    }

    pub fn token(&mut self) -> CancellationToken {
        let token = CancellationToken {
            flag: Arc::new(AtomicBool::new(false)),
            source: Some(Arc::clone(&self.flag)),
        };
        self.childs.push(token.clone());

        token
    }

    pub fn cancel(&mut self) {
        self.flag.store(true, Ordering::SeqCst);
        self.cancel_childs();
    }

    pub fn cancel_childs(&mut self) {
        for child in &mut self.childs {
            child.cancel();
        }
    }

    #[inline]
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
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
