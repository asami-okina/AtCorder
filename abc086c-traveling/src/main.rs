use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    let source = AutoSource::from(
        "
        2
        3 1 2
        6 1 1
        "
    );

    input!(
        from source,
        max: i32,

    );
}
