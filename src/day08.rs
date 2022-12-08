use indoc::indoc;
use ndarray::prelude::*;

fn digit_matrix_to_array(input: &str) -> Array2<i8> {
    //first read input into vec
    let in_as_vec: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    // now get dimensions; skip validation
    let n_rows = in_as_vec.len();
    let n_cols = in_as_vec[0].len();

    let mut arr: Array2<i8> = Array2::default((n_rows, n_cols));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = in_as_vec[i][j];
        }
    }

    arr
}


enum Direction {
    Normal,
    Reverse
}

fn cumulative_max(arr: &Array2<i8>, axis: Axis, direction: Direction) -> Array2<i8> {
    let mut cum_max: Array2<i8> = Array2::default((arr.nrows(), arr.ncols()));
    
    for (i, row) in arr.axis_iter(axis).enumerate() {
        let mut acc = 0;

        let row_iter_enum: Box<dyn Iterator<Item=(usize, &i8)>> = match &direction {
            Direction::Normal => Box::new(row.iter().enumerate()),
            Direction::Reverse => Box::new(row.iter().enumerate().rev())
        };

        for (j, &x) in row_iter_enum {
            if acc < x {
                acc = x;
            }
            cum_max[[i, j]] = acc;
        }
    }
    if axis == Axis(0) {
        cum_max
    } else {
        cum_max.reversed_axes()
    }
}

#[cfg(test)]
mod tests {
    use ndarray::{Array1, Zip};

    use super::*;
    #[test]
    fn check_test_building_array() {
        let input = "123\n456\n789";

        let arr: Array2<i8> = digit_matrix_to_array(input);

        assert_eq!(1, arr[[0, 0]]);
        assert_eq!(4, arr[[1, 0]]);
        assert_eq!(6, arr[[1, 2]]);
    }

    #[test]
    fn test_cumulative_max() {
        let input = indoc!(
            "
            30373
            25512
            65332
            33549
            35390"
        );

        let arr: Array2<i8> = digit_matrix_to_array(input);

        // figure out cumulative maximum leftwise
        let cum_max: Array2<i8> = cumulative_max(&arr, Axis(0), Direction::Normal);
        assert_eq!(cum_max[[1, 3]], 5);

        let cum_max: Array2<i8> = cumulative_max(&arr, Axis(1), Direction::Normal);
        assert_eq!(cum_max[[3,0]], 6);

        let cum_max: Array2<i8> = cumulative_max(&arr, Axis(0), Direction::Reverse);
        assert_eq!(cum_max[[1,0]], 5);

        let cum_max: Array2<i8> = cumulative_max(&arr, Axis(1), Direction::Reverse);
        assert_eq!(cum_max[[0,3]], 9);
    }

    #[test]
    fn test_compute_visibility() {
        let input = indoc!(
            "
            30373
            25512
            65332
            33549
            35390"
        );
        
        let arr: Array2<i8> = digit_matrix_to_array(input);

        // figure out cumulative maximum leftwise
        let cum_max: Array2<i8> = cumulative_max(&arr, Axis(0), Direction::Normal);

        // now use that to figure out which trees are visible
        // first, we grab a view of the original array from the middle
        let relevant_trees = arr.slice(s![1..-1, 1..-1]);
        assert_eq!(relevant_trees[[0,0]], 5);

        // next, we grab the relevant cumsum slice:
        let relevant_max = cum_max.slice(s![1..-1,0..-2]);
        assert_eq!(relevant_max[[0,0]], 2);

        // a tree is visible from the left if its height is larger than the one at the relevant tree
        let some_output: Array2<bool> = Zip::from(&relevant_max).and(&relevant_trees).map_collect(|&max, &tree| max < tree);
        assert_eq!(some_output[[0,0]], true);
        assert_eq!(some_output[[0, 2]], false);

    }
}
