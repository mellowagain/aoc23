fn main() {
    let input = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut sum = 0;

    for (y, &line) in input.iter().enumerate() {
        let mut buffer = String::with_capacity(3);
        let mut valid = false;

        for (x, char) in line.char_indices() {
            if !char.is_digit(10)  {
                if valid && !buffer.is_empty() {
                    let number: i32 = buffer.parse().unwrap();
                    sum += number;
                }

                println!("\"{buffer}\" valid? {valid}");
                valid = false;
                buffer.clear();
                continue
            }

            buffer.push(char);

            let above = (x, y.wrapping_sub(1));
            valid |= valid_marker_at(&input, above);
            println!("{char} above: {valid}");

            let right = (x + 1, y);
            valid |= valid_marker_at(&input, right);
            println!("{char} right: {valid}");

            let right_corner_above = (x + 1, y.wrapping_sub(1));
            valid |= valid_marker_at(&input, right_corner_above);
            println!("{char} right corner above: {valid}");

            let right_corner_below = (x + 1, y + 1);
            valid |= valid_marker_at(&input, right_corner_below);
            println!("{char} right corner below: {valid}");

            let below = (x, y + 1);
            valid |= valid_marker_at(&input, below);
            println!("{char} below: {valid}");

            let left = (x.wrapping_sub(1), y);
            valid |= valid_marker_at(&input, left);

            println!("{char} left: {valid}");

            let left_corner_above = (x.wrapping_sub(1), y.wrapping_sub(1));
            valid |= valid_marker_at(&input, left_corner_above);
            println!("{char} left corner above: {valid}");

            let left_corner_below = (x.wrapping_sub(1), y + 1);
            valid |= valid_marker_at(&input, left_corner_below);
            println!("{char} left corner below: {valid}");


        }
    }


    println!("part a: {sum}"); // 552835 too low

}

fn valid_marker_at(lines: &[&str], (x, y): (usize, usize)) -> bool {
    if let Some(a) = lines.get(y).and_then(|&line| line.chars().nth(x)) {
        !a.is_digit(10) && a != '.'
    } else {
        false
    }
}
