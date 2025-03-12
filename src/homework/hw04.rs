#[test]
fn main() {
    const H: u32 = 6;
    const W: u32 = 2 * H - 1;

    for y in 0..(2 * H - 1) {
        let abs_y = if y < H { y } else { 2 * H - 2 - y };
        let star = 2 * abs_y + 1;
        let space = (W - star) / 2;

        for x in 0..W {
            let sym = if x >= space && x < space + star { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}