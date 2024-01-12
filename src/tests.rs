use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS_LOCK_MUTEX: Mutex<Executor> = Mutex::new(Executor);
}

#[derive(Clone, Copy)]
pub struct Executor;

impl Executor {
    pub fn run(self, f: fn() -> Result<(), String>) -> Result<(), String> {
        return f();
    }
}
