use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     60
    //     ",
    // );
    input!(
        // from source,
        n: i64
    );

    let result: String;

    if n < 60 {
        // 60未満
        result = String::from("21:");
    } else {
        result = String::from("22:");
    }

    // 分数の長さチェック
    if n.to_string().len() == 1 {
        // 分数が1桁の場合、0埋めする
        let parse_minutes = format!("{}{}", "0", &n.to_string());
        println!("{}{}", result, parse_minutes);
    } else {
        // 分数が2桁以上の場合
        let minutes: i64;
        if n >= 60 {
            // 分数が60分以上の場合
            // 60分を超えた場合は、60分引いた数を分数とする
            minutes = n - 60;
            // 分数が1桁の場合、0埋めする
            if minutes.to_string().len() == 1 {
                let minutes = format!("{}{}", "0", &minutes.to_string());
                println!("{}{}", result, minutes);
            } else {
                // 分数が2桁以上の場合
                println!("{}{}", result, minutes);
            }
        } else {
            // 分数が60分以下の場合
            minutes = n;
            println!("{}{}", result, minutes);
        }
    }

    // ローカルでは問題ないが、CEとなる
    // let dt1: DateTime<Local> = Local.datetime_from_str("2018/12/07 21:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
    // let dt2 = dt1 + Duration::minutes(n);
    // let dt3 = dt2.format("%H:%M").to_string();
    // println!("{}",dt3);
}
