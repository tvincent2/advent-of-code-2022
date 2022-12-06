use std::collections::HashSet;

fn all_different(characters: &[char]) -> bool {
    let char_set = characters.iter().collect::<HashSet<&char>>();
    char_set.len() == characters.len()
}

fn find_start(signal: &str, number_of_distinct_chars: usize) -> usize {
    let char_vec = signal.chars().collect::<Vec<char>>();
    for (index, characters) in char_vec.windows(number_of_distinct_chars).enumerate() {
        if all_different(characters) {
            return index + number_of_distinct_chars
        }
    }
    0
}

fn find_signal_start(signal: &str) -> usize {
    find_start(signal, 4)
}

fn find_message_start(signal: &str) -> usize {
    find_start(signal, 14)
}

fn main() {
    let signal = include_str!("../../input/day-06");
    let signal_start = find_signal_start(signal);
    println!("Signal start: {}", signal_start);
    let message_start = find_message_start(signal);
    println!("Signal start: {}", message_start);
}

#[cfg(test)]
mod tests {
    use crate::{find_signal_start, find_message_start};

    #[test]
    fn signal_start() {
        assert_eq!(find_signal_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_signal_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_signal_start("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_signal_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_signal_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    
    #[test]
    fn message_start() {
        assert_eq!(find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message_start("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}