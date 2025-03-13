#[test]
fn main() {
    let triangle_count = 10;
    tree(triangle_count);
}

fn tree(triangle_count: usize) {
    let mut width = 1;
    let max_width = triangle_count * 2;

    for _ in 0..triangle_count {
        for i in 0..(width / 2 + 1) {
            let star = "*".repeat(i * 2 + 1);
            let space = " ".repeat((max_width - star.len()) / 2);
            println!("{}{}", space, star);
        }
        width += 2;
    }
}