use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn bodkin_fn(input: TokenStream) -> TokenStream {
    let ast: syn::ExprClosure = syn::parse(input.clone()).unwrap();

    let number_of_args = ast.inputs.len();

    let args = 0..number_of_args;

    let expanded = quote! {
        |n, span| {
            use std::borrow::Borrow;

            if n.len() == #number_of_args {
                Ok(
                    Rc::new(
                        (#ast)
                        (
                            #(
                                n[#args].clone().downcast().or(Err(bodkin::BodkinEvalError::TypeErr(span.clone())))?
                            ),*
                        )
                    )
                )
            } else {
                Err(bodkin::BodkinEvalError::IncorrectNumberOfArgs(span))
            }
        }
    };

    TokenStream::from(expanded)
}