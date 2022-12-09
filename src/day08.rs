use ndarray::prelude::*;
use ndarray::Zip;

pub fn run_day_08(input: String) {
    let trees = digit_matrix_to_array(&input);

    let ans = count_visible_trees(&trees);

    println!("There are {} visible trees!", ans);
}

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

fn cumulative_max(arr: &Array2<i8>, view: ViewPoint) -> Array2<i8> {
    fn cum_sum_for_array(row: &ArrayView1<i8>) -> Array1<i8> {
        let mut max: i8 = 0;
        Zip::from(row).map_collect(|&x| {
            if x > max {
                max = x;
            }
            max
        })
    }

    let axis = match &view {
        ViewPoint::Left | ViewPoint::Right => Axis(0),
        _ => Axis(1),
    };

    let slice = match &view {
        ViewPoint::Left | ViewPoint::Top => s![.., ..],
        _ => s![..;-1, ..;-1],
    };

    let mut cum_max: Array2<i8> = Array2::default(arr.raw_dim());
    let mut cum_max_view = cum_max.slice_mut(slice);
    let arr_view = arr.slice(slice);

    Zip::from(cum_max_view.axis_iter_mut(axis))
        .and(arr_view.axis_iter(axis))
        .for_each(|mut cumulative, slice| {
            cumulative.assign(&cum_sum_for_array(&slice));
        });

    return cum_max;
}

fn count_visible_trees(arr: &Array2<i8>) -> usize {
    use ViewPoint::*;
    let views = vec![Left, Right, Top, Bottom];

    fn combine_maps(a: &Array2<bool>, b: &Array2<bool>) -> Array2<bool> {
        Zip::from(a).and(b).map_collect(|a, b| *a || *b)
    }

    let visible_tree_maps: Vec<Array2<bool>> = views
        .iter()
        .map(|view| compute_visible_trees_from(arr, *view))
        .collect();

    let visible_trees = visible_tree_maps.iter().fold(
        Array2::default((arr.nrows() - 2, arr.ncols() - 2)),
        |acc, el| combine_maps(&acc, el),
    );

    let answer = visible_trees.iter().filter(|&&i| i == true).count();

    let trees_on_perimeter = 2 * (arr.nrows() + arr.ncols()) - 4; //count all edges counts corners twice
    answer + trees_on_perimeter
}

#[derive(Debug, Clone, Copy)]
enum ViewPoint {
    Left,
    Right,
    Top,
    Bottom,
}

fn compute_visible_trees_from(tree_map: &Array2<i8>, view: ViewPoint) -> Array2<bool> {
    let relevant_trees = tree_map.slice(s![1..-1, 1..-1]); //only inner trees important

    let cum_max = cumulative_max(&tree_map, view);

    use ViewPoint::*;
    let some_slice = match view {
        Left => s![1..-1, 0..-2],
        Right => s![1..-1, 2..],
        Top => s![0..-2, 1..-1],
        Bottom => s![2.., 1..-1],
    };

    let relevant_max = cum_max.slice(some_slice);

    let visible_trees: Array2<bool> = Zip::from(&relevant_max)
        .and(&relevant_trees)
        .map_collect(|&max, &tree| max < tree);
    visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

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
        let cum_max: Array2<i8> = cumulative_max(&arr, ViewPoint::Left);
        assert_eq!(cum_max[[1, 3]], 5);

        let cum_max: Array2<i8> = cumulative_max(&arr, ViewPoint::Top);
        assert_eq!(cum_max[[3, 0]], 6);

        let cum_max: Array2<i8> = cumulative_max(&arr, ViewPoint::Right);
        assert_eq!(cum_max[[1, 0]], 5, "Failed for right side");

        let cum_max: Array2<i8> = cumulative_max(&arr, ViewPoint::Bottom);
        assert_eq!(cum_max[[0, 3]], 9);
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
        let cum_max: Array2<i8> = cumulative_max(&arr, ViewPoint::Left);

        // now use that to figure out which trees are visible
        // first, we grab a view of the original array from the middle
        let relevant_trees = arr.slice(s![1..-1, 1..-1]);
        assert_eq!(relevant_trees[[0, 0]], 5);

        // next, we grab the relevant cumsum slice:
        let relevant_max = cum_max.slice(s![1..-1, 0..-2]);
        assert_eq!(relevant_max[[0, 0]], 2);

        // a tree is visible from the left if its height is larger than the one at the relevant tree
        //let some_output: Array2<bool> = Zip::from(&relevant_max).and(&relevant_trees).map_collect(|&max, &tree| max < tree);
        let some_output = compute_visible_trees_from(&arr, ViewPoint::Left);
        let expected = array![
            [true, false, false],
            [false, false, false],
            [false, true, false]
        ];
        assert_eq!(some_output, expected);

        let other_output = compute_visible_trees_from(&arr, ViewPoint::Bottom);
        let expected = array![
            [false, false, false],
            [false, false, false],
            [false, true, false]
        ];

        assert_eq!(other_output, expected);

        let other_output = compute_visible_trees_from(&arr, ViewPoint::Right);
        let expected = array![
            [false, true, false],
            [true, false, true],
            [false, false, false]
        ];
        assert_eq!(other_output, expected);

        let other_output = compute_visible_trees_from(&arr, ViewPoint::Top);
        let expected = array![
            [true, true, false],
            [false, false, false],
            [false, false, false]
        ];
        assert_eq!(other_output, expected);
    }

    #[test]
    fn test_count_visible() {
        let input = indoc!(
            "
            30373
            25512
            65332
            33549
            35390"
        );

        let arr = digit_matrix_to_array(input);

        let result = count_visible_trees(&arr);
        assert_eq!(result, 21);
    }

    #[test]
    fn tree_vis_hacking() {
        let row = array![2, 5, 5, 1, 2];
        let height_pos = HashMap::new();
        for i in 0..row.len() {
            let height = row[height_pos];
            todo!()
        }
    }
}
