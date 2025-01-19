use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let mut ooo = String::new();
    for _ in 1..=n {
        ooo += "o";
    }

    println!("L{}ng", ooo);
}
