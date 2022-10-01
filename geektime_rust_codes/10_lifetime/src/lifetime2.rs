fn main() {
    let s1 = String::from("Lindsey");
    let result;
    {
        let s2 = String::from("Rosie");
        // s2 生命周期不够长
        result = max(&s1, &s2);
    }

    println!("bigger one: {}", result);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // s1 和 s2 有相同的生命周期 'a ，所以它满足 (s1: &'a str, s2: &'a str) 的约束
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
