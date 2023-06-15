use rlua::Context;

mod print;
pub mod prompts;

pub fn set(ctx: Context) {
    print::set(ctx);
    prompts::set(ctx);
}
