use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    let source = AutoSource::from(
        "
        10
        "
    );

    input! {
        from source,
        n: i64,
    };

    if -2i64.pow(31) <= n && n < 2i64.pow(31){
        println!("Yes");
    } else {
        println!("No");
    }
}