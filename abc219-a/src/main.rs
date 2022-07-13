use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     100
    //     ",
    // );

    input! {
        // from source,
        x: i32,
    };

    let mut grade: String = String::new();

    // 現在のランク計算
    if 0 <= x && x < 40 {
        grade = "elementary".to_string();
    } else if 40 <= x && x < 70 {
        grade = "middle".to_string();
    } else if 70 <= x && x < 90 {
        grade = "senior".to_string();
    } else {
        grade = "expert".to_string();
    }

    if grade == "expert".to_string() {
        println!("expert");
    } else {
        if grade == "elementary".to_string() {
            println!("{}", 40 - x);
        } else if grade == "middle".to_string() {
            println!("{}", 70 - x);
        } else if grade == "senior".to_string() {
            println!("{}", 90 - x);
        }
    }
}
