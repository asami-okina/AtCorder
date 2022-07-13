use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     er
    //     "
    // );

    input! {
        // from source,
        s: String
    };
    let s_chars: Vec<char> = s.chars().collect();

    let length: usize = s.len();
    // 最後から2番目の文字取得
    let second_from_last = s_chars[length - 2];
    // 最後から1番目の文字取得
    let last = s_chars[length - 1];

    // er
    let er = format!("{}{}", second_from_last, last);
    if er == "er".to_string() {
        println!("er");
    }

    // ist
    // 最後から3番目の文字取得
    if length > 2 {
        let third_from_last = s_chars[length - 3];
        let ist = format!("{}{}{}", third_from_last, second_from_last, last);

        if ist == "ist".to_string() {
            println!("ist");
        }
    }
}
