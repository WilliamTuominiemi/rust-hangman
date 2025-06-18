use std::io;

fn main() {
    let target: &str = "HANGMAN";
    let mut anti_target: String = "*******".to_string();

    let mut guesses_left = 14;

    let mut word_guessed = false;

    while guesses_left > 0 && !word_guessed {
        std::process::Command::new("clear").status().unwrap();
        println!("Guess the word: {}", anti_target);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let guess = input
                    .trim()
                    .chars()
                    .next()
                    .unwrap_or_default()
                    .to_ascii_uppercase();

                // Find all matches in target word for the guessed character
                let indexes = target
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == guess)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                // For each match, show that letter in the anti target
                for index in indexes {
                    let mut anti_target_chars = anti_target.chars().collect::<Vec<_>>();
                    anti_target_chars[index] = guess;
                    anti_target = anti_target_chars.iter().collect::<String>();
                }

                // If there aren't any hidden letters left, user has guessed the word
                if !anti_target.contains("*") {
                    word_guessed = true;
                }
            }
            Err(error) => println!("error: {error}"),
        }

        guesses_left -= 1;
    }

    if word_guessed {
        println!("You guessed the word")
    } else {
        println!("You ran out of guesses");
    }
}
