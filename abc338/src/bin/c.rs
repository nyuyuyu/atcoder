use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [i32; n],
        a: [i32; n],
        b: [i32; n],
    }

    let mut answer = 0;

    let mut x = 0;
    loop {
        let mut r = vec![0; n];
        for (i, ai) in a.iter().enumerate() {
            r[i] = q[i] - ai * x;
        }
        if r.iter().find(|&&s| s < 0) != None {
            break;
        }

        let mut candidate = vec![1001001];
        for (i, bi) in b.iter().enumerate() {
            if *bi == 0 {
                continue;
            }
            candidate.push(r[i] / bi);
        }

        let y = candidate.iter().min().unwrap();
        answer = *[answer, x + y].iter().max().unwrap();

        x += 1;
    }

    println!("{}", answer);
}
