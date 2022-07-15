use std::collections::HashSet;
use std::hash::Hash;

use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     99 99 98
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };

    let mut vec: Vec<i32> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{}", vec[1] + vec[2]);

}