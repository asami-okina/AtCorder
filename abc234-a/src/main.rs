

use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     10
    //     "
    // );

    input! {
        // from source,
        t: i32,
    };
    println!("{}", f(f(f(t)+t)+f(f(t))));
}

fn f(x: i32) -> i32 {
    x.pow(2) + 2 * x + 3
}