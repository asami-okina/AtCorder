use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     0
    //     "
    // );

    input! {
        // from source,
        x: i32,
    };

    if x >= 100 && x % 100 == 0{
        println!("Yes");
    } else {
        println!("No");
    }
} 