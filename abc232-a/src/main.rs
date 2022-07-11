use proconio::{input, marker::Chars};
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3x7
    //     "
    // );

    input! {
        // from source,
        s: Chars
    };

    let a: i32 = s[0] as i32 - 48;
    let b: i32 = s[2] as i32 - 48;
    println!("{}", a * b);

}