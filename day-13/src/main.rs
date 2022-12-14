use std::{fs, cmp::Ordering};
use serde::Deserialize;
use serde_json::{Value};

#[derive(Debug, PartialEq, Deserialize)]
enum Data {
    List(Vec<Data>),
    Value(u32),
}

#[derive(Debug, PartialEq)]
struct Packets {
    left: Value,
    right: Value,
}

impl From<&str> for Packets {
    fn from(input: &str) -> Self {
        let split = input.split("\n").collect::<Vec<&str>>();
        Packets { left: serde_json::from_str(split[0]).unwrap(), right: serde_json::from_str(split[1]).unwrap() }
    }
}

impl Packets {
    fn is_in_right_order(&self) -> bool {
        in_right_order(&self.left, &self.right).unwrap_or(true)
    }
}

fn in_right_order(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if l.as_u64() == r.as_u64() {
                None
            } else {
                Some(l.as_u64() <= r.as_u64())
            }
        },
        (Value::Array(array_left), Value::Array(array_right)) => {
            for (l, r) in array_left.iter().zip(array_right.iter()) {
                let result = in_right_order(l, r);
                if result.is_some() {
                    return result;
                }
            }
            if array_left.len() == array_right.len() {
                None
            } else {
                Some(array_left.len() < array_right.len())
            }
            
        },
        (l, r) if l.is_u64() && r.is_array() => {
            let array_left = Value::Array(vec![l.clone()]);
            in_right_order(&array_left, r)
        },
        (l, r) if l.is_array() && r.is_u64() => {
            let array_right = Value::Array(vec![r.clone()]);
            in_right_order(l, &array_right)
        }
        _ => unreachable!(),
    }
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match in_right_order(a, b) {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    }
}

fn main() {
    let file_name = "input/day-13";
    let file_content = fs::read_to_string(file_name).expect("oops");

    let packets = file_content.split("\n\n").map(|input| Packets::from(input)).collect::<Vec<Packets>>();
    let sum_in_order = packets.iter().enumerate().filter(|(_, packet)| packet.is_in_right_order()).map(|(index, _)| {println!("{}", index + 1);index + 1}).sum::<usize>();
    println!("Sum of indices in order: {}", sum_in_order);

    let divider_2: Value = serde_json::from_str("[[2]]").unwrap();
    let divider_6: Value = serde_json::from_str("[[6]]").unwrap();
    let mut all_packets: Vec<Value> = vec![divider_2.clone(), divider_6.clone()];
    for packet in packets {
        all_packets.push(packet.left);
        all_packets.push(packet.right);
    }
    all_packets.sort_by(|a,b| compare(a, b));
    println!("{:?}", all_packets);

    let position_2 = all_packets.iter().position(|packet| *packet == divider_2).unwrap() + 1;
    let position_6 = all_packets.iter().position(|packet| *packet == divider_6).unwrap() + 1;
    println!("Positions: {} {}", position_2, position_6);
    println!("Product of positions: {}", position_2 * position_6);
}

#[cfg(test)]
mod tests {
    use crate::Packets;

    #[test]
    fn compare_simple_arrays_in_order() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]";
        let packets = Packets::from(input);

        assert!(packets.is_in_right_order());
    }
    
    #[test]
    fn compare_simple_arrays_not_in_order() {
        let input = "[1,1,5,1,1]\n[1,1,3,1,1]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }
    
    #[test]
    fn compare_nested_arrays_in_order() {
        let input = "[[1],[2,3,4]]\n[[1],4]";
        let packets = Packets::from(input);

        assert!(packets.is_in_right_order());
    }
    
    #[test]
    fn compare_nested_arrays_not_in_order() {
        let input = "[[1],4]\n[[1],[2,3,4]]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }

    #[test]
    fn compare_nested_arrays_array_size_matters_not_in_order() {
        let input = "[9]\n[[8,7,6]]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }

    #[test]
    fn compare_nested_arrays_array_size_matters_in_order() {
        let input = "[[8,7,6]]\n[9]";
        let packets = Packets::from(input);

        assert!(packets.is_in_right_order());
    }

    #[test]
    fn compare_with_one_empty_array_in_order() {
        let input = "[]\n[3]";
        let packets = Packets::from(input);

        assert!(packets.is_in_right_order());
    }

    #[test]
    fn compare_nested_empty_arrays_not_in_order() {
        let input = "[[[]]]\n[[]]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }

    #[test]
    fn compare_big_arrays_not_in_order() {
        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }

    #[test]
    fn compare_arrays_of_sevens_not_in_order() {
        let input = "[7,7,7,7]\n[7,7,7]";
        let packets = Packets::from(input);

        assert!(!packets.is_in_right_order());
    }
}