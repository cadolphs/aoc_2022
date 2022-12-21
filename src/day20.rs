use std::ops::Index;

use itertools::Itertools;

pub fn run_day_20(input: String) {
    
    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    //let numbers = vec![1, 2, -3, 3, -2, 0, 4];

    let mut tagged_numbers: Vec<TaggedNumber> = numbers.into_iter()
        .enumerate()
        .map(|x| x.into())
        .collect();

    let mut tagged_numbers_1: CircularVec = tagged_numbers.clone().into();
    
    tagged_numbers_1.mix_once();

    let zero_pos = tagged_numbers_1.storage.iter().find_position(|el| el.val == 0).unwrap().0;
    let ans: i64 = [1000, 2000, 3000].map(|idx| zero_pos as i64 + idx).map(|idx| tagged_numbers_1[idx].val).into_iter().sum();

    println!("The answer is {}", ans);

    tagged_numbers.iter_mut().for_each(|x| x.val *= 811589153);
    let mut tagged_numbers_2: CircularVec = tagged_numbers.into();
    for _ in 0..10 {
        tagged_numbers_2.mix_once();
    }
    let zero_pos = tagged_numbers_2.storage.iter().find_position(|el| el.val == 0).unwrap().0;
    let ans: i64 = [1000, 2000, 3000].map(|idx| zero_pos as i64 + idx).map(|idx| tagged_numbers_2[idx].val).into_iter().sum();

    println!("The answer is {}", ans);

}

#[derive(PartialEq, Clone, Copy)]
struct TaggedNumber {
    val: i64,
    original_pos: usize,
}

impl From<(usize, i64)> for TaggedNumber {
    fn from((original_pos, val): (usize, i64)) -> Self {
        Self { original_pos, val }
    }
}

#[derive(Clone)]
struct CircularVec {
    storage: Vec<TaggedNumber>,
    size: usize,
}

impl From<Vec<TaggedNumber>> for CircularVec {
    fn from(v: Vec<TaggedNumber>) -> Self {
        let size = v.len();
        CircularVec { storage: v, size }
    }
}

impl CircularVec {
    fn swap(&mut self, i: i64, j: i64) {
        let i: usize = i.rem_euclid(self.size as i64) as usize;
        let j: usize = j.rem_euclid(self.size as i64) as usize;
        self.storage.swap(i, j);
    }

    fn best_times_to_swap(&self, val: i64) -> i64 {
        let mod_offset = self.size as i64 - 1;
        let pos_times = val.rem_euclid(mod_offset);
        let neg_times = pos_times - (mod_offset);
    
        if pos_times > neg_times.abs() {
            neg_times
        } else {
            pos_times
        }
    }

    fn mix_once(&mut self) {
        let n = self.size;
        for pos in 0..n {
            let (pos_to_shuffle, item_to_shuffle) = self.storage.iter().find_position(|tn| tn.original_pos == pos).unwrap();
            let times_to_swap = self.best_times_to_swap(item_to_shuffle.val);

            let mut j = pos_to_shuffle as i64;
            if times_to_swap > 0 {
                for _ in 0..times_to_swap {
                    self.swap(j, j + 1);
                    j += 1;
                }
            } else if times_to_swap < 0 {
                for _ in 0..times_to_swap.abs() {
                    self.swap(j, j-1);
                    j -= 1;
                }
            }
        }
    }
}

impl Index<i64> for CircularVec {
    type Output = TaggedNumber;

    fn index(&self, index: i64) -> &Self::Output {
        let index: usize = index.rem_euclid(self.size as i64) as usize;
        &self.storage[index]
    }
}
