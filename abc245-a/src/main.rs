use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     7 30 7 30
    //     "
    // );

    input! {
        // from source,
        a: usize,
        b: usize,
        c: usize,
        d: usize
    };

    let takahashi_time = format!("{:02}:{:02}:{:02}", a, b, 0);
    let aoki_time = format!("{:02}:{:02}:{:02}", c, d, 1);

    if takahashi_time < aoki_time {
        println!("Takahashi");
    } else  {
        println!("Aoki");
    }
}
