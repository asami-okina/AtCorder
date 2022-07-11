

use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     270 750
    //     "
    // );

    input! {
        // from source,
        mut x: i32,
        y: i32
    };
    
    let mut count:i32 = 0;

    loop {
        if x >= y {
            println!("{}",count);
            break;
        }

        x = x + 10;
        count += 1;
    }
}
