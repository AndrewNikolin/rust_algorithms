use std::cmp::Ordering;
use std::io;

fn main() {
    let mut test = [22, 72, 66, 27, 23, 73, 42, 53, 18, 20];
    let size = test.len();

    insertion_sort(&mut test);

    print_array(&test);

    insertion_sort_reversed(&mut test);

    print_array(&test);

    merge_sort(&mut test, 0, size - 1);

    print_array(&test);
}

fn merge_sort(a: &mut [i32], low: usize, high: usize) {
    if low < high {
        let middle = (low + high) / 2;
        merge_sort(a, low, middle);
        merge_sort(a, middle + 1, high);
        merge(a, low, middle, high);
    }
}

fn merge(a: &mut [i32], low: usize, middle: usize, high: usize) {
    let mut left: usize = low;
    let mut right: usize = middle + 1;

    let mut tmp: Vec<i32> = vec![0; high - low + 1];
    let mut index = 0;

    while left <= middle && right <= high {
        if a[left] < a[right] {
            tmp[index] = a[left];
            left += 1;
        }
        else {
            tmp[index] = a[right];
            right += 1;
        }
        index += 1;
    }

    for i in left..=middle {
        tmp[index] = a[i];
        index += 1;
    }

    for i in right..=high {
        tmp[index] = a[i];
        index += 1;
    }

    for i in 0..tmp.len() {
        a[low+i] = tmp[i];
    }
}

fn insertion_sort(a: &mut [i32]) {
    for j in 1..a.len() as i32 {
        let key = a[j as usize];
        let mut i = j - 1;
        while i >= 0 && a[i as usize] > key {
            a[(i+1) as usize] = a[i as usize];
            i = i - 1;
        }
        a[(i+1) as usize] = key;
    }
}

fn insertion_sort_reversed(a: &mut [i32]) {
    for j in 1..a.len() as i32 {
        let key = a[j as usize];
        let mut i = j - 1;
        while i >= 0 && a[i as usize] < key {
            a[(i+1) as usize] = a[i as usize];
            i = i - 1;
        }
        a[(i+1) as usize] = key;
    }
}

fn print_array(a: &[i32]) {
    println!();

    for x in a {
        print!("{} ", x);
    }
}