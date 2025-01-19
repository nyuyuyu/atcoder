use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    const REPUNIT_DIGITS: usize = 12;

    let mut v = Vec::new();
    for i in 1..=REPUNIT_DIGITS {
        for j in 1..=REPUNIT_DIGITS {
            for k in 1..=REPUNIT_DIGITS {
                let mut sum = 0;
                for x in [i, j, k] {
                    sum += "1".repeat(x).parse::<u64>().unwrap();
                }

                v.push(sum);
            }
        }
    }

    v.sort();
    v.dedup();

    println!("{}", v[n - 1]);
}
