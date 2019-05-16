extern crate rand;

use rand::Rng;

fn main() {
    
    let winning_guess: Vec<u8> = vec![7, 10, 14, 34, 36, 37];
    let max = 49;
    let length = winning_guess.len() as u8;

    let mut rng = rand::thread_rng();
    let mut guesses = 0;
    let mut guess: Vec<u8> = vec![0; length as usize];


    loop {
        let mut correct = false;
        guesses += 1;

        for num in &mut guess {
            *num = rng.gen_range(0, max);
        }

        while correct == false {
            guess.sort();
            correct = true;
            for i in 0..(length - 1) {
                let mut index = i as usize;
                match guess[index] == guess[index + 1] {
                    true => {
                        guess[index] = rng.gen_range(0, max);
                        correct = false;
                        break
                    },
                    false => correct = true
                }
            }
        }
        
        guess.sort();

        if guess == winning_guess {
            println!("{} Guesses", guesses);
            println!("WINNING GUESS!");
            break
        }
    }
}
