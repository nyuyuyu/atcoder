use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    const KEYWORDS: [&str; 4] = ["dream", "dreamer", "erase", "eraser"];

    let mut answer = "NO";
    'outer: loop {
        let s_clone = s.clone();
        for i in KEYWORDS {
            if s == i {
                answer = "YES";
                break 'outer;
            }

            for j in KEYWORDS {
                let index = i.len() + j.len();
                if s_clone.len() < index {
                    continue;
                }

                let slice = &s_clone[..index];
                if slice == String::from(i) + j {
                    s = String::from(&s_clone[(i.len())..]);
                }
                if slice == String::from(j) + i {
                    s = String::from(&s_clone[(j.len())..]);
                }

                if s != s_clone {
                    continue 'outer;
                }
            }
        }

        if s == s_clone {
            break 'outer;
        }
    }

    println!("{}", answer);
}
