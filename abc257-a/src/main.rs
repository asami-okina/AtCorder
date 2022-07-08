use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     2 12
    //     ",
    // );

    input!(
        // from source,
        n: usize,
        x: usize
    );

    let alphabet_list = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>();

    let mut new_string: String = String::new();

    for i in alphabet_list {
        let repeat_char = &i.to_string().repeat(n);
        new_string.push_str(&repeat_char);
    }

    match new_string.chars().nth(x - 1) {
        Some(v) => {
            println!("{}", v);
        }
        None => {}
    }
}
