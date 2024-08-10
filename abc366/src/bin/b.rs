use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut max_length = 0;
    for i in &s {
        if max_length < i.len() {
            max_length = i.len();
        }
    }

    for i in 0..max_length {
        let mut line = String::from("");
        for j in (0..n).rev() {
            let mut c = s[j].chars().nth(i).unwrap_or(' ');
            if c == ' ' {
                for k in (0..j).rev() {
                    if s[k].chars().nth(i).unwrap_or(' ') != ' ' {
                        c = '*';
                        break;
                    }
                }
            }
            line.push(c);
        }
        println!("{}", line);
    }
}
