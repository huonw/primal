fn main() {
    let ns = (1..100 + 1).map(|x| x * 100_000).collect::<Vec<_>>();

    // now we can efficiently sum them up
    let sum = ns.iter()
                .map(|n| primal::StreamingSieve::nth_prime(*n))
                .fold(0u64, |a, b| a + b as u64);
    println!("the sum is {}", sum);
}
