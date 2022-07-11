use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     4 5
    //     ",
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
    };

    if (a - b).abs() == 1 {
        println!("Yes");
    } else if (a - b).abs() == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
