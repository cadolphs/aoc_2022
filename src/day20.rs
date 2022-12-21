use std::ops::Index;

use itertools::Itertools;

pub fn run_day_20(input: String) {
    
    let numbers: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    //let numbers = vec![1, 2, -3, 3, -2, 0, 4];

    let tagged_numbers: Vec<TaggedNumber> = numbers.into_iter()
        .enumerate()
        .map(|x| x.into())
        .collect();

    let mut tagged_numbers: CircularVec<TaggedNumber> = tagged_numbers.into();
    let n = tagged_numbers.size;

    for pos in 0..n {
        let (pos_to_shuffle, item_to_shuffle) = tagged_numbers.storage.iter().find_position(|tn| tn.original_pos == pos).unwrap();
        let times_to_swap = best_times_to_swap(item_to_shuffle.val, n);
        
        let mut j = pos_to_shuffle as i32;
        if times_to_swap > 0 {
            for _ in 0..times_to_swap {
                tagged_numbers.swap(j, j + 1);
                j += 1;
            }
        } else if times_to_swap < 0 {
            for _ in 0..times_to_swap.abs() {
                tagged_numbers.swap(j, j-1);
                j -= 1;
            }
        }
    }

    let zero_pos = tagged_numbers.storage.iter().find_position(|el| el.val == 0).unwrap().0;
    let ans: i32 = [1000, 2000, 3000].map(|idx| zero_pos as i32 + idx).map(|idx| tagged_numbers[idx].val).into_iter().sum();

    println!("The answer is {}", ans);

}

fn best_times_to_swap(val: i32, n: usize) -> i32 {
    let pos_times = val.rem_euclid(n as i32 - 1);
    let neg_times = pos_times - (n as i32 -1);

    if pos_times > neg_times.abs() {
        neg_times
    } else {
        pos_times
    }
}

#[derive(PartialEq, Clone, Copy)]
struct TaggedNumber {
    val: i32,
    original_pos: usize,
}

impl From<(usize, i32)> for TaggedNumber {
    fn from((original_pos, val): (usize, i32)) -> Self {
        Self { original_pos, val }
    }
}

struct CircularVec<T> {
    storage: Vec<T>,
    size: usize,
}

impl<T> From<Vec<T>> for CircularVec<T> {
    fn from(v: Vec<T>) -> Self {
        let size = v.len();
        CircularVec { storage: v, size }
    }
}

impl<T> CircularVec<T> {
    fn swap(&mut self, i: i32, j: i32) {
        let i: usize = i.rem_euclid(self.size as i32) as usize;
        let j: usize = j.rem_euclid(self.size as i32) as usize;
        self.storage.swap(i, j);
    }
}

impl<T> Index<i32> for CircularVec<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        let index: usize = index.rem_euclid(self.size as i32) as usize;
        &self.storage[index]
    }
}
