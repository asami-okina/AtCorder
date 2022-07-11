
use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     100
    //     "
    // );

    input! {
        // from source,
        k: i32
    };

    let hour: i32 = (k / 60) + 21;
    let minute: i32 = k % 60;
    println!("{:02}:{:02}", hour, minute);
}
