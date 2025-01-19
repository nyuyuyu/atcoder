use proconio::input;

fn main() {
    input! {
        x: String
    }

    let v: Vec<&str> = x.split('.').collect();

    let mut answer = String::from("");
    answer.push_str(v[0]);

    let mut flag = false;
    let mut decimal = String::from("");
    for i in v[1].chars().rev() {
        if i == '0' && !flag {
            continue;
        } else {
            flag = true;
        }
        decimal.push(i);
    }

    if decimal.len() > 0 {
        answer.push('.');
        for i in decimal.chars().rev() {
            answer.push(i);
        }
    }

    println!("{}", answer);
}
