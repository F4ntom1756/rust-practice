#[test]
fn main() {
    let numbers_to_test = [0, 1, 2, 3, 4, 5, 100, 10007];
    for number in numbers_to_test.iter() {
        println!("Число {} є простим: {}", number, is_prime(number));
    }
}

fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= *n {
        if *n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data
        .iter()
        .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
}