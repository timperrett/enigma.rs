
use std::char;
use std::collections::HashMap;

/**
 * Here the signal is connected to the 'T' input on the plugboard. Some of
 * the letters on the plugboard will be wired up to other letters (the plugs),
 * causing the signal to be diverted. If the 'T' input is not plugged to another
 * letter then our signal will pass straight to the 'T output. In our case, though
 * the 'T' is plugged to the 'K', so the signal is diverted to a new path, the
 * letter is now 'K'.
 */
pub struct Plugboard {
    mapped: HashMap<char,char>
}

impl Plugboard {

    fn from_alphabet(input: Vec<char>) -> Plugboard {
        Plugboard { mapped: HashMap::new() }
    }

    fn from_mapping(input: HashMap<char, char>) -> Plugboard {
        Plugboard { mapped: input }
    }

    fn transform(&self, c: &char) -> &char {
        self.mapped.get(c).unwrap()
    }
}