use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashMap;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     2 5 2
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };

    if a == b {
        println!("{}", c);
        return;
    }
    if a == c {
        println!("{}", b);
        return;
    }
    if b == c {
        println!("{}", a);
        return;
    }
    println!("0");
}
