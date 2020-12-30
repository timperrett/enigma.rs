use std::char;
use std::collections::HashMap;
use super::alphabet::with_ordered_index;

/**
 * primary difference here is that `Reflector` has no notch, and is always configured
 * with symetric wiring tables. i.e. A -> Z, Z -> A. Reflectors also do not have
 * configurable rings; they were fixed and came in preset variants: A, B and C
 * Known as Umkehrwalze in german.
 */
struct Reflector {
    wiring: HashMap<char,char>
}

impl Reflector {

    fn transform(&self, c: &char) -> &char {
        self.wiring.get(c).unwrap()
    }

    fn from(s: &str) -> Reflector {
        let map = with_ordered_index(s);
        Reflector { wiring: map }
    }

    fn layout_a() -> Reflector {
        Reflector::from("EJMZALYXVBWFCRQUONTSPIKHGD")
    }
    
    fn layout_b() -> Reflector {
        Reflector::from("YRUHQSLDPXNGOKMIEBFZCWVJAT")
    }

    fn layout_c() -> Reflector {
        Reflector::from("FVPJIAOYEDRZXWGCTKUQSBNMHL")
    }
}
