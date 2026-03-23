use aethelgard_sdk::AethelgardV2;

fn main() {
    let mut core = AethelgardV2::new(64, 0.05);
    let (estimate, error) = core.process_sample(0.85, 0.82);
    println!("Aethelgard V2.1 Output -> Est: {}, Err: {}", estimate, 
error);
}

