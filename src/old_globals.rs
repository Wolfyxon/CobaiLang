use crate::interpreter::{Function, FunctionContext, Value};

pub fn get_functions() -> Vec<Function> {
    vec![
        Function::new_internal("print".to_string(), Vec::new(), print)
    ]
}

pub fn print(ctx: FunctionContext) {
    for arg in ctx.args {
        let mut arg_str = String::new();

        match arg {
            Value::String(s) => arg_str = s,
            Value::Number(n) => arg_str = n.to_string(),

            Value::Null => arg_str = "null".to_string(),
            _ => arg_str = "<unprintable>".to_string()
        }

        print!("{} ", arg_str);
    }
}