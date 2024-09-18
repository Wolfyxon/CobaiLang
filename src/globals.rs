/*use crate::runtime::{FunctionContext, Value};

pub fn print<'a>(ctx: FunctionContext) -> Value<'a> {
    let text = &ctx.get_argument("text").unwrap();
    
    match text {
        Value::String(s) => {
            println!("{}", s);
        }
        
        _ => todo!("Only strings are supported for now")
    }

    Value::Null
}*/