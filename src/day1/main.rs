fn main() {
    let input = include_str!("input.txt");

    let a = compute(input);
    let b = compute(&replace(input));

    println!("part a: {a}");
    println!("part b: {b}");
}

fn compute(input: &str) -> i32 {
    input.lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect())
        .map(|line: String| (line.chars().next().unwrap(), line.chars().last().unwrap()))
        .map(|(first, last)| {
            let mut str = first.to_string();
            str.push(last);
            str
        })
        .map(|a| a.parse::<i32>().unwrap())
        .sum()
}

fn replace(input: &str) -> String {
    // very dirty hack
    input.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}
