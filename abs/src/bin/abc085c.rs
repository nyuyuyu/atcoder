use proconio::input;

fn main() {
    input! {
        ny: [i32; 2],
    }

    let n = ny[0];
    let y = ny[1];

    const YUKICHI_YEN: i32 = 10000;
    const INAZO_YEN: i32 = 5000;
    const SOSEKI_YEN: i32 = 1000;

    let mut yukichi_count = -1;
    let mut inazo_count = -1;
    let mut soseki_count = -1;

    for i in 0..=(n) {
        if (YUKICHI_YEN * i) > y {
            break;
        }

        for j in 0..=(n - i) {
            if (YUKICHI_YEN * i + INAZO_YEN * j) > y {
                break;
            }

            for k in 0..=(n - i - j) {
                if i + j + k != n {
                    continue;
                }

                if (YUKICHI_YEN * i + INAZO_YEN * j + SOSEKI_YEN * k) == y {
                    yukichi_count = i;
                    inazo_count = j;
                    soseki_count = k;
                    break;
                }
            }
        }
    }

    println!("{} {} {}", yukichi_count, inazo_count, soseki_count);
}
