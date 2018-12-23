/*
Sort

It sorts stuff. What more do you want?
*/

// TODO
// - Turn into a library
// - Make more generic (e. g. the vector generation function
// - Add more sorts:
//   - Gnome Sort
//   - Heap Sort (Smoothsort? Ternary Heap Sort?)
//   - Timsort
//   - Quick Sort (median of medians)
//   - Shellsort?
// - Update Bubble sort implementation
// - Compare results with rust's built in sort method?
// - Post to Github!
// - Add visualization?
// - Allow choice of algorithm as cmd arg

pub enum SortMethod {
    BuiltIn,
    Bubble,
    Insertion,
    Selection,
    Merge,
    Quick
}

pub fn sort<T: num::Integer + Clone + Copy>(v: &mut Vec<T>) {
    sort_with_method(v, SortMethod::Merge)
}

pub fn sort_with_method<T: num::Integer + Clone + Copy>(v: &mut Vec<T>, method: SortMethod) {
    match method {
        SortMethod::BuiltIn => built_in_sort(v),
        SortMethod::Bubble => bubble_sort(v),
        SortMethod::Insertion => insertion_sort(v),
        SortMethod::Selection => selection_sort(v),
        SortMethod::Merge => {
            let sorted = merge_sort(v.clone());
            v.copy_from_slice(&sorted);
        }
        SortMethod::Quick => {
            quick_sort(v);
            //sorted.clone();
        }
    }
}

fn built_in_sort<T: num::Integer>(v: &mut Vec<T>) {
    v.sort();
}

/*
Sorts a vector in place using Bubble Sort.
*/
fn bubble_sort<T: num::Integer>(v: &mut Vec<T>) {
    for _ in 0..v.len() {
        for j in 0..v.len() - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

fn insertion_sort<T: num::Integer>(v: &mut Vec<T>) {
    for i in 1..v.len() {
        for j in (0..i).rev() {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort<T: num::Integer>(v: &mut Vec<T>) {
    for i in 0..v.len() {
        let mut min_index = i;
        for j in i..v.len() {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }

        v.swap(i, min_index);
    }
}

fn merge_sort<T: num::Integer + Copy>(v: Vec<T>) -> Vec<T> {
    if v.len() == 1 {
        return vec![v[0]];
    }

    let m = v.len() / 2;
    let left_half = v[0..m].to_vec();
    let right_half = v[m..v.len()].to_vec();

    let left_sorted = merge_sort(left_half);
    let right_sorted = merge_sort(right_half);

    let mut a = 0;
    let mut b = 0;
    let mut sorted: Vec<T> = vec![];

    while a < left_sorted.len() && b < right_sorted.len() {
        if left_sorted[a] <= right_sorted[b] {
            sorted.push(left_sorted[a]);
            a += 1;
        } else if b < right_sorted.len() {
            sorted.push(right_sorted[b]);
            b += 1;
        }
    }

    // Fill in the remaining items
    for i in a..left_sorted.len() {
        sorted.push(left_sorted[i]);
    }
    for i in b..right_sorted.len() {
        sorted.push(right_sorted[i]);
    }

    sorted
}

fn quick_sort<T>(v: &mut [T])
    where T: num::Integer + Copy
{
    if v.len() == 1 {
        return;
    }

    // Pick Pivot (currently doing it randomly)
    //let med_pos = median_index(&vec![0, v.len()/2, v.len() - 1], &v);
    let med_pos: usize = rand::random::<usize>() % v.len();
    let med = v[med_pos];
    v.swap(med_pos, 0);

    // Partition
    let mut pos_first_greater = 1;

    for i in 1..v.len() {
        if v[i] < med {
            v.swap(i, pos_first_greater);

            pos_first_greater += 1;
        }
    }
    v.swap(0, pos_first_greater - 1);

    // Recursion
    if v.len() > 0 && pos_first_greater > 1 {
        quick_sort(&mut v[..pos_first_greater - 1]);
    }
    if v.len() > 0 && pos_first_greater < v.len() {
        quick_sort(&mut v[pos_first_greater..]);
    }
}

pub fn median_index<T: PartialOrd>(indices: &Vec<usize>, v: &[T]) -> usize {
    let p1 = &v[indices[0]];
    let p2 = &v[indices[1]];
    let p3 = &v[indices[2]];

    if (p1 > p2 && p1 < p3) || (p1 < p2 && p1 > p3) {
        return indices[0];
    } else if (p2 > p1 && p2 < p3) || (p2 < p1 && p2 > p3) {
        return indices[1];
    } else {
        return indices[2];
    }
}