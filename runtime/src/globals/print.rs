use rlua::{Context, Variadic};

fn print(args: Variadic<String>) -> Result<(), rlua::Error> {
    let args = args.to_vec();

    for (i, arg) in args.iter().enumerate() {
        print!("{}", arg);

        if i != args.len() - 1 {
            print!(" ");
        }
    }

    println!();

    Ok(())
}

pub fn set(ctx: Context) {
    let globals = ctx.globals();

    let func = ctx.create_function(|_, args: Variadic<String>| return print(args));

    globals.set("print", func.unwrap()).unwrap();
}
