pub mod alphabet;
pub mod reflector;
pub mod plugboard;
pub mod rotor;

use plugboard::Plugboard;

struct Machine {
    plugboard: Plugboard,
}
