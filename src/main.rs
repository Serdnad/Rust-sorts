extern crate num;
extern crate rand;

use std::env;
use std::vec;

mod sort;

fn main() {
    const DEFAULT_SIZE: usize = 10;
    const DEFAULT_ALG: sort::SortMethod = sort::SortMethod::Quick;

    // Hacky code that takes the vector size as the last CLI arg or defaults to 10
    let mut size = DEFAULT_SIZE;
    let mut sort_alg = DEFAULT_ALG;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        size = args[1].parse().expect("Error - Expected array size (int) as first arg!");
    }

    if args.len() >= 3 {
        sort_alg = match &args[2][..] {
            "" => DEFAULT_ALG,
            "built_in" => sort::SortMethod::BuiltIn,
            "quick" => sort::SortMethod::Quick,
            "merge" => sort::SortMethod::Merge,
            "bubble" => sort::SortMethod::Bubble,
            "insertion" => sort::SortMethod::Insertion,
            "selection" => sort::SortMethod::Selection,
            _ => sort::SortMethod::BuiltIn
        };
    }

    println!("Size: {}", size);

    // Generate numbers
    let mut nums = rand_vec(size);
    //nums = vec![4, 3, 2, 6, 1, 5];
    //nums = vec![1, 5, 3, 4, 2];
    //nums = vec![2, 1, 3, 4];

    //print_vec(&nums);

    sort::sort_with_method(&mut nums, sort_alg);

    //print_vec(&nums);

    //println!("{}", sort::median_index(&vec![0, 1, 2], &vec![3, 4, 5]));


    // Verify result is sorted
    match is_sorted(&nums) {
        true => println!("Sorted properly!"),
        false => println!("Not sorted!!"),
    }

}

fn is_sorted<T: num::Integer>(nums: &Vec<T>) -> bool {
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            return false;
        }
    }

    true
}

fn rand_vec(size: usize) -> Vec<u8> {
    let mut v = vec![];
    for _ in 0..size {
        let r = rand::random::<u8>();
        v.push(r);
    }

    v
}

fn sorted_vec(size: usize) -> Vec<u8> {
    let mut v = vec![];
    for i in 0..size {
        let val: u8 = (i / size) as u8 * std::u8::MAX;
        v.push(val);
    }

    v
}

fn print_vec<T: std::fmt::Debug>(v: &Vec<T>) {
    for i in v {
        print!("{:?} ", i);
    }
    println!();
}