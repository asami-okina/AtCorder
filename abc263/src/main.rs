use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashMap;

fn main() {
    let source = AutoSource::from(
        "
        10
        1 1 2 3
        ",
    );

    input! {
        from source,
        n: usize,
        list: [usize; n - 1]
    };

    // c:list内で一番最後の要素
    let mut c = list[n-2];
    let mut count = 1;
    loop {
        // N 4
        // (親:list)1 1 3の場合もある。
        // 人2の親が人1,人3の親が人1,人4の親が人3。人2と人3は兄弟。
        // 兄弟がいる場合もあるから、兄弟の場合は世代カウントしたくない。
        if c == 1 {
            break;
        }
        // 1つ前の要素
        c = list[c-2];
        count += 1;
    }
    println!("{}",count);
}
