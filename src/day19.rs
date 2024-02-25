use aoc_2022_rust::Puzzle;
use nom::{bytes::complete::tag, character::complete, combinator::map, sequence::tuple, IResult};

#[derive(Debug, Clone)]
pub struct Day19 {
    input: Vec<Blueprint>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct OreRobot {
    ore: i32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct ClayRobot {
    ore: i32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct ObsidianRobot {
    ore: i32,
    clay: i32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct GeodeRobot {
    ore: i32,
    obsidian: i32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    i: i32,
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
}

fn parse_ore_robot(input: &str) -> IResult<&str, OreRobot> {
    map(
        tuple((tag(" Each ore robot costs "), complete::i32, tag(" ore."))),
        |(_, ore, _)| OreRobot { ore },
    )(input)
}

fn parse_clay_robot(input: &str) -> IResult<&str, ClayRobot> {
    map(
        tuple((tag(" Each clay robot costs "), complete::i32, tag(" ore."))),
        |(_, ore, _)| ClayRobot { ore },
    )(input)
}

fn parse_obsidian_robot(input: &str) -> IResult<&str, ObsidianRobot> {
    map(
        tuple((
            tag(" Each obsidian robot costs "),
            complete::i32,
            tag(" ore and "),
            complete::i32,
            tag(" clay."),
        )),
        |(_, ore, _, clay, _)| ObsidianRobot { ore, clay },
    )(input)
}

fn parse_geode_robot(input: &str) -> IResult<&str, GeodeRobot> {
    map(
        tuple((
            tag(" Each geode robot costs "),
            complete::i32,
            tag(" ore and "),
            complete::i32,
            tag(" obsidian."),
        )),
        |(_, ore, _, obsidian, _)| GeodeRobot { ore, obsidian },
    )(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            tag("Blueprint "),
            complete::i32,
            tag(":"),
            parse_ore_robot,
            parse_clay_robot,
            parse_obsidian_robot,
            parse_geode_robot,
        )),
        |(_, i, _, ore, clay, obsidian, geode)| Blueprint {
            i,
            ore,
            clay,
            obsidian,
            geode,
        },
    )(input)
}

impl Puzzle for Day19 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/19.input");
        for line in INPUT.lines() {
            self.input.push(parse_blueprint(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        let start = State {
            time: 0,
            robots: (1, 0, 0, 0),
            resources: (0, 0, 0, 0),
        };
        let mut quality_level = 0;
        for blueprint in &self.input {
            //            println!("Processing blueprint {:}", blueprint.i);
            let geode = dfs(start, *blueprint, 24);
            quality_level += blueprint.i * geode;
        }
        format!("{:?}", quality_level)
    }

    fn part2(&self) -> String {
        let start = State {
            time: 0,
            robots: (1, 0, 0, 0),
            resources: (0, 0, 0, 0),
        };
        let mut total = 1;
        for blueprint in self.input.iter().take(3) {
            //            println!("Processing blueprint {:}", blueprint.i);
            let geode = dfs(start, *blueprint, 32);
            //            println!("{:}", geode);
            total *= geode;
        }
        format!("{:?}", total)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    time: i32,
    robots: Robots,       // (ore, clay, obsidian, geode)
    resources: Resources, // (ore, clay, obsidian, geode)
}

type Robots = (i32, i32, i32, i32);
type Resources = (i32, i32, i32, i32);

fn compute_resources(robots: Robots, resources: Resources, time: i32) -> Resources {
    (
        robots.0 * time + resources.0,
        robots.1 * time + resources.1,
        robots.2 * time + resources.2,
        robots.3 * time + resources.3,
    )
}

fn next_states(state: &State, blueprint: Blueprint, time_bound: i32) -> Vec<State> {
    let mut nxt_sts = Vec::new();
    let (ore, clay, obsidian, geode) = state.resources;
    let Blueprint {
        i: _,
        ore: ore_robot,
        clay: clay_robot,
        obsidian: obsidian_robot,
        geode: geode_robot,
    } = blueprint;

    // Let's make an Ore robot
    let time_needed = if blueprint.ore.ore <= state.resources.0 {
        1
    } else {
        div_ceil(blueprint.ore.ore - state.resources.0, state.robots.0) + 1
    };
    if state.robots.0
        < ore_robot
            .ore
            .max(clay_robot.ore)
            .max(obsidian_robot.ore)
            .max(geode_robot.ore)
        && state.time + time_needed <= time_bound
    {
        let st = State {
            time: state.time + time_needed,
            robots: (
                state.robots.0 + 1,
                state.robots.1,
                state.robots.2,
                state.robots.3,
            ),
            resources: compute_resources(
                state.robots,
                (ore - blueprint.ore.ore, clay, obsidian, geode),
                time_needed,
            ),
        };
        nxt_sts.push(st);
    }
    // Let's make a Clay robot
    let time_needed = if blueprint.clay.ore <= state.resources.0 {
        1
    } else {
        div_ceil(blueprint.clay.ore - state.resources.0, state.robots.0) + 1
    };
    if state.robots.1 < obsidian_robot.clay && state.time + time_needed <= time_bound {
        let st = State {
            time: state.time + time_needed,
            robots: (
                state.robots.0,
                state.robots.1 + 1,
                state.robots.2,
                state.robots.3,
            ),
            resources: compute_resources(
                state.robots,
                (ore - blueprint.clay.ore, clay, obsidian, geode),
                time_needed,
            ),
        };
        nxt_sts.push(st);
    }

    // Let's make an Obsidian robot if possible at all
    if state.robots.1 > 0 {
        let time_ore = if blueprint.obsidian.ore <= state.resources.0 {
            1
        } else {
            div_ceil(blueprint.obsidian.ore - state.resources.0, state.robots.0) + 1
        };
        let time_clay = if blueprint.obsidian.clay <= state.resources.1 {
            1
        } else {
            div_ceil(blueprint.obsidian.clay - state.resources.1, state.robots.1) + 1
        };
        let time_needed = time_ore.max(time_clay);
        if state.robots.2 < geode_robot.obsidian && state.time + time_needed <= time_bound {
            let st = State {
                time: state.time + time_needed,
                robots: (
                    state.robots.0,
                    state.robots.1,
                    state.robots.2 + 1,
                    state.robots.3,
                ),
                resources: compute_resources(
                    state.robots,
                    (
                        ore - blueprint.obsidian.ore,
                        clay - blueprint.obsidian.clay,
                        obsidian,
                        geode,
                    ),
                    time_needed,
                ),
            };
            nxt_sts.push(st);
        }
    }

    // Let's make a Geode robot if possible at all
    if state.robots.2 > 0 {
        let time_ore = if blueprint.geode.ore <= state.resources.0 {
            1
        } else {
            div_ceil(blueprint.geode.ore - state.resources.0, state.robots.0) + 1
        };
        let time_obsidian = if blueprint.geode.obsidian <= state.resources.2 {
            1
        } else {
            div_ceil(blueprint.geode.obsidian - state.resources.2, state.robots.2) + 1
        };
        let time_needed = time_ore.max(time_obsidian);
        if state.time + time_needed <= time_bound {
            let st = State {
                time: state.time + time_needed,
                robots: (
                    state.robots.0,
                    state.robots.1,
                    state.robots.2,
                    state.robots.3 + 1,
                ),
                resources: compute_resources(
                    state.robots,
                    (
                        ore - blueprint.geode.ore,
                        clay,
                        obsidian - blueprint.geode.obsidian,
                        geode,
                    ),
                    time_needed,
                ),
            };
            nxt_sts.push(st);
        }
    }
    if nxt_sts.is_empty() {
        let time_needed = time_bound - state.time;
        nxt_sts.push(State {
            time: time_bound,
            robots: state.robots,
            resources: compute_resources(state.robots, state.resources, time_needed),
        })
    }
    nxt_sts
}

fn dfs(start: State, blueprint: Blueprint, time_bound: i32) -> i32 {
    //State {
    let mut queue = Vec::new();
    let mut geode = 0;
    // Insert starting node in the queue
    queue.push(start);

    // While there are unexplored states...
    while let Some(state) = queue.pop() {
        if state.time == time_bound {
            geode = geode.max(state.resources.3);
        } else {
            for new_state in next_states(&state, blueprint, time_bound) {
                queue.push(new_state);
            }
        }
    }
    geode
}

fn div_ceil(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}
