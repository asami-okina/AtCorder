use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     5 6 4
    //     ",
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };

    let mut sum: i32 = 0;
    sum += 7 - a;
    sum += 7 - b;
    sum += 7 - c;
    println!("{}", sum);
}
