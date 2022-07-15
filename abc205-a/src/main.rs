use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     45 200
    //     "
    // );

    input! {
        // from source,
        a: f64,
        b: f64,
    };
    println!("{}", a * (b / 100.0));
}