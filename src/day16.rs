use std::collections::HashMap;

use bit_set::BitSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::IResult;
use petgraph::algo::floyd_warshall;
use petgraph::graph::Graph;

pub fn run_day_16(input: String) {
    let g = ProblemGraph::parse(&input);

    println!("There are {} relevant nodes", g.node_weights.len());

    let ans = g.solve();

    println!("Best steam release is {}", ans);

    let ans2 = g.solve2();

    println!("Best answer with elephant is {}", ans2);
}
#[derive(Clone)]
struct ProblemGraph {
    node_weights: Vec<i32>,
    dist_mat: Vec<Vec<i32>>,
    start_node: usize,
}

type MemoMap = HashMap<(usize, i32, BitSet), i32>;

type MemoMap2 = HashMap<(usize, usize, i32, BitSet), i32>;

impl ProblemGraph {
    fn parse(input: &str) -> ProblemGraph {
        let (res, line_output) = separated_list1(tag("\n"), read_line)(input).unwrap();

        let mut g: Graph<(i32), ()> = Graph::new();

        let mut node_ids: HashMap<String, _> = HashMap::new();

        for (node, weight, _) in &line_output {
            let node_id = g.add_node(*weight);
            node_ids.insert(node.clone(), node_id);
        }

        for (node, _, neighbors) in &line_output {
            let node_id = node_ids.get(node).unwrap();
            for neighbor in neighbors {
                let neighbor_id = node_ids.get(neighbor).unwrap();
                g.update_edge(*node_id, *neighbor_id, ());
            }
        }

        let all_pairs_paths = floyd_warshall(&g, |_| 1).unwrap();

        let relevant_node_names: Vec<String> = line_output
            .iter()
            .filter(|(name, weight, _)| name == "AA" || *weight > 0)
            .map(|(node, _, _)| node.clone())
            .collect();

        let relevant_node_idxs = relevant_node_names
            .iter()
            .map(|name| node_ids.get(name).unwrap())
            .cloned()
            .collect_vec();
        let mut weights = vec![];
        let mut dist_mat = vec![];
        for (i, idx_i) in relevant_node_idxs.iter().enumerate() {
            weights.push(*g.node_weight(*idx_i).unwrap());
            dist_mat.push(vec![]);
            for (_j, idx_j) in relevant_node_idxs.iter().enumerate() {
                dist_mat[i].push(*all_pairs_paths.get(&(*idx_i, *idx_j)).unwrap());
            }
        }

        // the start node is the one with 0 flow. All others have been removed!
        let start_node = weights
            .iter()
            .find_position(|weight| **weight == 0)
            .unwrap()
            .0;

        ProblemGraph {
            node_weights: weights,
            dist_mat,
            start_node,
        }
    }

    fn solve(&self) -> i32 {
        let mut memo: MemoMap = HashMap::new();
        let mut available_nodes: BitSet = BitSet::with_capacity(self.node_weights.len());
        for (node, weight) in self.node_weights.iter().enumerate() {
            if *weight > 0 {
                available_nodes.insert(node);
            }
        }
        assert_eq!(available_nodes.len(), 15);
        let time = 30;

        self.find_max(&mut memo, self.start_node, time, available_nodes)
    }

    fn solve2(&self) -> i32 {
        let mut available_nodes: BitSet = BitSet::with_capacity(self.node_weights.len());
        for (node, weight) in self.node_weights.iter().enumerate() {
            if *weight > 0 {
                available_nodes.insert(node);
            }
        }

        let mut best_overall = 0;
        for el_subset_len in 1..(available_nodes.len()/2) {
            for el_items in available_nodes.iter().combinations(el_subset_len) {
                let mut el_nodes = BitSet::from_iter(el_items);
                let mut my_nodes: BitSet<u32> = BitSet::from_iter(available_nodes.difference(&el_nodes));

                let mut el_memo = HashMap::new();
                let mut my_memo = HashMap::new();
                
                let best_elephant = self.find_max(&mut el_memo, self.start_node, 26, el_nodes);
                let best_human = self.find_max(&mut my_memo, self.start_node, 26, my_nodes);

                let best = best_human + best_elephant;
                best_overall = std::cmp::max(best, best_overall);
            }
        }

        best_overall
    }


    fn find_max(&self, memo: &mut MemoMap, node: usize, time: i32, available_nodes: BitSet) -> i32 {
        if time <= 2 {
            // not enough time to _go_ to a valve _and_ open it _and_ benefit from it
            return 0;
        }
        // now try memo
        let key = (node, time, available_nodes.clone());
        if let Some(val) = memo.get(&key) {
            return *val;
        }

        // find all nodes we _could_ get to and active and have time to spare
        let max_dist = time - 2;
        let candidates = available_nodes
            .iter()
            .filter(|&n| self.dist_mat[node][n] <= max_dist);

        // first without branch test just to get it right
        let best = candidates
            .map(|n| {
                let mut new_available_nodes = available_nodes.clone();
                new_available_nodes.remove(n);
                let time_left = time - 1 - self.dist_mat[node][n];
                self.node_weights[n] * time_left
                    + self.find_max(memo, n, time_left, new_available_nodes)
            })
            .max()
            .unwrap_or(0);
        memo.insert(key, best);
        best
    }
}

type LineOutput = (String, i32, Vec<String>);

fn read_line(input: &str) -> IResult<&str, LineOutput> {
    let (input, valve) = read_valve(input)?;
    let (input, flow_rate) = read_flow_rate(input)?;
    let (input, neighbors) = read_tunnel_list(input)?;

    Ok((input, (valve, flow_rate, neighbors)))
}

fn read_valve(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve): (&str, &str) = alpha1(input)?;

    Ok((input, valve.to_string()))
}

fn read_flow_rate(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag(" has flow rate=")(input)?;
    i32(input)
}

fn read_tunnel_or_tunnels(input: &str) -> IResult<&str, &str> {
    let read_tunnel = tag("; tunnel leads ");
    let read_tunnels = tag("; tunnels lead ");

    let (input, _) = alt((read_tunnel, read_tunnels))(input)?;
    let (input, _) = tag("to valve")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    tag(" ")(input)
}

fn read_tunnel_list(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = read_tunnel_or_tunnels(input)?;
    let (input, valves) = separated_list1(tag(", "), alpha1)(input)?;
    let valves = valves.into_iter().map(|valve| valve.to_string()).collect();

    Ok((input, valves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_valve() {
        let input = "Valve AA has flow rate=22; tunnels lead to valves BB, CC";
        let (input, valve) = read_valve(input).unwrap();
        assert_eq!(valve, "AA".to_string());

        let (input, flow_rate) = read_flow_rate(input).unwrap();
        assert_eq!(flow_rate, 22);

        let (input, tunnels) = read_tunnel_list(input).unwrap();
        assert_eq!(input, "");

        let input = "Valve AA has flow rate=22; tunnels lead to valves BB, CC\nSome more stuff";
        let (_, output) = read_line(input).unwrap();
        assert_eq!(output.0, "AA");
        assert_eq!(output.1, 22);
        assert_eq!(output.2, vec!["BB".to_string(), "CC".to_string()]);
    }

    #[test]
    fn test_graph_reading() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB, CC\nValve BB has flow rate=15; tunnel leads to valve AA\nValve CC has flow rate=12; tunnel leads to valve AA";
        let g = ProblemGraph::parse(input);

        assert_eq!(g.node_weights, vec![0, 15, 12]);
        assert_eq!(
            g.dist_mat,
            vec![vec![0, 1, 1], vec![1, 0, 2], vec![1, 2, 0]]
        );

        assert_eq!(g.start_node, 0); // not actually sure if that's stable
    }
}
