use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;
use rlua::Context;

mod get_prompt;

lazy_static! {
    static ref PROMPTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn get_prompts() -> MutexGuard<'static, Vec<String>> {
    let prompts = PROMPTS.lock().unwrap();

    return prompts;
}

pub fn set(ctx: Context) {
    get_prompt::set(ctx);
}
