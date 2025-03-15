#[test]
fn main() {
    let numbers = [123, 121, 1221];

    for number in numbers.iter() {
        println!("Число {} є паліндромом: {}", number, is_palindrome(*number));
    }
}

fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
}