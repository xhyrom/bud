use rlua::{Lua, Result};

pub mod globals;
mod preprocessor;

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    pub fn new() -> Self {
        let lua = Lua::new();

        lua.context(|lua_ctx| globals::set(lua_ctx));

        Runtime { lua }
    }

    pub fn run(self, script: &str) -> Result<()> {
        let script = preprocessor::preprocess(script);

        self.lua.context(|lua_ctx| {
            lua_ctx.load(&script).exec()?;

            Ok(())
        })
    }
}
