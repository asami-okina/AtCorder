
use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashMap;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     0 0 0 0 0 0 0 0 0 0
    //     "
    // );

    input! {
        // from source,
        list: [usize;10]
    };
    
    let zero = 0;
    let first = list[zero];
    let second = list[first];
    let result = list[second];
    println!("{}", result);
}
