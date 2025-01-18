use proconio::input;

fn main() {
    input! {
        mut x: usize,
    }

    let mut answer = 0;
    for i in 2..usize::MAX {
        if f(i) == x {
            answer = i;
            break;
        }
    }

    println!("{}", answer);
}

fn f(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        f(n - 1) * n
    }
}
