use std::vec::Vec;

use crate::token::CancellationToken;
use crate::CANCEL_PANIC_MSG;

/// Source of all tokens
#[derive(Debug)]
pub struct CancellationSource {
    flag: bool,
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
            flag: false,
            childs: Vec::new(),
        }
    }

    pub fn token(&mut self) -> CancellationToken {
        let token = CancellationToken::new();
        self.childs.push(token.clone());

        token
    }

    pub fn cancel(&mut self) {
        self.flag = true;
        self.cancel_childs();
    }

    pub fn cancel_childs(&mut self) {
        for child in &mut self.childs {
            child.cancel();
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.flag
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
