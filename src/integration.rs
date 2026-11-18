const MAX_ITERATIONS: usize = 20;

pub struct VariableStepResult {
    pub integral: f64,
    pub n: usize,
    pub iterations: usize,
    pub converged: bool,
    pub evaluations: usize,
}

pub fn task_function(x: f64) -> f64 {
    (0.8 * x + 1.2).cos() / (1.5 + (x * x + 0.6).sin())
}

pub fn left_rectangle(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;

    for i in 0..n {
        let x = a + i as f64 * h;
        sum += f(x);
    }

    sum * h
}

pub fn right_rectangle(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;

    for i in 1..=n {
        let x = a + i as f64 * h;
        sum += f(x);
    }

    sum * h
}

pub fn trapezoid(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));

    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x);
    }

    sum * h
}

pub fn simpson(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        let k = if i % 2 == 1 {4.0} else {2.0};
        sum += k * f(x);
    }

    sum * h / 3.0
}

pub fn left_rect_var_step_alg1(f: fn(f64) -> f64, a: f64, b: f64, n0: usize, eps: f64) -> VariableStepResult {
    let mut i_n = 0.0;
    let mut n = n0;
    let mut evaluations = 0;
    let mut iter = 0;

    loop {
        let i2_n = left_rectangle(f, a, b, n);
        evaluations += n;
        iter += 1;
        if iter >= 2 && (i_n - i2_n).abs() <= eps {
            break VariableStepResult {integral: i2_n, n, iterations: iter, converged: true, evaluations};
        } else if iter >= MAX_ITERATIONS {
            break VariableStepResult {integral: i2_n, n, iterations: iter, converged: false, evaluations};
        }
        i_n = i2_n;
        n *= 2;
    }
}

pub fn left_rect_var_step_alg2(f: fn(f64) -> f64, a: f64, b: f64, n0: usize, eps: f64) -> VariableStepResult {
    let mut n = n0;
    let mut h = (b - a) / n as f64;

    let mut sum = 0.0;
    for i in 0..n {
        sum += f(a + i as f64 * h);
    }
    let mut i_n = sum * h;
    let mut evaluations = n;
    let mut iter = 1;

    loop {
        for i in 0..n {
            let x = a + h / 2.0 + i as f64 * h;
            sum += f(x);
        }
        evaluations += n;

        h /= 2.0;
        n *= 2;
        iter += 1;

        let i2_n = sum * h;

        if (i_n - i2_n).abs() <= eps {
            break VariableStepResult{integral: i2_n, n, iterations: iter, converged: true, evaluations};
        } else if iter >= MAX_ITERATIONS {
            break VariableStepResult{integral: i2_n, n, iterations: iter, converged: false, evaluations};
        }
        i_n = i2_n;
    }
}