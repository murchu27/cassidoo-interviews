fn main() {
    let soln: &str = "fudge";
    println!("[Psst! The solution is \"{}\"!]", soln);

    let guess1: &str = "reads";
    let guess1_result: String = wordleGuess(guess1, soln);
    println!("You guessed:\n{}\n{}", guess1, guess1_result);

    let guess2: &str = "lodge";
    let guess2_result: String = wordleGuess(guess2, soln);
    println!("You guessed:\n{}\n{}", guess2, guess2_result);
}

#[allow(non_snake_case)]
fn wordleGuess(guess: &str, soln: &str) -> String {
    // words in Wordle have 5 characters
    const L: usize = 5;
    assert_eq!(L, guess.len());
    assert_eq!(L, soln.len());

    // start with an array of "black square emoji" chars
    let mut r: [char; L] = ['⬛'; L];
    let mut used_indices: [bool; L] = [false; L];

    // first, pass through guess and look for exact matches
    for i in 0..L {
        if guess.chars().nth(i) == soln.chars().nth(i) {
            // on a match, change to a "green square emoji"
            r[i] = '🟩';

            // keep track of which chars have already been matched
            used_indices[i] = true;
        }
    }

    // second, pass through guess and look for matches in wrong position
    for i in 0..L {
        // pass through soln to see if ith char of guess is in soln anywhere
        for j in 0..L {
            // skip any characters that have been matched on the first pass
            // or that have already been used for a wrong-position match
            if used_indices[j] {
                continue;
            }
            if guess.chars().nth(i) == soln.chars().nth(j) {
                // on a match, change to a "yellow square emoji"
                r[i] = '🟨';

                // keep track of which chars been used for a wrong-position match
                used_indices[j] = true;
            }
        }
    }

    r.iter().collect()
}
