use crate::prelude::*;
use std::collections::VecDeque;

day!(19, parse => part1, part2);

struct Blueprint {
    robot_costs: [[u16; 4]; 4],
}
impl std::str::FromStr for Blueprint {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Blueprint, Self::Err> {
        let mut segs = input.split_whitespace();

        let ore_bot_costs = [segs.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        let clay_bot_costs = [segs.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        let obsidian_bot_costs = [
            segs.nth(5).unwrap().parse().unwrap(),
            segs.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];
        let geode_bot_costs = [
            segs.nth(5).unwrap().parse().unwrap(),
            0,
            segs.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = Blueprint {
            robot_costs: [
                ore_bot_costs,
                clay_bot_costs,
                obsidian_bot_costs,
                geode_bot_costs,
            ],
        };
        Ok(blueprint)
    }
}

struct State {
    resources: [u16; 4],
    bots: [u16; 4],
    time: u16,
}

fn find_max_geodes(time: u16, blueprint: &Blueprint) -> u16 {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        resources: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        time: 0,
    });

    let mut max_geodes = 0;
    let max_robots = blueprint
        .robot_costs
        .iter()
        .fold([0, 0, 0, u16::MAX], |mut cur, costs| {
            cur[0] = cur[0].max(costs[0]);
            cur[1] = cur[1].max(costs[1]);
            cur[2] = cur[2].max(costs[2]);
            cur
        });
    while let Some(state) = queue.pop_front() {
        for i in 0..4 {
            // check to see if we have created enough robots
            if state.bots[i] == max_robots[i] {
                continue;
            }

            let costs = &blueprint.robot_costs[i];

            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= state.resources[idx] => 0,
                        // no target bot type made yet
                        // we can't build it (it takes more than max_time to build it).
                        _ if state.bots[idx] == 0 => time + 1,
                        _ => {
                            (costs[idx] - state.resources[idx] + state.bots[idx] - 1)
                                / state.bots[idx]
                        }
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = state.time + wait_time + 1;
            if new_elapsed >= time {
                continue;
            }

            // gather ores with previously available bots
            let mut new_inventory = [0; 4];
            for idx in 0..state.bots.len() {
                new_inventory[idx] =
                    state.resources[idx] + state.bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = state.bots;
            new_bots[i] += 1;

            // extra optimization:
            // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
            let remaining_time = time - new_elapsed;
            if ((remaining_time - 1) * remaining_time) / 2
                + new_inventory[3]
                + remaining_time * new_bots[3]
                < max_geodes
            {
                continue;
            }

            queue.push_back(State {
                resources: new_inventory,
                bots: new_bots,
                time: new_elapsed,
            })
        }
        let geodes = state.resources[3] + state.bots[3] * (time - state.time);
        max_geodes = geodes.max(max_geodes);
    }
    max_geodes
}

fn parse(input: &str) -> ParseResult<Vec<Blueprint>> {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect_vec();
    Ok(blueprints)
}

fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .map(|blueprint| find_max_geodes(24, blueprint))
        .enumerate()
        .map(|(i, max_geodes)| (i + 1) * usize::from(max_geodes))
        .sum()
}

fn part2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|blueprint| usize::from(find_max_geodes(32, blueprint)))
        .product()
}

tests! {
    const EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.
";
    const INPUT: &str = include_str!("data/19.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 33);
    simple_tests!(parse, part1, part1_input_test, INPUT => 1023);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 3472);
    simple_tests!(parse, part2, part2_input_test, INPUT => 13520);
}
