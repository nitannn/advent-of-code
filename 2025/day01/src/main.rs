fn main() {
    let input = include_str!("inputs/sample.txt");
    let answer = solve(input);
    println!("Answer: {}", answer);
}

fn solve(input: &str) -> i32 {
    let instructions: Vec<&str> = input.lines().collect();
    
    let mut current_dial = 50;
    let mut answer = 0;
    for instruction in instructions {
        // Instructions are in the format of "R<distance>" or "L<distance>"
        // where <distance> is a positive integer from 0-99
        // Example: "R50" or "L23"
        
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();
        
        match direction {
            'R' => {
                let new_dial = current_dial + distance;
                current_dial = new_dial % 100;
            },
            'L' => {
                let new_dial = current_dial - distance;
                current_dial = (100 + new_dial) % 100;
            },
            _ => {
                panic!("Invalid direction");
            },
        }

        println!("Current dial: {}", current_dial);
        if current_dial == 0 {
            answer += 1;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("inputs/part1.txt");
        assert_eq!(solve(input), 1018);
    }
}

