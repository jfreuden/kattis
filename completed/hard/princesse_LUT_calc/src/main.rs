fn calculate_k(penalty_value: u32, starting_k: f64) -> f64 {
    let left_side = (penalty_value + 1) as f64;
    let mut k = starting_k;

    while left_side > k.ln() / (1.0f64 - k).ln() {
        k -= 0.0000000001f64;
        // k -= (f32::MIN_POSITIVE * 4096.0f32) as f64;

    }
    k
}

fn main() {
    println!("const SPLIT_LUT: [f32; 1001] = [");

    // S + 1 = ln k / ln (1-k)
    // For each S from 0 to 1000, calculate K for a LUT.
    let mut last_value = 0.5f64;
    for penalty in 0..=1000 {
        last_value = calculate_k(penalty, last_value);

        println!("{}f32, ", last_value as f32);
    }
    println!("];");
}