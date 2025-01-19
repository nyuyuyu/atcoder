use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [[i32; 2]; n],
    }

    let mut takahashi = 0;
    let mut aoki = 0;
    for i in &x {
        takahashi += i[0];
        aoki += i[1];
    }

    if takahashi > aoki {
        println!("Takahashi");
    }
    if aoki > takahashi {
        println!("Aoki");
    }
    if takahashi == aoki {
        println!("Draw");
    }
}
