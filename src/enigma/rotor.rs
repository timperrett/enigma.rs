
struct Rotor {
    wiring: String,  // the mapping of normal A->Z letters to its scrambled form
    ring: char, // Ringstellung: offset of the wiring relative to the posistion
    notch: char,
    position: char, // Grundstellung: the position the alphabet ring is currently rotated too
}

impl Rotor {
    // fn new()

    // fn forward(&self, c: char) -> char {
    // }

    // fn backward(&self, c: char) -> char {
    // }
}
