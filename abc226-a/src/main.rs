use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3.456
    //     "
    // );

    input! {
        // from source,
        x: f64,
    };
    println!("{}", x.round());
}