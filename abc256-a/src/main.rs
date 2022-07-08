use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3
    //     ",
    // );

    input!(
        // from source,
        n: u32
    );

    let base: i32 = 2;
    println!("{}", base.pow(n));
}
