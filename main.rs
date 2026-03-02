fn series_sum(n: u32) -> String 
{
    let sum: f64 = (0..n)
        .map(|i| 1.0 / (1.0 + (i as f64 * 3.0)))
        .sum();

    format!("{:.2}", sum)
}
