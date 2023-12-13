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

                valid = false;
                buffer.clear();
                continue
            }

            buffer.push(char);

            let above = (x, y.wrapping_sub(1));
            valid |= valid_marker_at(&input, above);

            let right = (x + 1, y);
            valid |= valid_marker_at(&input, right);

            let right_corner_above = (x + 1, y.wrapping_sub(1));
            valid |= valid_marker_at(&input, right_corner_above);

            let right_corner_below = (x + 1, y + 1);
            valid |= valid_marker_at(&input, right_corner_below);

            let below = (x, y + 1);
            valid |= valid_marker_at(&input, below);

            let left = (x.wrapping_sub(1), y);
            valid |= valid_marker_at(&input, left);

            let left_corner_above = (x.wrapping_sub(1), y.wrapping_sub(1));
            valid |= valid_marker_at(&input, left_corner_above);

            let left_corner_below = (x.wrapping_sub(1), y + 1);
            valid |= valid_marker_at(&input, left_corner_below);
        }

        // in case there is a trailing buffer
        if valid && !buffer.is_empty() {
            let number: i32 = buffer.parse().unwrap();
            sum += number;
        }
    }

    println!("part a: {sum}");
}

fn valid_marker_at(lines: &[&str], (x, y): (usize, usize)) -> bool {
    if let Some(a) = lines.get(y).and_then(|&line| line.chars().nth(x)) {
        !a.is_digit(10) && a != '.'
    } else {
        false
    }
}
