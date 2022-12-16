use std::collections::HashMap;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u64};
use nom::combinator::{map_res, opt};
use nom::multi::separated_list1;
use nom::IResult;
use petgraph::adj::{Neighbors, NodeIndex};
use petgraph::algo::floyd_warshall;
use petgraph::graph::Graph;

struct ProblemGraph {
    node_weights: Vec<u64>,
    dist_mat: Vec<Vec<i32>>,
}

impl ProblemGraph {
    fn parse(input: &str) -> ProblemGraph {
        let (res, line_output) = separated_list1(tag("\n"), read_line)(input).unwrap();

        let mut g: Graph<(u64), ()> = Graph::new();

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
        println!("{:?}", all_pairs_paths);
        let relevant_node_names: Vec<String> = line_output
            .iter()
            .filter(|(name, weight, _)| name == "AA" || *weight > 0)
            .map(|(node, _, _)| node.clone())
            .collect();

        let relevant_node_idxs = relevant_node_names.iter().map(|name| node_ids.get(name).unwrap()).cloned().collect_vec();
        let mut weights = vec![];
        let mut dist_mat = vec![];
        for (i, idx_i) in relevant_node_idxs.iter().enumerate() {
            weights.push(*g.node_weight(*idx_i).unwrap());
            dist_mat.push(vec![]);
            for (j, idx_j) in relevant_node_idxs.iter().enumerate() {
                dist_mat[i].push(*all_pairs_paths.get(&(*idx_i, *idx_j)).unwrap());
            }
        }
        
        ProblemGraph { node_weights: weights, dist_mat }
    }
}

type LineOutput = (String, u64, Vec<String>);

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

fn read_flow_rate(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag(" has flow rate=")(input)?;
    u64(input)
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
        let (input, output) = read_line(input).unwrap();
        assert_eq!(output.0, "AA");
        assert_eq!(output.1, 22);
        assert_eq!(output.2, vec!["BB".to_string(), "CC".to_string()]);
    }

    #[test]
    fn test_graph_reading() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB, CC\nValve BB has flow rate=15; tunnel leads to valve AA\nValve CC has flow rate=12; tunnel leads to valve AA";
        let g = ProblemGraph::parse(input);

        assert_eq!(g.node_weights, vec![0, 15, 12]);
        assert_eq!(g.dist_mat, vec![vec![0, 1, 1], vec![1, 0, 2], vec![1, 2, 0]]);

    }
}
