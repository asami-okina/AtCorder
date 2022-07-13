use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     321
    //     "
    // );

    input! {
        // from source,
        s: usize
    };
    println!("{:04}", s);
    
}