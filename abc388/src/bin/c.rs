use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;
    for i in 0..n {
        count += n - a.partition_point(|&x| x < a[i] * 2);
    }

    println!("{}", count);
}
