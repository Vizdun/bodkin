use std::rc::Rc;

use bodkin::{bodkin_fn, Bodkin};

pub fn main() {
    let mut bdkn = Bodkin::new();

    bdkn.run(
        r#"
mulTwo x = mul x 2.5

math = add x y
  x = mul y z
  y = -5
  z = mulTwo 75
"#,
    )
    .unwrap();

    bdkn.add_extern("add", bodkin_fn!(|a: Rc<f64>, b: Rc<f64>| *a + *b));

    bdkn.add_extern("mul", bodkin_fn!(|a: Rc<f64>, b: Rc<f64>| *a * *b));

    println!("{:#?}", bdkn.eval::<f64>("math", &vec![]));
}
