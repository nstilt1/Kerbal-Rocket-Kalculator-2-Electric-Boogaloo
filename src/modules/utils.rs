fn exp_taylor(x: f64, terms: usize) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    for n in 1..terms {
        term *= x / n as f64;
        sum += term;
    }
    sum
}

pub fn ln(x: f64) -> f64 {
    assert!(x > 0.0, "Natural log is undefined for non-positive numbers");

    let mut guess = x - 1.0;
    for _ in 0..10 {
        let exp_guess = exp_taylor(guess, 50);
        guess -= (exp_guess - x) / exp_guess;
    }
    (guess * 100_000.0).round() / 100_000.0
}
