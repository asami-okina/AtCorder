use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     97
    //     "
    // );

    input! {
        // from source,
        n: u8
    };

    println!("{}", n as char);
}
