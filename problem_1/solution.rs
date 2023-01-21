fn main() {
    
    let n = 1000;
    let rng = 1..n;
    let total = rng
        .filter(|e| (e%3==0)||(e%5==0))
        .sum::<i32>();
    
    println!("{}",total);
}