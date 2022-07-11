use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     1
    //     ",
    // );

    input! {
        // from source,
        n: i32,
    };
    let count: String;

    if n >= 42 {
        count = format!("{:03}", n + 1);
    } else {
        count = format!("{:03}", n);
    }
    println!("{}", format!("{}{}", "AGC", count));
}
