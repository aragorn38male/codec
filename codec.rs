fn encode(entry: String) -> String {
    let mut result = "".to_string();
    let mut n = 1;
    let mut p = 0;
    while p < entry.chars().count() {
        while entry[0..entry.len()].chars().nth(p) == entry[0..entry.len()].chars().nth(p + 1) {
            n += 1;
            p += 1;
        }
        result += &n.to_string();
        result += if &entry[p..p + 1] == "0" { "z" } else { "o" };
        p += 1;
        n = 1;
    }
    result
}

fn decode(entry: String) -> String {
    let mut result = "".to_string();
    let mut d = 0;
    let mut u = 0;
    for i in 0..entry.len() {
        let c = entry[i..i + 1].chars().nth(0).unwrap();
        if c == 'o' {
            let n = &entry[d..u];
            let n: i32 = n.parse().unwrap();
            for _n in 0..n {
                result += "1";
            }
            u += 1;
            d = u;
        } else if c == 'z' {
            let n = &entry[d..u];
            let n: i32 = n.parse().unwrap();
            for _n in 0..n {
                result += "0";
            }
            u += 1;
            d = u;
        } else {
            u += 1;
        }
    }
    result
}

fn main() {
    let entry = "0110001111000000100000000000001".to_string();
    println!("{}", entry);
    let encoded = encode(entry);
    println!("{encoded}");
    let decoded = decode(encoded);
    println!("{decoded}");
}
