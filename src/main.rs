use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut test = [22, 72, 66, 27, 23, 73, 42, 53, 18, 20];

    insertion_sort(&mut test);

    for x in &test {
        print!("{} ", x);
    }
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

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}