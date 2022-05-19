use std::rc::Rc;

use bodkin::{bodkin_args, bodkin_fn, Bodkin};

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

pub fn main() {
    let mut bdkn = Bodkin::new();

    bdkn.run(
        r#"
name = newName "John" "Smith"

nameFmt first last = concat last ", " first
"#,
    )
    .unwrap();

    bdkn.add_extern(
        "newName",
        bodkin_fn!(|first: Rc<String>, last: Rc<String>| Name {
            first: first.to_string(),
            last: last.to_string()
        }),
    );

    bdkn.add_extern("concat", |v, span| {
        let mut s = String::new();

        for v in v {
            s.push_str(
                &v.downcast::<String>()
                    .or(Err(bodkin::BodkinEvalError::TypeErr(span.clone())))?,
            )
        }

        Ok(Rc::new(s))
    });

    println!("{:#?}", bdkn.eval::<Name>("name", &vec![]));

    println!(
        "{:#?}",
        bdkn.eval::<String>(
            "nameFmt",
            bodkin_args![String::from("Jeff"), String::from("Bezos")]
        )
    );
}
