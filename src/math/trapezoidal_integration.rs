/// calculates the signed area between the function f and the x axis from
/// x = a to b using a trapezoidal Riemann sum. precision is the number of
/// trapezoids to calculate
pub fn trapezoidal_integral<F>(a: f64, b: f64, f: F, precision: u32) -> f64
where
    F: Fn(f64) -> f64,
{
    fn core<F>(a: f64, b: f64, f: F, precision: u32) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let delta = (b - a) / precision as f64;

        (0..precision)
            .map(|trapezoid| {
                let left_side = a + (delta * trapezoid as f64);
                let right_size = left_side + delta;

                0.5 * (f(left_side) + f(right_size)) * delta
            })
            .sum()
    }

    if a > b {
        -core(b, a, f, precision)
    } else {
        core(a, b, f, precision)
    }
}

fn main() {
    let f = |x: f64| x.powi(3);
    let result = trapezoidal_integral(0.0, 1.0, f, 1000);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integral() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(0.0, 1.0, f, 1000);
        assert!((result - 1.0 / 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_precision() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(0.0, 1.0, f, 10000);
        assert!((result - 1.0 / 3.0).abs() < 0.00001);
    }

    #[test]
    fn test_negative() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(-1.0, 1.0, f, 10000);
        assert!((result - 2.0 / 3.0).abs() < 0.00001);
    }

    #[test]
    fn test_negative_precision() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(-1.0, 1.0, f, 100000);
        assert!((result - 2.0 / 3.0).abs() < 0.000001);
    }
}
