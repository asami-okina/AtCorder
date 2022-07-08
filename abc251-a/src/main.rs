use proconio::input;
// use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     vv
    //     "
    // );

    input! {
        // from source,
        s: String
    };

    let max_length: i32 = 6;
    let input_length: i32 = s.len() as i32;
    let repeat_time = max_length / input_length;

    let new_string: String = s.repeat(repeat_time as usize);
    println!("{}", new_string);
}
