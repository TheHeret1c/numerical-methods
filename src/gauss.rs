pub fn forward_pass_classic(m: &mut Vec<Vec<f64>>) -> Result<(), String> {
    for k in 0..m.len() {
        if m[k][k].abs() < 1e-12 {
            return Err("Нулевой ведущий элемент: система вырождена или требует перестановки строк".to_string());
        }
        let p = m[k][k];
        for j in k..=m.len() {
            m[k][j] /= p;
        }
        for i in (k + 1)..m.len() {
            let c = m[i][k];
            for j in 0..=m.len() {
                m[i][j] -= c * m[k][j];
            }
        }
    }
    Ok(())
}

pub fn forward_pass(m: &mut Vec<Vec<f64>>) -> Result<(), String> {
    for k in 0..m.len() {
        if m[k][k].abs() < 1e-12 {
            return Err("Нулевой ведущий элемент: система вырождена или требует перестановки строк".to_string());
        }
        for i in (k + 1)..m.len() {
            let c = m[i][k] / m[k][k];
            for j in 0..=m.len() {
                m[i][j] -= c * m[k][j];
            }
        }
    }
    Ok(())
}

pub fn backward_pass(m: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut x = vec![0.0; m.len()];
    for i in (0..m.len()).rev() {
        let mut s = m[i][m.len()];
        for j in (i + 1)..m.len() {
            s -= m[i][j] * x[j];
        }
        x[i] = s / m[i][i];
    }
    x
}

pub fn gauss_jordan(m: &mut Vec<Vec<f64>>) -> Result<(), String> {
    for k in 0..m.len() {
        if m[k][k].abs() < 1e-12 {
            return Err("Нулевой ведущий элемент: система вырождена или требует перестановки строк".to_string());
        }
        let p = m[k][k];
        for j in k..=m.len() {
            m[k][j] /= p;
        }
        for i in 0..m.len() {
            if i == k {
                continue;
            }
            let c = m[i][k];
            for j in 0..=m.len() {
                m[i][j] -= c * m[k][j];
            }
        }
    }
    Ok(())
}