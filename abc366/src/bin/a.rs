use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }

    if t.abs_diff(a) > (n - t - a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
