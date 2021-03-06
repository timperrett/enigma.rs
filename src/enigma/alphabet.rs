
use std::char;
use std::collections::HashMap;

// a single sequence of A to Z letters, 26 chars long
fn ordered() -> Vec<char> {
    ('A'..='Z').collect()
}

/**
 * Generates the next letter in the alphabet; if provided Z will yield A.
 * Only expected to work with characters that are A-Z, uppercase only. 
 * Function is guarded by validation higher in the call stack.
 */
fn next_letter(after: char) -> char {
    if after == 'Z' {
        return 'A'
    } else {
        let int = after as u32;
        let nxt = int + 1;
        return char::from_u32(nxt).unwrap() // unwrap with no guard; bit sketchy
    }
}

pub fn with_ordered_index(input: &str) -> HashMap<char,char> {
    let mut mapping: HashMap<char,char> = HashMap::new();
    let alphabet = ordered();
    for (i, v) in input.chars().enumerate(){ // zip with index
        mapping.insert(alphabet[i], v);
    }
    
    mapping
}


// use std::iter;
// use rand::{Rng, thread_rng};
// use rand::distributions::Alphanumeric;

// fn shuffled() -> Vec<char> {
//     let mut rng = thread_rng();
//     let chars = iter::repeat(())
//         .map(|()| rng.sample(Alphanumeric))
//         .take(7)
//         .collect();

//     return chars
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_len() {
        assert_eq!(26, ordered().len());
        assert_eq!(Option::from(&'A'), ordered().first());
        assert_eq!(Option::from(&'Z'), ordered().last());
    }

    #[test]
    fn test_next_letter() {
        assert_eq!('Q', next_letter('P'));
        assert_eq!('A', next_letter('Z'));
    }

    #[test]
    fn test_with_ordered_index() {
        let e1 = hashmap!{
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
        };

        assert_eq!(e1, with_ordered_index("XYZ"));
    }

}