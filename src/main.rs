use rand::seq::IteratorRandom;

const VOWELS: [char; 8] = ['َ', 'ً', 'ِ', 'ٍ', 'ُ', 'ٌ', 'ْ', 'ّ'];
const HAMZA: char = 'ء';
const TAA_M: char = 'ة';
const ALIPH: [char; 4] = ['ا', 'آ', 'ر', 'أ'];

fn main() {
    let input = include_str!("rafii.txt")
        .chars()
        .filter(|c| !VOWELS.contains(c))
        .collect::<String>();

    let processing = shuffle_text(&input);

    std::fs::write("output_test/output.txt", processing).expect("Unable to write file");
}

fn shuffle_text(text: &str) -> String {
    let mut letter_number: usize = 0;
    let mut taa_flag = false;

    let result = text
        .chars()
        .map(|c| {
            if c.is_alphabetic() && !taa_flag {
                letter_number += 1;
                let mut attempt_c = choose_alphabetic_char(text);


                // This tries to avoid the letters HAMZA and TAA_M from appearing as
                // the first or second letter. Also prevents the permutations of ALIPH
                // to appear in consecutive places, tho this probably halfs the frequency.
                while (letter_number < 3 && (attempt_c == HAMZA || attempt_c == TAA_M))
                    || (letter_number % 2 == 0 && ALIPH.contains(&attempt_c))
                {
                    attempt_c = choose_alphabetic_char(text);
                }

                taa_flag = attempt_c == HAMZA || attempt_c == TAA_M;

                attempt_c
            } else if !c.is_alphabetic() {
                letter_number = 0;
                taa_flag = false;
                c
            } else {
                letter_number = 0;
                taa_flag = false;
                ' '
            }
        })
        .collect();
    result
}

fn choose_alphabetic_char(text: &str) -> char {
    let attempt_c = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .choose(&mut rand::thread_rng())
        .unwrap_or('ظ');
    attempt_c
}
