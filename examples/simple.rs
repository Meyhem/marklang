extern crate marklang;

use marklang::MarkovLanguageGenerator;

fn main() {
    let mut g = MarkovLanguageGenerator::new(2);
    g.fit_str("hello").unwrap();

    for _ in 0..5 {
        println!("{:?}", g.gen(5));
    }
}