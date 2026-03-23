use aethelgard_sdk::AethelgardV2;

fn main() {
    let mut core = AethelgardV2::new(64, 0.1);
    let dirty_input = 0.85;
    let clean_target = 0.70;

    println!("Tick | Estimate | Error    | Status");
    println!("-------------------------------------------");

    for tick in 1..=101 {
        let (est, err) = core.process_sample(dirty_input, clean_target);
        
        if tick % 10 == 0 || tick == 1 {
            let status = if err.abs() < 0.001 { "STABLE" } else { 
"LEARNING" };
            println!("{:4} | {:.6} | {:.6} | {}", tick, est, err, status);
        }
    }
}

