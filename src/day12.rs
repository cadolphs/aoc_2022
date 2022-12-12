pub fn run_day_12(input: String) {

}

fn read_terrain(input: &str) -> (Vec<Vec<i8>>, Vec2D, Vec2D) {
    let mut terrain = Vec::new();
    
    let mut start_pos = Vec2D(0, 0);
    let mut end_pos = Vec2D(0,0);

    for (row, line) in input.lines().enumerate() {
        terrain.push(vec![]);
        for (col, letter) in line.chars().enumerate() {
            terrain[row].push(letter_to_height(letter));
            if letter == 'S' {
                start_pos = Vec2D(row, col);
            }
            else if letter == 'E' {
                end_pos = Vec2D(row, col);
            }
    }}

    (terrain, start_pos, end_pos)
}

fn letter_to_height(letter: char) -> i8 {
    match letter {
        'S' => 1,
        'E' => 26,
        _ => {
            let code = letter as i8;
            if code >= ('a' as i8) && code <= ('z' as i8) {
                code - ('a' as i8) + 1
            } else {
                panic!("Invalid input.")
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2D(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_read_map() {
        let map = indoc!("
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi");

        let (terrain, start, stop) = read_terrain(map);
        assert_eq!(terrain.len(), 5);
        assert_eq!(terrain[0].len(), 8);

        assert_eq!(start, Vec2D(0,0));
        assert_eq!(stop, Vec2D(2,5));

        assert_eq!(terrain[start.0][start.1], 1);
        assert_eq!(terrain[stop.0][stop.1], 26);
    }
}