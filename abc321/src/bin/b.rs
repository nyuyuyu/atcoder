use proconio::input;

fn main() {
    input! {
        nx: (i32, i32),
        a: [i32; nx.0 - 1],
    }

    let x = nx.1;

    let mut answer = -1;
    for i in 0..=100 {
        let mut aa = a.clone();
        aa.push(i);

        let score: i32 = aa.iter().sum();
        let score = score - (aa.iter().min().unwrap() + aa.iter().max().unwrap());
        if score >= x {
            answer = i;
            break;
        }
    }

    println!("{}", answer);
}
