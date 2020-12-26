pub mod alphabet;

use std::collections::HashMap;

struct Machine {
    plugboard: Plugboard,
}

/**
 * Here the signal is connected to the 'T' input on the plugboard. Some of
 * the letters on the plugboard will be wired up to other letters (the plugs),
 * causing the signal to be diverted. If the 'T' input is not plugged to another
 * letter then our signal will pass straight to the 'T output. In our case, though
 * the 'T' is plugged to the 'K', so the signal is diverted to a new path, the
 * letter is now 'K'.
 */
struct Plugboard {
    mapped: HashMap<char,char>
}

impl Plugboard {

    fn from_alphabet(input: Vec<char>){
    }

    fn from_mapping(m: HashMap<char, char>){
    }

    fn transform(&self, c: &char) -> &char {
        self.mapped.get(c).unwrap()
    }
}