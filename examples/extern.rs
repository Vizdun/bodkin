use std::rc::Rc;

use bodkin::{bodkin_fn, Bodkin};

pub fn main() {
    let mut bdkn = Bodkin::new();

    bdkn.run(r#"truth = true"#).unwrap();

    bdkn.add_extern("true", bodkin_fn!(|| true));

    println!("{:#?}", bdkn.eval::<bool>("truth", &vec![]));
}
