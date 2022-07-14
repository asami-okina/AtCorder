use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     12.5
    //     "
    // );

    input! {
        // from source,
        s: String
    };

    // String to &str
    let s_to_str: &str = &s;

    // Stringから対象文字を取得
    let vec: Vec<&str> = s_to_str.split('.').collect();

    let x:i32 = vec[0].parse().unwrap();
    let y:i32 = vec[1].parse().unwrap();

    if 0 <= y && y <= 2 {
        println!("{}{}", x, '-');
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}{}", x, '+');
    }

}