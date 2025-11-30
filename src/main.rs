use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end_matches(&['\r', '\n'][..]).to_string();

    let mut t: Vec<char> = Vec::with_capacity(input.len() * 2 + 3);
    t.push('^');
    t.push('#');
    for ch in input.chars() {
        t.push(ch);
        t.push('#');
    }
    t.push('$');

    let n = t.len();
    let mut p = vec![0usize; n];
    let mut center = 0usize;
    let mut right = 0usize;
    let mut best_center = 0usize;
    let mut best_len = 0usize;

    for i in 1..n - 1 {
        if i < right {
            let mirror = 2 * center - i;
            p[i] = p[mirror].min(right - i);
        }
        while i + 1 + p[i] < n
            && i as isize - 1 - p[i] as isize >= 0
            && t[i + 1 + p[i]] == t[i - 1 - p[i]]
        {
            p[i] += 1;
        }
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
        if p[i] > best_len {
            best_len = p[i];
            best_center = i;
        }
    }

    let start_in_t = best_center.saturating_sub(best_len);
    let start = (start_in_t.saturating_sub(1)) / 2;
    let res: String = input.chars().skip(start).take(best_len).collect();
    println!("{}", res);
}
