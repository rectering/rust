use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let data = "Hola, mundo!"; // Cambia esto por tus datos

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let hash_value = hasher.finish();

    println!("Hash de '{}': {}", data, hash_value);
}
