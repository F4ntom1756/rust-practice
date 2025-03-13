fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temporary = b;
        b = a % b;
        a = temporary;
    }
    a
}

#[test]
fn main() {
    let data = [
        ((24, 60), 12),
        ((15, 9), 3),
        ((15, 6), 3),
        ((140, 40), 20),
        ((24, 16), 8),
        ((100, 10), 10),
        ((120, 80), 40),
        ((80, 120), 40),
        ((100, 20), 20),
        ((37, 11), 1),
        ((120, 90), 30),
    ];
    for i in 0..data.len() {
        let (a, b) = data[i].0;
        let expected = data[i].1;
        let result = gcd(a, b);

        println!("gcd({}, {}) = {} (Очікувано: {})", a, b, result, expected);
    }
}
