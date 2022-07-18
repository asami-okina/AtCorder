use proconio::input;
use proconio::{marker::Chars, *};
use proconio::source::auto::AutoSource;

fn main() {
    let source = AutoSource::from(
        "
        2
        23
        45      
        ",
    );

    input! {
        from source,
        n: usize,
        list:[Chars; n]
    };

    let mut result = String::new();

    for x in 0..n {
        for y in 0..n {
            for di in -1i8..=1 {
                for dj in -1i8..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let mut i = x as i8;
                    let mut j = y as i8;
                    let mut cur = String::new();

                    for _ in 0..n {
                        i += di;
                        j += dj;

                        // 一番上に来たら、一番下に移動
                        if i < 0 {
                            i = (n - 1) as i8;
                        }
                        
                        if j < 0 {
                            j = (n-1) as i8;
                        }

                        // 一番下にきたら、一番上に移動
                        if i >= n as i8 {
                            i = 0;
                        }

                        if j >= n as i8 {
                            j = 0;
                        }
                        cur.push(list[i as usize][j as usize]);
                        if result < cur {
                            result = cur;
                        }
                    }
                }
            }
        }
    }
    println!("{}", result);
}
