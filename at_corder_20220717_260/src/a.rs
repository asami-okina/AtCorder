use std::collections::HashMap;

use proconio::input;
use proconio::source::auto::AutoSource;
use proconio::marker::{Bytes, Chars};
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     xxx
    //     "
    // );

    input! {
        // from source,
        s: Chars
    };

    let mut list: HashMap<char, i32> = std::collections::HashMap::new();

    for i in s {
        *list.entry(i).or_insert(0) += 1;
    }
    
    for (key, value) in list {
        if value == 1 {
            println!("{}", key);
            return;
        }
    }
    println!("-1");

}