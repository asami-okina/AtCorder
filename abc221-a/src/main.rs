use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     5 5
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
    };
    
    if a == b {
        println!("1");
    } else {
        let diff = (a-b).abs() as u32;
        println!("{}", 32i32.pow(diff));
    }
}