fn main() {
    let input = include_str!("inputs/question.txt");
    let answer = solve(input, "0x434C49434B");
    println!("Answer: {}", answer);
}

fn solve(input: &str, password_method: &str) -> i32 {
    let instructions: Vec<&str> = input.lines().collect();
    
    let mut current_dial = 50;
    let mut answer = 0;
    for instruction in instructions {
        // Instructions are in the format of "R<distance>" or "L<distance>"
        // where <distance> is a positive integer from 0-99
        // Example: "R50" or "L23"
        
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();
        let mut passed_0_count;
        match direction {
            'R' => {
                let new_dial = current_dial + distance;
                current_dial = new_dial % 100;

                passed_0_count = new_dial / 100;

                if current_dial == 0 {
                    passed_0_count -= 1;
                }
            },
            'L' => {
                let new_dial = current_dial - distance;
                if new_dial < 0 {
                    passed_0_count = ((new_dial / 100) * -1) + 1;

                    if current_dial == 0 {
                        passed_0_count -= 1;
                    }

                    if new_dial % 100 == 0 {
                       passed_0_count -= 1; 
                    }
                } else {
                    passed_0_count = 0;
                }

                current_dial = new_dial.rem_euclid(100); 
            },
            _ => {
                panic!("Invalid direction");
            },
        }

        println!("Instruction: {}, Current dial: {}, Extra count: {}", instruction, current_dial, passed_0_count);
        if current_dial == 0 {
            answer += 1;
        }

        if password_method == "0x434C49434B" && passed_0_count > 0 {
            answer += passed_0_count;
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
        assert_eq!(solve(input, "default"), 3);
    }

    #[test]
    fn test_question() {
        let input = include_str!("inputs/question.txt");
        assert_eq!(solve(input, "default"), 1018);
    }

    #[test]
    fn test_sample_part2() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(solve(input, "0x434C49434B"), 6);
    }

    #[test]
    fn test_left_100() {
        // start dial is x, single turns
        let input = "L100\n";
        assert_eq!(solve(input, "0x434C49434B"), 1)
    }

    #[test]
    fn test_left_500() {
        // start dial is x, multiple turns
        let input = "L500\n";
        assert_eq!(solve(input, "0x434C49434B"), 5) 
    }

    #[test]
    fn test_left_250() {
        // start dial is x, and end dial is 0
        let input = "L250\n";
        assert_eq!(solve(input, "0x434C49434B"), 3) 
    }

    #[test]
    fn test_right_left_200() {
        // start dial is 0, and end dial is 0
        let input = "R50\nL200\n";
        assert_eq!(solve(input, "0x434C49434B"), 3) 
    }

    #[test]
    fn test_right_300() {
        // start dial is x, multiple turns
        let input = "R300";
        assert_eq!(solve(input, "0x434C49434B"), 3) 
    }

    #[test]
    fn test_right_350() {
        // start dial is x, and end dial is 0
        let input = "R350";
        assert_eq!(solve(input, "0x434C49434B"), 4) 
    }

    #[test]
    fn test_left_right_300() {
        // start dial is 0, and end dial is 0
        let input = "L50\nR300";
        assert_eq!(solve(input, "0x434C49434B"), 4) 
    }

    #[test]
    fn test_question_part2() {
        let input = include_str!("inputs/question.txt");
        assert_eq!(solve(input, "0x434C49434B"), 5815);
    }
}

