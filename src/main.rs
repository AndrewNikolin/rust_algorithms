use std::cmp::Ordering;
use std::io;

fn main() {
    let mut test = [22, 72, 66, 27, 23, 73, 42, 53, 18, 20];

    insertion_sort(&mut test);

    print_array(&test);

    insertion_sort_reversed(&mut test);

    print_array(&test);
}

fn insertion_sort(A: &mut [i32]) {
    for j in 1..A.len() as i32 {
        let key = A[j as usize];
        let mut i = j - 1;
        while i >= 0 && A[i as usize] > key {
            A[(i+1) as usize] = A[i as usize];
            i = i - 1;
        }
        A[(i+1) as usize] = key;
    }
}

fn insertion_sort_reversed(A: &mut [i32]) {
    for j in 1..A.len() as i32 {
        let key = A[j as usize];
        let mut i = j - 1;
        while i >= 0 && A[i as usize] < key {
            A[(i+1) as usize] = A[i as usize];
            i = i - 1;
        }
        A[(i+1) as usize] = key;
    }
}

fn print_array(A: &[i32]) {
    println!();

    for x in A {
        print!("{} ", x);
    }
}