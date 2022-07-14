use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     3 2
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
    };
    let mut count: i32 = 0;

    for i in a..=b {
        if i >= a && i <= b {
            count += 1;
        }
    }
    println!("{}", count);
}
