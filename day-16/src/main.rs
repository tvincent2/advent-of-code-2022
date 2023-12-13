use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;
use set_partitions::{ArrayVecSetPartition, Subsets};

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    name: String,
    adjacent_valves: Vec<String>,
    flow_rate: u32,
    opened: bool,
    opened_at: u32,
}

impl Valve {
    fn can_be_opened(&self) -> bool {
        self.flow_rate > 0 && !self.opened
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Solution {
    remaining_time: u32,
    flow: u32,
    valves: HashMap<String, Valve>,
}

impl Solution {
    fn decrease_time(&mut self, time: u32) {
        self.remaining_time -= time;
    }

    fn open_valve(&mut self, valve_name: &str) {
        self.decrease_time(1);
        self.flow += self.valves[valve_name].flow_rate * self.remaining_time;
        self.valves
            .entry(valve_name.to_string())
            .and_modify(|valve| {valve.opened = true; valve.opened_at = self.remaining_time});
        // println!("\t\tOpening valve {valve_name}, it remains {} minutes, so the improvement is {} and flow is now {}", self.remaining_time, self.valves[valve_name].flow_rate * self.remaining_time, self.flow);
    }

    fn upper_bound(
        &self,
        current_valve: &str,
        distances: &HashMap<String, HashMap<String, u32>>,
    ) -> u32 {
        let mut upper_bound = self.flow;
        let openable_valves_flows = self
            .valves
            .iter()
            .filter(|(name, valve)| {
                distances[current_valve][*name] < self.remaining_time - 1 && valve.can_be_opened()
            })
            .map(|(name, valve)| (name.clone(), valve.flow_rate))
            .collect::<Vec<(String, u32)>>();

        for (name, flow) in openable_valves_flows {
            upper_bound += flow * (self.remaining_time - distances[current_valve][&name] - 1);
        }
        upper_bound
    }
}

fn resolve_rec(
    current_valve: String,
    partial_solution: Solution,
    best_known_solution: &Solution,
    visited_valves: Vec<String>,
    distances: &HashMap<String, HashMap<String, u32>>,
    initial_time: u32,
    relaunchable: bool,
) -> Option<Solution> {
    // println!("Resolve {} {:?} {} {}", relaunchable, visited_valves, partial_solution.remaining_time, partial_solution.flow);
    if partial_solution.remaining_time <= 1 {
        // println!("leaving in A");
        return None;
    } else {
        let upper_bound = partial_solution.upper_bound(&current_valve, distances);
        if !relaunchable && upper_bound < best_known_solution.flow {
            // println!("leaving in B, upper bound is {} and best known is {}", upper_bound, best_known_solution.flow);
            return None;
        }
        let mut best_solution = best_known_solution.clone();
        let mut new_solution = partial_solution.clone();
        if partial_solution.valves[&current_valve].can_be_opened() {
            new_solution.open_valve(&current_valve);
        }
        let solution_with_elephant = match relaunchable {
            false => None,
            true => {
                let mut new_partial_solution = new_solution.clone();
                new_partial_solution.remaining_time = initial_time;
                resolve_rec(
                "AA".to_string(),
                new_partial_solution,
                best_known_solution,
                visited_valves.clone(),
                distances,
                initial_time,
                false,
            )},
        };
        if let Some(ref solution) = solution_with_elephant {
            best_solution = solution.clone();
        }
        if partial_solution.remaining_time == 0
            || visited_valves.len() == partial_solution.valves.len()
        {
            match &solution_with_elephant {
                None => {
                    if new_solution.flow > best_known_solution.flow {
                        // println!("leaving in C");
                        return Some(new_solution);
                    } else {
                        // println!("leaving in D");
                        return None;
                    }
                }
                Some(sol) => {
                    if sol.flow > new_solution.flow && sol.flow > best_known_solution.flow {
                        // println!("leaving in E");
                        return Some(sol.clone());
                    } else if new_solution.flow > best_known_solution.flow {
                        // println!("leaving in F");
                        return Some(new_solution);
                    } else {
                        // println!("leaving in G");
                        return None;
                    }
                }
            }
        }
        let adjacent_valves = new_solution
            .valves
            .iter()
            .filter(|(name, valve)| {
                distances[&current_valve][*name] < new_solution.remaining_time
                    && valve.can_be_opened()
            })
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();
        // println!("\t{} {:?} valves to visit: {:?}", relaunchable, visited_valves, adjacent_valves);
        if adjacent_valves.is_empty() {
            if let Some(ref sol) = solution_with_elephant {
                if sol.flow > new_solution.flow {
                    return Some(sol.clone());
                }
            }
            return Some(new_solution);
        }
        for next_valve in adjacent_valves {
            let mut new_visited_valves2 = visited_valves.clone();
            new_visited_valves2.push(next_valve.to_owned());
            let mut new_solution2 = new_solution.clone();
            new_solution2.decrease_time(distances[&current_valve][&next_valve]);
            if let Some(solution) = resolve_rec(
                next_valve.clone(),
                new_solution2,
                &best_solution,
                new_visited_valves2,
                distances,
                initial_time,
                relaunchable,
            ) {
                if !relaunchable && solution.flow == upper_bound {
                    return Some(solution);
                } else if solution.flow > best_solution.flow {
                    best_solution = solution
                }
            }
        }
        // if best_solution.flow > 1707 {
        //     println!("Impossible best solution: {:?}", best_solution);
        //     panic!();
        // }
        Some(best_solution)
    }
}

fn resolve(
    valves: &HashMap<String, Valve>,
    distances: &HashMap<String, HashMap<String, u32>>,
) -> Solution {
    let empty_solution = Solution {
        remaining_time: 30,
        flow: 0,
        valves: valves.clone(),
    };
    let dummy_solution = Solution {
        remaining_time: 30,
        flow: 0,
        valves: HashMap::new(),
    };
    resolve_rec(
        "AA".to_string(),
        empty_solution,
        &dummy_solution,
        vec!["AA".to_string()],
        distances,
        30,
        false,
    )
    .unwrap()
}

fn resolve2(
    valves: &HashMap<String, Valve>,
    distances: &HashMap<String, HashMap<String, u32>>,
) -> Solution {
    let empty_solution = Solution {
        remaining_time: 26,
        flow: 0,
        valves: valves.clone(),
    };
    let dummy_solution = Solution {
        remaining_time: 26,
        flow: 0,
        valves: HashMap::new(),
    };
    resolve_rec(
        "AA".to_string(),
        empty_solution,
        &dummy_solution,
        vec!["AA".to_string()],
        distances,
        26,
        true,
    )
    .unwrap()
}

fn compute_distance(from: String, to: String, valves: &HashMap<String, Valve>) -> u32 {
    if from == to {
        return 0;
    }
    let mut visited = VecDeque::from([(from, 0)]);
    while !visited.is_empty() {
        let (current, distance) = visited.pop_front().unwrap();
        for neighbor in &valves[&current].adjacent_valves {
            if *neighbor == to {
                return distance + 1;
            } else {
                visited.push_back((neighbor.clone(), distance + 1));
            }
        }
    }
    unreachable!()
}

fn main() {
    let file_name = "input/day-16";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<valves>\w+[, \w]*)").unwrap();

    let valves: Vec<Valve> = reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let capture = re.captures(&l).unwrap();
            let valves = capture["valves"]
                .split(", ")
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            Valve {
                name: capture["name"].to_string(),
                adjacent_valves: valves,
                flow_rate: capture["flow_rate"].parse::<u32>().unwrap(),
                opened: false,
                opened_at: 0
            }
        })
        .collect();

    let mut hash_valves: HashMap<String, Valve> = HashMap::new();
    for valve in &valves {
        hash_valves.insert(valve.name.clone(), valve.clone());
    }

    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut keys_to_remove = vec![];
    for name in hash_valves.keys() {
        if hash_valves[name].can_be_opened() || name == "AA" {
            distances.insert(name.clone(), HashMap::new());
            for name2 in hash_valves.keys() {
                if hash_valves[name2].can_be_opened() || name2 == "AA" {
                    distances.entry(name.clone()).and_modify(|map| {
                        map.insert(
                            name2.clone(),
                            compute_distance(name.clone(), name2.clone(), &hash_valves),
                        );
                    });
                }
            }
        } else {
            keys_to_remove.push(name.clone());
        }
    }

    for key in keys_to_remove {
        if hash_valves[&key].flow_rate == 0 {
            hash_valves.remove(&key);
        }
    }

    // println!("{:#?}", distances);
    // println!("{:?}", hash_valves);

    let solution = resolve(&hash_valves, &distances);
    println!("{:?}", solution);

    let solution2 = resolve2(&hash_valves, &distances);
    println!("{:?}", solution2);
}
