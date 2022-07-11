
use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     1 2 1 1000
    //     "
    // );

    input! {
        // from source,
        a: f64,
        b: f64,
        c: f64,
        x: f64
    };

    // いろはちゃんの点数が上位A位以内の場合
    if x <= a {
        let result:f64 = 1.0;
        println!("{:.12}", result);
        return;
    }

    // いろはちゃんの点数が上位A+1位からB位までの参加者に該当している場合、その中からC名ランダムに選ばれる
    if x >= a + 1.0 && x <= b {
        let random = c / (b - a);
        println!("{:.12}",random);
        return;
    } else {
        // 該当しない場合
        let result:f64 = 0.0;
        println!("{:.12}", result);
        return;
    }


}
