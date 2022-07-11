use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3141
    //     "
    // );

    input! {
        // from source,
        d: f32,
    };
    println!("{}", d / 100.0);
}