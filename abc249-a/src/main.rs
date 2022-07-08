use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     4 3 3 6 2 5 103
    //     "
    // );

    input! {
        // from source,
        a: i32, // 高橋さん何秒歩いたか
        b: i32, // 高橋さん秒速何メートルか
        c: i32, // 高橋さん何秒休んだか
        d: i32, // 青木さん何秒歩いたか
        e: i32, // 青木さん秒速何メートルか
        f: i32, // 青木さん何秒休んだか
        x: i32 // 何秒後に再開するか
    };

    // 例
    // [前提]
    // a = 4, b = 3, c = 3
    // d = 6, e = 2, f = 5
    // x = 103
    // [高橋くんの場合]
    // 103:x / (4:a + 3:c) = ①14 ②... 5
    // ①14 * 4:a = 56
    // ②5 > 4 {② = 4:a}
    // 56 + 4 = 60
    // 60 * b (道のり*速さ=距離)←結果

    // 高橋くん
    // xの中のトータル秒数(歩く+休む)
    let takahashi_one_set_count: i32 = x / (a + c);
    // 残りの時間のうち、歩く時間より残りが大きければ、歩くmaxの距離を指定
    let mut takahashi_count_over: i32 = x % (a + c);
    if takahashi_count_over > a {

        takahashi_count_over = a;
    }
    // トータルの時間
    let takahashi_distance = takahashi_one_set_count * a + takahashi_count_over;
    // トータルの距離(時間*速さ)
    let takahashi_result = takahashi_distance * b;

    // 青木くん
    let aoki_one_set_count: i32 = x / (d + f);
    let mut aoki_count_over: i32 = x % (d + f);
    if aoki_count_over > d {

        aoki_count_over = d;
    }
    let aoki_distance = aoki_one_set_count * d + aoki_count_over;
    let aoki_result = aoki_distance * e;

    if takahashi_result > aoki_result {
        println!("Takahashi");
    } else if takahashi_result < aoki_result {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
