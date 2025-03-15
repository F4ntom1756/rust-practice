#[test]
fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [0, 8, -8, 1, 2, 10, -1, -2, -10];

    for n in shifts.iter() {
        let rotated = rotate(s.clone(), *n);
        println!("{} передвинуто на {} = {}", s, n, rotated);
    }
}

fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len() as isize;
    let shift = (n % len + len) % len;

    let (left, right) = s.split_at(len as usize - shift as usize);
    format!("{}{}", right, left)
}

fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.clone(), *n), exp.to_string())
    });
}