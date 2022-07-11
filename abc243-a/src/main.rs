

use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     100000 1 1 1
    //     "
    // );

    input! {
        // from source,
        mut v: i32,
        a: i32,
        b: i32,
        c: i32
    };


    loop {
        if v - a >= 0 {
           v = v - a;
        }  else {
            println!("F");
            break;
        }

        if v - b >= 0 {
           v = v - b;
        } else {
            println!("M");
            break;
        }
        
        if v - c >= 0{
            v = v - c;
        } else {
            println!("T");
            break;
        }
    }

    // println!("{}",person);
}
