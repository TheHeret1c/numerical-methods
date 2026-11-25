pub type Rhs = fn(f64, &[f64]) -> Vec<f64>;

pub fn example1(x: f64, y: &[f64]) -> Vec<f64> {
    vec![y[0] * (1.0 - x)]
}

pub fn exact1(x: f64, a: f64, y0: f64) -> f64 {
    y0 * ((x - x * x / 2.0) - (a - a * a / 2.0)).exp()
}

pub fn euler(f: Rhs, x0: f64, y0: &[f64], h: f64, n: usize) -> Vec<(f64, Vec<f64>)> {
    let mut x = x0;
    let mut y = y0.to_vec();
    let mut result = vec![(x, y.clone())];

    for i in 0..n {
        let slope = f(x, &y);
        for j in 0..y.len() {
            y[j] += h * slope[j];
        }
        x = x0 + (i + 1) as f64 * h;
        result.push((x, y.clone()));
    }

    result
}

pub fn runge_kutta(f: Rhs, x0: f64, y0: &[f64], h: f64, n: usize) -> Vec<(f64, Vec<f64>)> {
    let mut x = x0;
    let mut y = y0.to_vec();
    let mut result = vec![(x, y.clone())];

    for i in 0..n {
        let k1 = f(x, &y);
        let k2 = f(x + h / 2.0, &add_scaled(&y, &k1, h / 2.0));
        let k3 = f(x + h / 2.0, &add_scaled(&y, &k2, h / 2.0));
        let k4 = f(x + h, &add_scaled(&y, &k3, h));

        for j in 0..y.len() {
            y[j] += h * (k1[j] + 2.0 * k2[j] + 2.0 * k3[j] + k4[j]) / 6.0;
        }
        x = x0 + (i + 1) as f64 * h;
        result.push((x, y.clone()));
    }

    result
}

fn add_scaled(y: &[f64], k: &[f64], c: f64) -> Vec<f64> {
    let mut res = y.to_vec();

    for i in 0..res.len() {
        res[i] += k[i] * c;
    }

    res
}

pub fn example2(x: f64, y: &[f64]) -> Vec<f64> {
    vec![y[1], -y[1] / x - y[0]]
}

pub fn example3(t: f64, v: &[f64]) -> Vec<f64> {
    vec![
        -2.0 * v[0] + 5.0 * v[2],
        (t - 1.0).sin() * v[0] - v[1] + 3.0 * v[2],
        -v[0] + 2.0 * v[2],
    ]
}