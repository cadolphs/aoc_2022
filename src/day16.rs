use std::collections::HashMap;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u64, line_ending};
use nom::branch::alt;
use nom::combinator::{opt, map_res};
use nom::multi::separated_list1;
use petgraph::graph::UnGraph;

struct InputGraph {
    g: UnGraph<u64, ()>
}

impl InputGraph {
    fn new(graph: UnGraph<String, ()>) -> Self {
        InputGraph { g: graph }
    }

    fn parse(input: &str) -> InputGraph {
        let (res, line_output) = separated_list1(tag("\n"), read_line)(input).unwrap();

        let g: UnGraph<String, ()> = UnGraph::new_undirected();

        for (node, flow_rate, neighbors) in &line_output {
            g.add_node(weight)
        }
        graph
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
        let input = "Valve AA has flow rate=22; tunnels lead to valves BB, CC\nValve BB has flow rate=15; tunnel leads to valve AA";
        let g = InputGraph::parse(input);
    }
}