use aoc_2022_rust::Puzzle;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day16 {
    dist: HashMap<(Valve, Valve), isize>,
    goal: HashSet<Valve>,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            dist: HashMap::new(),
            goal: HashSet::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.dist = HashMap::new();
        self.goal = HashSet::new();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Valve {
    name: (char, char),
    rate: isize,
}

type Map = HashMap<Valve, Vec<Valve>>;

fn parse_valve_name(input: &str) -> IResult<&str, (char, char)> {
    pair(anychar, anychar)(input)
}

fn parse_decimal(input: &str) -> IResult<&str, isize> {
    map(digit1, |s: &str| isize::from_str_radix(s, 10).unwrap())(input)
}

fn parse_valve_name_list(input: &str) -> IResult<&str, Vec<(char, char)>> {
    separated_list0(tag(", "), parse_valve_name)(input)
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

fn parse_line(input: &str) -> IResult<&str, (Valve, Vec<(char, char)>)> {
    let p = tuple((
        tag("Valve "),
        parse_valve_name,
        tag(" has flow rate="),
        parse_decimal,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        parse_valve_name_list,
    ));
    map(p, |(_, name, _, rate, _, list)| {
        (Valve { name, rate }, list)
    })(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State1 {
    valve: Valve,
    closed: HashSet<Valve>,
    gain: isize,
    remaining_steps: isize,
}

fn traverse_alone(dist: &HashMap<(Valve, Valve), isize>, state: State1, max_gain: &mut isize) {
    if state.remaining_steps <= 2 || state.closed.len() == 0 {
        *max_gain = (*max_gain).max(state.gain);
        return ();
    }
    for valve in &state.closed {
        if state.remaining_steps <= dist[&(state.valve, *valve)] + 1 {
            continue;
        }
        let mut new_closed = state.closed.clone();
        new_closed.remove(&valve);
        let new_remaining_steps = state.remaining_steps - dist[&(state.valve, *valve)] - 1;
        let newstate = State1 {
            valve: *valve,
            closed: new_closed,
            gain: state.gain + new_remaining_steps * valve.rate,
            remaining_steps: new_remaining_steps,
        };
        traverse_alone(&dist, newstate, max_gain);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State2 {
    me: Valve,
    elephant: Valve,
    closed: HashSet<Valve>,
    gain: isize,
    remaining_steps: isize,
    me_traveling: isize,
    elephant_traveling: isize,
}

fn traverse_with_elephant(
    dist: &HashMap<(Valve, Valve), isize>,
    state: State2,
    max_gain: &mut isize,
) {
    if state.remaining_steps <= 2 || state.closed.len() == 0 {
        let extra_me = if state.me_traveling < state.remaining_steps {
            (state.remaining_steps - state.me_traveling) * state.me.rate
        } else {
            0
        };
        let extra_elephant = if state.elephant_traveling < state.remaining_steps {
            (state.remaining_steps - state.elephant_traveling) * state.elephant.rate
        } else {
            0
        };

        *max_gain = (*max_gain).max(state.gain + extra_me + extra_elephant);
        return ();
    }

    if state.me_traveling == 0 && state.elephant_traveling == 0 {
        for new_me in &state.closed {
            for new_elephant in &state.closed {
                if new_elephant == new_me {
                    continue;
                }
                let mut new_closed = state.closed.clone();
                new_closed.remove(&new_me);
                new_closed.remove(&new_elephant);
                let new_remaining_steps_me = state.remaining_steps - dist[&(state.me, *new_me)] - 1;
                let new_remaining_steps_elephant =
                    state.remaining_steps - dist[&(state.elephant, *new_elephant)] - 1;
                let new_remaining_steps = new_remaining_steps_me.max(new_remaining_steps_elephant);
                let newstate = State2 {
                    me: *new_me,
                    elephant: *new_elephant,
                    closed: new_closed,
                    gain: state.gain
                        + state.remaining_steps * (state.me.rate + state.elephant.rate),
                    remaining_steps: new_remaining_steps,
                    me_traveling: new_remaining_steps - new_remaining_steps_me,
                    elephant_traveling: new_remaining_steps - new_remaining_steps_elephant,
                };
                traverse_with_elephant(&dist, newstate, max_gain);
            }
        }
    } else if state.elephant_traveling == 0 {
        for new_elephant in &state.closed {
            let mut new_closed = state.closed.clone();
            new_closed.remove(&new_elephant);
            let new_remaining_steps_me = state.remaining_steps - state.me_traveling;
            let new_remaining_steps_elephant =
                state.remaining_steps - dist[&(state.elephant, *new_elephant)] - 1;
            let new_remaining_steps = new_remaining_steps_me.max(new_remaining_steps_elephant);
            let newstate = State2 {
                me: state.me,
                elephant: *new_elephant,
                closed: new_closed,
                gain: state.gain + state.remaining_steps * state.elephant.rate,
                remaining_steps: new_remaining_steps,
                me_traveling: new_remaining_steps - new_remaining_steps_me,
                elephant_traveling: new_remaining_steps - new_remaining_steps_elephant,
            };
            traverse_with_elephant(&dist, newstate, max_gain);
        }
    } else if state.me_traveling == 0 {
        for new_me in &state.closed {
            let mut new_closed = state.closed.clone();
            new_closed.remove(&new_me);
            let new_remaining_steps_me = state.remaining_steps - dist[&(state.me, *new_me)] - 1;
            let new_remaining_steps_elephant = state.remaining_steps - state.elephant_traveling;
            let new_remaining_steps = new_remaining_steps_me.max(new_remaining_steps_elephant);
            let newstate = State2 {
                me: *new_me,
                elephant: state.elephant,
                closed: new_closed,
                gain: state.gain + state.remaining_steps * state.me.rate,
                remaining_steps: new_remaining_steps,
                me_traveling: new_remaining_steps - new_remaining_steps_me,
                elephant_traveling: new_remaining_steps - new_remaining_steps_elephant,
            };
            traverse_with_elephant(&dist, newstate, max_gain);
        }
    }
}

impl Puzzle for Day16 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/16.input");
        let mut valve_names = HashMap::new();
        let mut valve_list = HashMap::new();
        let mut input = HashMap::new();
        for line in INPUT.lines() {
            let (valve, list) = parse_line(line).unwrap().1;
            valve_names.insert(valve.name, valve);
            valve_list.insert(valve, list);
        }
        let valves = valve_names.values().collect::<Vec<_>>();

        for valve in valves {
            let list = valve_list[&valve]
                .iter()
                .map(|name| valve_names[name].clone())
                .collect::<Vec<_>>();
            input.insert(valve.clone(), list);
        }

        self.goal = input
            .keys()
            .filter(|v| v.rate > 0)
            .map(|v| *v)
            .collect::<HashSet<_>>();
        for start in &self.goal {
            let h = all_shortest_paths(&input, *start, &self.goal);
            for (end, d) in h {
                self.dist.insert((*start, end), d);
            }
        }

        let starting_valve = Valve {
            name: ('A', 'A'),
            rate: 0,
        };
        let h = all_shortest_paths(&input, starting_valve, &self.goal);
        for (end, d) in h {
            self.dist.insert((starting_valve, end), d);
        }
    }

    fn part1(&self) -> String {
        let starting_valve = Valve {
            name: ('A', 'A'),
            rate: 0,
        };
        let state = State1 {
            valve: starting_valve,
            closed: self.goal.clone(),
            gain: 0,
            remaining_steps: 30,
        };
        let mut gain = 0;
        traverse_alone(&self.dist, state, &mut gain);

        format!("{:?}", gain)
    }

    fn part2(&self) -> String {
        let starting_valve = Valve {
            name: ('A', 'A'),
            rate: 0,
        };
        let mut gain = 0;
        let goal_vec = self.goal.iter().collect::<Vec<_>>();
        for i in 0..self.goal.len() {
            for j in i + 1..self.goal.len() {
                let me = goal_vec[i];
                let elephant = goal_vec[j];
                let mut closed = self.goal.clone();
                closed.remove(&me);
                closed.remove(&elephant);
                let remaining_steps_me = 26 - self.dist[&(starting_valve, *me)] - 1;
                let remaining_steps_elephant = 26 - self.dist[&(starting_valve, *elephant)] - 1;
                let remaining_steps = remaining_steps_me.max(remaining_steps_elephant);
                let state = State2 {
                    me: *me,
                    elephant: *elephant,
                    closed: closed,
                    gain: 0,
                    remaining_steps: remaining_steps,
                    me_traveling: remaining_steps - remaining_steps_me,
                    elephant_traveling: remaining_steps - remaining_steps_elephant,
                };
                traverse_with_elephant(&self.dist, state, &mut gain);
            }
        }
        format!("{:?}", gain)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Dist {
    valve: Valve,
    d: isize,
}

fn all_shortest_paths(graph: &Map, start: Valve, goal: &HashSet<Valve>) -> HashMap<Valve, isize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Valve, isize> = HashMap::new();
    for valve in graph.keys() {
        dist.insert(*valve, isize::MAX);
    }

    let mut dd: HashMap<Valve, isize> = HashMap::new();

    let mut queue = VecDeque::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    queue.push_back(Dist { valve: start, d: 0 });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Dist { valve, d }) = queue.pop_front() {
        // If reached node is of interest, insert it in the HashMap
        if goal.contains(&valve) {
            dd.insert(valve, d);
        }

        // Important as we may have already found a better way
        if d > dist[&valve] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node in &graph[&valve] {
            let next = Dist {
                valve: *node,
                d: d + 1,
            };

            // If so, add it to the frontier and continue
            if next.d < dist[&next.valve] {
                queue.push_back(next);
                // Relaxation, we have now found a better way
                dist.insert(next.valve, next.d);
            }
        }
    }
    // Return HashMap of interesting distances
    dd
}
