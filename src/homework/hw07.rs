#[test]
fn main() {
    let input = "Hello World! Привіт Світ!";
    let inverted = invert_the_case(input.to_string());
    println!("Оригінальні слова: {}", input);
    println!("Інвертовані слова: {}", inverted);
}

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string().chars().next().unwrap()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string().chars().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привіт", "пРИВІТ"),
    ];

    for (a, b) in data.iter() {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    }
}