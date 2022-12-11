use itertools::Itertools;

pub fn render<I: IntoIterator<Item=i32>>(input: I) -> String {
    let mut iter = input.into_iter();

    let mut row = (&mut iter).take(40).collect_vec();
    let mut chars: Vec<char> = Vec::new();
    
    while row.len() == 40 {
        let col_positions = (0..40);
        let chars_iter = col_positions.zip(row).map(|(col, x)| {
            if (col - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        });
        chars.extend(chars_iter);
        row = (&mut iter).take(40).collect_vec();
        if row.len() > 0 {
            chars.push('\n');
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn render_single_line() {
        use std::iter;

        let input = iter::repeat(5).take(40);

        let output = render(input);

        assert_eq!(output, "....###.................................");
    }

    #[test]
    fn render_two_lines() {
        use std::iter;

        let input = iter::repeat(5).take(80);

        let output = render(input);

        assert_eq!(output, "....###.................................\n....###.................................");
    }
}