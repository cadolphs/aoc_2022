use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u64};
use nom::branch::alt;
use nom::combinator::{opt, map_res};
use nom::multi::separated_list1;


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
    use crate::day16::{read_flow_rate, read_tunnel_list};

    use super::read_valve;

    #[test]
    fn test_reading_valve() {
        let input = "Valve AA has flow rate=22; tunnels lead to valves BB, CC";
        let (input, valve) = read_valve(input).unwrap();
        assert_eq!(valve, "AA".to_string());

        let (input, flow_rate) = read_flow_rate(input).unwrap();
        assert_eq!(flow_rate, 22);

        let (input, tunnels) = read_tunnel_list(input).unwrap();
        assert_eq!(input, "");
    }
}