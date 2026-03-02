# CodeWars-Sum-of-the-first-nth-term-of-Series-7-kyu---Passed
Your task is to write a function which returns the n-th term of the following series, which is the sum of the first n terms of the sequence (n is the input parameter).
Task
Your task is to write a function which returns the n-th term of the following series, which is the sum of the first n terms of the sequence (n is the input parameter).

You will need to figure out the rule of the series to complete this.

Rules
You need to round the answer to 2 decimal places and return it as String.

If the given value is 0 then it should return "0.00".

You will only be given Natural Numbers as arguments.

Examples (Input --> Output)
n
1 --> 1 --> "1.00"
2 --> 1 + 1/4 --> "1.25"
5 --> 1 + 1/4 + 1/7 + 1/10 + 1/13 --> "1.57"

TEST CASES:
#[cfg(test)]
mod tests {
    use super::series_sum;
    use rand::{thread_rng, Rng};
    
    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(actual == expected, "Expected series_sum({input}) to be {expected}, but was {actual}");
    }

    #[test]
    fn test_basic() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(4, "1.49");
        test(5, "1.57");
        test(6, "1.63");
        test(7, "1.68");
        test(8, "1.73");
        test(9, "1.77");
        test(15, "1.94");
        test(39, "2.26");
        test(58, "2.40");
    }
    
    #[test]
    fn test_edge_case() {
        test(0, "0.00");
    }
    
    #[test]
    fn test_random() {
        fn solution(n: u32) -> String {
            let sum: f64 = (0..n).map(|k| 1.0 / (k as f64 * 3.0 + 1.0)).sum();
            format!("{sum:.2}")
        }
        
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            // excluding 0 to make it clear when edge case fails
            let n = rng.gen_range(1..200);
            test(n, &solution(n));
        }
    }
}
