use std::rc::Rc;

use bodkin::Bodkin;

pub fn main() {
    let mut bdkn = Bodkin::new();

    bdkn.run(r#"rec x = rec x"#).unwrap();

    println!("{:#?}", bdkn.eval::<f64>("rec", &[Rc::new(0.0)]));
}
