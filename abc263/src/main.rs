use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashMap;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     10
    //     1 2 3 4 5 6 7 8 9         
    //     ",
    // );

    input! {
        // from source,
        n: usize,
        list: [usize; n - 1]
    };

    // c:list内で一番最後の要素
    let mut c = list[n-2];
    let mut count = 1;
    loop {
        if c == 1 {
            break;
        }
        // 1つ前の要素
        c = list[c-2];
        count += 1;
    }
    println!("{}",count);
}
