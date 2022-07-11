use proconio::source::auto::AutoSource;
use proconio::{input, marker::Chars};

fn main() {
    // let source = AutoSource::from(
    //     "
    //     999
    //     ",
    // );

    input! {
        // from source,
        list: Chars
    };

    let abc: i32 = format!("{}{}{}", list[0], list[1], list[2])
        .parse()
        .unwrap();
    let bca: i32 = format!("{}{}{}", list[1], list[2], list[0])
        .parse()
        .unwrap();
    let cab: i32 = format!("{}{}{}", list[2], list[0], list[1])
        .parse()
        .unwrap();
    println!("{}", abc + bca + cab);
}
