use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3 4 5
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };
    println!("{}",if a.pow(2) + b.pow(2) < c.pow(2) {"Yes"} else {"No"});
}