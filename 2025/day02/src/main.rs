fn main() {
    let input = include_str!("inputs/question.txt");
    let answer = solve(input, None, None);
    println!("Answer: {}", answer);
}

fn solve(input: &str, min_repeat: Option<usize>, max_repeat: Option<usize>) -> i64 {
    let mut invalid_ints = Vec::new();
    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let (start, end) = range.split_once("-").unwrap();
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        invalid_ints.extend(get_invalid_range(start, end, min_repeat, max_repeat));
    }
    invalid_ints.iter().sum()
}

fn get_invalid_range(
    start: i64,
    end: i64,
    min_repeat: Option<usize>,
    max_repeat: Option<usize>,
) -> Vec<i64> {
    let mut invalid_ints = Vec::new();
    for n in start..=end {
        let s = n.to_string();
        let len = s.len();
        
        // Default: min_repeat = 2, max_repeat = len (number of digits)
        let min_k = min_repeat.unwrap_or(2);
        let max_k = max_repeat.unwrap_or(len);
        
        for k in min_k..=max_k {
            if len % k == 0 {
                let chunk_size = len / k;
                let chunk = &s[0..chunk_size];
                if s.as_bytes().chunks(chunk_size).all(|c| c == chunk.as_bytes()) {
                    invalid_ints.push(n);
                    break;
                }
            }
        }
    }
    invalid_ints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(solve(input, Some(2), Some(2)), 1227775554);
    }

    #[test]
    fn test_question() {
        let input = include_str!("inputs/question.txt");
        assert_eq!(solve(input, Some(2), Some(2)), 22062284697);
    }

    #[test]
    fn test_range_11_22() {
        // 11-22 has two invalid IDs, 11 and 22.
        assert_eq!(get_invalid_range(11, 22, Some(2), Some(2)), vec![11, 22]);
    }

    #[test]
    fn test_range_95_115() {
        // 95-115 has one invalid ID, 99.
        assert_eq!(get_invalid_range(95, 115, Some(2), Some(2)), vec![99]);
    }

    #[test]
    fn test_range_998_1012() {
        // 998-1012 has one invalid ID, 1010.
        assert_eq!(get_invalid_range(998, 1012, Some(2), Some(2)), vec![1010]);
    }

    #[test]
    fn test_range_1188511880_1188511890() {
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        assert_eq!(get_invalid_range(1188511880, 1188511890, Some(2), Some(2)), vec![1188511885]);
    }

    #[test]
    fn test_range_222220_222224() {
        // 222220-222224 has one invalid ID, 222222.
        assert_eq!(get_invalid_range(222220, 222224, Some(2), Some(2)), vec![222222]);
    }

    #[test]
    fn test_range_1698522_1698528() {
        // 1698522-1698528 contains no invalid IDs.
        assert_eq!(get_invalid_range(1698522, 1698528, Some(2), Some(2)), Vec::<i64>::new());
    }

    #[test]
    fn test_range_446443_446449() {
        // 446443-446449 has one invalid ID, 446446.
        assert_eq!(get_invalid_range(446443, 446449, Some(2), Some(2)), vec![446446]);
    }

    #[test]
    fn test_range_38593856_38593862() {
        // 38593856-38593862 has one invalid ID, 38593859.
        assert_eq!(get_invalid_range(38593856, 38593862, Some(2), Some(2)), vec![38593859]);
    }

    #[test]
    fn test_range_11_22_part2() {
        // 11-22 still has two invalid IDs, 11 and 22.
        assert_eq!(get_invalid_range(11, 22, None, None), vec![11, 22]);
    }

    #[test]
    fn test_range_95_115_part2() {
        // 95-115 now has two invalid IDs, 99 and 111.
        assert_eq!(get_invalid_range(95, 115, None, None), vec![99, 111]);
    }

    #[test]
    fn test_range_998_1012_part2() {
        // 998-1012 now has two invalid IDs, 999 and 1010.
        assert_eq!(get_invalid_range(998, 1012, None, None), vec![999, 1010]);
    }

    #[test]
    fn test_range_1188511880_1188511890_part2() {
        // 1188511880-1188511890 still has one invalid ID, 1188511885.
        assert_eq!(get_invalid_range(1188511880, 1188511890, None, None), vec![1188511885]);
    }

    #[test]
    fn test_range_222220_222224_part2() {
        // 222220-222224 still has one invalid ID, 222222.
        assert_eq!(get_invalid_range(222220, 222224, None, None), vec![222222]);
    }

    #[test]
    fn test_range_1698522_1698528_part2() {
        // 1698522-1698528 still contains no invalid IDs.
        assert_eq!(get_invalid_range(1698522, 1698528, None, None), Vec::<i64>::new());
    }

    #[test]
    fn test_range_446443_446449_part2() {
        // 446443-446449 still has one invalid ID, 446446.
        assert_eq!(get_invalid_range(446443, 446449, None, None), vec![446446]);
    }

    #[test]
    fn test_range_38593856_38593862_part2() {
        // 38593856-38593862 still has one invalid ID, 38593859.
        assert_eq!(get_invalid_range(38593856, 38593862, None, None), vec![38593859]);
    }

    #[test]
    fn test_range_565653_565659_part2() {
        // 565653-565659 now has one invalid ID, 565656.
        assert_eq!(get_invalid_range(565653, 565659, None, None), vec![565656]);
    }

    #[test]
    fn test_range_824824821_824824827_part2() {
        // 824824821-824824827 now has one invalid ID, 824824824.
        assert_eq!(get_invalid_range(824824821, 824824827, None, None), vec![824824824]);
    }

    #[test]
    fn test_range_2121212118_2121212124_part2() {
        // 2121212118-2121212124 now has one invalid ID, 2121212121.
        assert_eq!(get_invalid_range(2121212118, 2121212124, None, None), vec![2121212121]);
    }
}
