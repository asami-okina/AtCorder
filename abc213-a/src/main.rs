use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     10 12
    //     "
    // );

    input! {
        // from source,
        a: usize,
        b: usize,
    };

    // ビット演算OR: どちらかが1なら1、どちらも0の場合は0を返す
    for c in 0..=255 {
        if a ^ c == b {
            println!("{}",c);
            return;
        }
    }
}