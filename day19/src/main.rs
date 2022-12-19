use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day19/input.txt");
    //let file_lines = read_file_lines("day19/example-input.txt");
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Parsing time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 1
    writeln!(stdout, "*********** PART 1 ***********").unwrap();
    let start_time = Instant::now();
    let part1_answer = part1(&file_lines);
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 1 answer: {}", part1_answer).unwrap();
    writeln!(stdout, "Part 1 time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 2
    writeln!(stdout, "*********** PART 2 ***********").unwrap();
    let start_time = Instant::now();
    let part2_answer = part2(&file_lines);
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 2 answer: {}", part2_answer).unwrap();
    writeln!(stdout, "Part 2 time: {}us", elapsed.as_micros()).unwrap();
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Resources {
    fn new() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn ore_robot(ore: i64) -> Resources {
        Resources {
            ore,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn clay_robot(ore: i64) -> Resources {
        Resources {
            ore,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn obsidian_robot(ore: i64, clay: i64) -> Resources {
        Resources {
            ore,
            clay,
            obsidian: 0,
            geode: 0,
        }
    }

    fn geode_robot(ore: i64, obsidian: i64) -> Resources {
        Resources {
            ore,
            clay: 0,
            obsidian,
            geode: 0,
        }
    }

    fn one_ore() -> Resources {
        Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn one_clay() -> Resources {
        Resources {
            ore: 0,
            clay: 1,
            obsidian: 0,
            geode: 0,
        }
    }

    fn one_obsidian() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 1,
            geode: 0,
        }
    }

    fn one_geode() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 1,
        }
    }
}

impl std::ops::Add for Resources {
    type Output = Resources;

    fn add(self, other: Resources) -> Resources {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl std::ops::AddAssign for Resources {
    fn add_assign(&mut self, other: Resources) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

impl std::ops::Sub for Resources {
    type Output = Resources;

    fn sub(self, other: Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl std::ops::SubAssign for Resources {
    fn sub_assign(&mut self, other: Resources) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

impl std::ops::Mul<i64> for Resources {
    type Output = Resources;

    fn mul(self, scalar: i64) -> Resources {
        Resources {
            ore: self.ore * scalar,
            clay: self.clay * scalar,
            obsidian: self.obsidian * scalar,
            geode: self.geode * scalar,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: i64,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn from_string(line: &str) -> Blueprint {
        let caps = REGEX.captures(line).unwrap();
        let id = caps[1].parse().unwrap();
        let ore_robot_cost = Resources::ore_robot(caps[2].parse().unwrap());
        let clay_robot_cost = Resources::clay_robot(caps[3].parse().unwrap());
        let obsidian_robot_cost =
            Resources::obsidian_robot(caps[4].parse().unwrap(), caps[5].parse().unwrap());
        let geode_robot_cost =
            Resources::geode_robot(caps[6].parse().unwrap(), caps[7].parse().unwrap());

        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Factory {
    blueprint: Blueprint,
    resources: Resources,
    robots: Resources,
    num_steps_remaining: i64,
}

impl Factory {
    fn new(blueprint: &Blueprint, robots: Resources, steps: i64) -> Factory {
        Factory {
            blueprint: *blueprint,
            resources: Resources::new(),
            robots,
            num_steps_remaining: steps,
        }
    }

    fn collect(&self) -> Factory {
        let mut new_factory = *self;
        new_factory.resources += self.robots;
        new_factory.num_steps_remaining -= 1;
        new_factory
    }

    fn build_robot_with_cost(&self, robot: Resources, cost: Resources) -> Factory {
        let mut new_factory = *self;

        let resources_needed = cost - new_factory.resources;
        let mut max_time_needed = 0;
        if resources_needed.ore > 0 {
            max_time_needed = max_time_needed
                .max((resources_needed.ore + new_factory.robots.ore - 1) / new_factory.robots.ore);
        }
        if resources_needed.clay > 0 {
            max_time_needed = max_time_needed.max(
                (resources_needed.clay + new_factory.robots.clay - 1) / new_factory.robots.clay,
            );
        }
        if resources_needed.obsidian > 0 {
            max_time_needed = max_time_needed.max(
                (resources_needed.obsidian + new_factory.robots.obsidian - 1)
                    / new_factory.robots.obsidian,
            );
        }
        if resources_needed.geode > 0 {
            max_time_needed = max_time_needed.max(
                (resources_needed.geode + new_factory.robots.geode - 1) / new_factory.robots.geode,
            );
        }

        // Step one extra to give time to build the robot
        max_time_needed += 1;

        if max_time_needed > new_factory.num_steps_remaining {
            // Not enough time, so collect until time reaches 0
            new_factory.resources += new_factory.robots * new_factory.num_steps_remaining;
            new_factory.num_steps_remaining = 0;
        } else {
            // Collect until there is enough resources to build the robot, and then one more step while building
            new_factory.resources += new_factory.robots * max_time_needed;
            new_factory.num_steps_remaining -= max_time_needed;

            // Build the robot
            new_factory.resources -= cost;
            new_factory.robots += robot;
        }
        new_factory
    }

    fn most_geodes_possible(&self) -> i64 {
        self.resources.geode
            + self.robots.geode * self.num_steps_remaining
            + ((self.num_steps_remaining - 1) * self.num_steps_remaining) / 2
    }
}

fn parse_blueprints(file_lines: &[String]) -> Vec<Blueprint> {
    file_lines
        .iter()
        .map(|line| Blueprint::from_string(line))
        .collect()
}

fn most_geodes(factory: Factory) -> i64 {
    let mut factory_queue = vec![factory];
    let mut most_geodes_found = 0;
    while let Some(factory) = factory_queue.pop() {
        // If factory is done, check if it has the most geodes
        if factory.num_steps_remaining == 0 {
            most_geodes_found = most_geodes_found.max(factory.resources.geode);
            continue;
        }

        // If the factory has only one step left, collect and check if it has the most geodes
        // This is because it is too late to build another robot anyway
        if factory.num_steps_remaining == 1 {
            let new_factory = factory.collect();
            most_geodes_found = most_geodes_found.max(new_factory.resources.geode);
            continue;
        }

        // If this factory cannot be the best, don't bother exploring it
        if factory.most_geodes_possible() <= most_geodes_found {
            continue;
        }

        // Build ore robot next
        let new_factory =
            factory.build_robot_with_cost(Resources::one_ore(), factory.blueprint.ore_robot_cost);
        most_geodes_found = most_geodes_found.max(new_factory.resources.geode);
        if new_factory.num_steps_remaining > 0 {
            factory_queue.push(new_factory);
        }

        // Build clay robot next
        let new_factory =
            factory.build_robot_with_cost(Resources::one_clay(), factory.blueprint.clay_robot_cost);
        most_geodes_found = most_geodes_found.max(new_factory.resources.geode);
        if new_factory.num_steps_remaining > 0 {
            factory_queue.push(new_factory);
        }

        // Build obsidian robot next, if possible
        if factory.robots.clay > 0 {
            let new_factory = factory.build_robot_with_cost(
                Resources::one_obsidian(),
                factory.blueprint.obsidian_robot_cost,
            );
            most_geodes_found = most_geodes_found.max(new_factory.resources.geode);
            if new_factory.num_steps_remaining > 0 {
                factory_queue.push(new_factory);
            }
        }

        // Build geode robot next, if possible
        if factory.robots.obsidian > 0 {
            let new_factory = factory
                .build_robot_with_cost(Resources::one_geode(), factory.blueprint.geode_robot_cost);
            most_geodes_found = most_geodes_found.max(new_factory.resources.geode);
            if new_factory.num_steps_remaining > 0 {
                factory_queue.push(new_factory);
            }
        }
    }

    most_geodes_found
}

fn part1(file_lines: &[String]) -> String {
    let mut total_quality = 0;

    let blueprints = parse_blueprints(file_lines);
    for blueprint in blueprints {
        let factory = Factory::new(&blueprint, Resources::one_ore(), 24);
        let geodes = most_geodes(factory);
        let quality = geodes * blueprint.id;
        total_quality += quality;
    }

    total_quality.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let mut answer = 1;

    let blueprints = parse_blueprints(file_lines);
    for blueprint in blueprints.iter().take(3) {
        let factory = Factory::new(blueprint, Resources::one_ore(), 32);
        let geodes = most_geodes(factory);
        answer *= geodes;
    }

    answer.to_string()
}
