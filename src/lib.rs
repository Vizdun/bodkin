mod eval;
mod parser;

pub use crate::eval::BodkinCtx as Bodkin;
pub use crate::eval::BodkinEvalError;
pub use bodkin_macro::bodkin_fn;

#[macro_export]
macro_rules! bodkin_args {
    ( $( $x:expr ),* ) => {
        &[$(Rc::new($x)),*]
    };
}
