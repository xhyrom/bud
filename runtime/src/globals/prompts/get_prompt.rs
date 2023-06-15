use rlua::{Context, String as LuaString};

use super::PROMPTS;

fn get_prompt(ctx: Context, arg: usize) -> Result<LuaString, rlua::Error> {
    let prompts = PROMPTS.lock().unwrap();
    let prompt = prompts.get(arg).unwrap();

    Ok(ctx.create_string(prompt)?)
}

pub fn set(ctx: Context) {
    let globals = ctx.globals();

    let func = ctx.create_function(|lua_ctx, args: usize| return get_prompt(lua_ctx, args));

    globals.set("get_prompt", func.unwrap()).unwrap();
}
