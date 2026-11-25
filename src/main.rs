mod integration;
mod ode;
mod gauss;

use eframe::egui;
use crate::gauss::{backward_pass, forward_pass, forward_pass_classic, gauss_jordan};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TaskType {
    #[default]
    Integration,
    DiffEq,
    LinearSystem,
}

impl TaskType {
    fn label(self) -> &'static str {
        match self {
            TaskType::Integration => "Численное интегрирование",
            TaskType::DiffEq => "Дифференциальные уравнения",
            TaskType::LinearSystem => "Системы линейных уравнений"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum NumericalMethod {
    #[default]
    LeftRectangle,
    RightRectangle,
    Trapezoid,
    Simpson,
}

impl NumericalMethod {
    fn label(self) -> &'static str {
        match self {
            NumericalMethod::LeftRectangle => "Прямоугольников левых частей",
            NumericalMethod::RightRectangle => "Прямоугольников правых частей",
            NumericalMethod::Trapezoid => "Трапеций",
            NumericalMethod::Simpson => "Парабол",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Algorithm {
    #[default]
    ConstantStep,
    VariableStep,
}

impl Algorithm {
    fn label(self) -> &'static str {
        match self {
            Algorithm::ConstantStep => "С постоянным шагом",
            Algorithm::VariableStep => "С переменным шагом",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum OdeMethod {
    #[default]
    Euler,
    RungeKutta,
    Compare,
    Tune,
}

impl OdeMethod {
    fn label(self) -> &'static str {
        match self {
            OdeMethod::Euler => "Эйлера",
            OdeMethod::RungeKutta => "Рунге-Кутта",
            OdeMethod::Compare => "Сравнение методов",
            OdeMethod::Tune => "Подбор n для Эйлера",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum OdeProblem {
    #[default]
    Example1,
    Example2,
    Example3,
}

impl OdeProblem {
    fn label(self) -> &'static str {
        match self {
            OdeProblem::Example1 => "Пример 1: y' = y(1 - x)",
            OdeProblem::Example2 => "Пример 2: y'' + y'/x + y = 0",
            OdeProblem::Example3 => "Пример 3: система из трёх уравнений",
        }
    }

    fn rhs(self) -> ode::Rhs {
        match self {
            OdeProblem::Example1 => ode::example1,
            OdeProblem::Example2 => ode::example2,
            OdeProblem::Example3 => ode::example3,
        }
    }

    fn arg(self) -> &'static str {
        match self {
            OdeProblem::Example3 => "t",
            _ => "x",
        }
    }

    fn names(self) -> &'static [&'static str] {
        match self {
            OdeProblem::Example1 => &["y"],
            OdeProblem::Example2 => &["y", "y'"],
            OdeProblem::Example3 => &["x", "y", "z"],
        }
    }

    fn defaults(self) -> (&'static str, &'static str, &'static [&'static str]) {
        match self {
            OdeProblem::Example1 => ("0", "1", &["1"]),
            OdeProblem::Example2 => ("1", "1.5", &["0.77", "-0.44"]),
            OdeProblem::Example3 => ("0", "0.3", &["2", "1", "1"]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum GaussMethod {
    #[default]
    Classic,
    Optimal,
    GaussJordan
}

impl GaussMethod {
    fn label(self) -> &'static str {
        match self {
            GaussMethod::Classic => "Гаусс",
            GaussMethod::Optimal => "Оптимальное исключение",
            GaussMethod::GaussJordan => "Гаусс-Жордан",
        }
    }
}

impl Default for MathApp {
    fn default() -> Self {
        Self {
            task: TaskType::default(),
            method: NumericalMethod::default(),
            algorithm: Algorithm::default(),
            n_input: String::new(),
            partitions: Vec::new(),
            a_input: "0.3".to_string(),
            b_input: "0.9".to_string(),
            results: Vec::new(),
            n0_input: "10".to_string(),
            eps_input: "0.001".to_string(),
            var_results: None,
            ode_n_input: "10".to_string(),
            ode_results: Vec::new(),
            ode_a_input: "0".to_string(),
            ode_b_input: "1".to_string(),
            ode_y0_inputs: vec!["1".to_string()],
            ode_problem: OdeProblem::default(),
            ode_method: OdeMethod::default(),
            ode_results_rk: Vec::new(),
            ode_exact: Vec::new(),
            ode_n_list_input: String::new(),
            ode_n_list: Vec::new(),
            ode_tune: Vec::new(),
            ode_tune_ref: None,
            // СЛАУ
            gauss_method: GaussMethod::default(),
            gauss_n: 4,
            gauss_a: vec![
                vec!["5".into(), "7".into(), "6".into(), "5".into()],
                vec!["7".into(), "10".into(), "8".into(), "7".into()],
                vec!["6".into(), "8".into(), "10".into(), "9".into()],
                vec!["5".into(), "7".into(), "9".into(), "10".into()],
            ],
            gauss_b: vec!["23".into(), "32".into(), "33".into(), "31".into()],
            gauss_error: None,
            gauss_triangle: None,
            gauss_x: None,
            gauss_delta: "0".to_string(),
        }
    }
}
struct MathApp {
    task: TaskType,
    method: NumericalMethod,
    algorithm: Algorithm,
    n_input: String,
    partitions: Vec<usize>,
    a_input: String,
    b_input: String,
    results: Vec<(usize, f64)>,
    n0_input: String,
    eps_input: String,
    var_results: Option<(integration::VariableStepResult, integration::VariableStepResult)>,
    // Дифуры
    ode_n_input: String,
    ode_results: Vec<(f64, Vec<f64>)>,
    ode_a_input: String,
    ode_b_input: String,
    ode_y0_inputs: Vec<String>,
    ode_problem: OdeProblem,
    ode_method: OdeMethod,
    ode_results_rk: Vec<(f64, Vec<f64>)>,
    ode_exact: Vec<f64>,
    ode_n_list_input: String,
    ode_n_list: Vec<usize>,
    ode_tune: Vec<(usize, Vec<f64>)>,
    ode_tune_ref: Option<(usize, Vec<f64>, Option<f64>)>,
    // СЛАУ
    gauss_method: GaussMethod,
    gauss_n: usize,
    gauss_a: Vec<Vec<String>>,
    gauss_b: Vec<String>,
    gauss_error: Option<String>,
    gauss_triangle: Option<Vec<Vec<f64>>>,
    gauss_x: Option<Vec<f64>>,
    gauss_delta: String,
}


impl eframe::App for MathApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.algorithm == Algorithm::VariableStep {
            self.method = NumericalMethod::LeftRectangle;
        }

        let old_method = self.method;
        let old_algorithm = self.algorithm;
        let old_ode_method = self.ode_method;
        let old_ode_problem = self.ode_problem;
        let old_gauss_method = self.gauss_method;



        egui::SidePanel::left("menu").show(ctx, |ui| {
            ui.heading("Меню");

            ui.strong("1. Тип задачи");
            ui.radio_value(&mut self.task, TaskType::Integration, TaskType::Integration.label());
            ui.radio_value(&mut self.task, TaskType::DiffEq, TaskType::DiffEq.label());
            ui.radio_value(&mut self.task, TaskType::LinearSystem, TaskType::LinearSystem.label());

            ui.add_space(10.0);
            if self.task == TaskType::Integration {
                ui.strong("2. Численный метод");
                ui.radio_value(&mut self.method, NumericalMethod::LeftRectangle, NumericalMethod::LeftRectangle.label());
                ui.add_enabled_ui(self.algorithm == Algorithm::ConstantStep, |ui| {
                    ui.radio_value(&mut self.method, NumericalMethod::RightRectangle, NumericalMethod::RightRectangle.label());
                    ui.radio_value(&mut self.method, NumericalMethod::Trapezoid, NumericalMethod::Trapezoid.label());
                    ui.radio_value(&mut self.method, NumericalMethod::Simpson, NumericalMethod::Simpson.label());
                });

                ui.add_space(10.0);
                ui.strong("3. Алгоритм");
                ui.radio_value(&mut self.algorithm, Algorithm::ConstantStep, Algorithm::ConstantStep.label());
                ui.radio_value(&mut self.algorithm, Algorithm::VariableStep, Algorithm::VariableStep.label());
            } else if self.task == TaskType::DiffEq {
                ui.strong("2. Задача");
                ui.radio_value(&mut self.ode_problem, OdeProblem::Example1, OdeProblem::Example1.label());
                ui.radio_value(&mut self.ode_problem, OdeProblem::Example2, OdeProblem::Example2.label());
                ui.radio_value(&mut self.ode_problem, OdeProblem::Example3, OdeProblem::Example3.label());

                ui.add_space(10.0);
                ui.strong("3. Численный метод");
                ui.radio_value(&mut self.ode_method, OdeMethod::Euler, OdeMethod::Euler.label());
                ui.radio_value(&mut self.ode_method, OdeMethod::RungeKutta, OdeMethod::RungeKutta.label());
                ui.radio_value(&mut self.ode_method, OdeMethod::Compare, OdeMethod::Compare.label());
                ui.radio_value(&mut self.ode_method, OdeMethod::Tune, OdeMethod::Tune.label());
            } else if self.task == TaskType::LinearSystem {
                ui.strong("2. Метод");
                ui.radio_value(&mut self.gauss_method, GaussMethod::Classic, GaussMethod::Classic.label());
                ui.radio_value(&mut self.gauss_method, GaussMethod::Optimal, GaussMethod::Optimal.label());
                ui.radio_value(&mut self.gauss_method, GaussMethod::GaussJordan, GaussMethod::GaussJordan.label());
            }
        });

        if self.ode_problem != old_ode_problem {
            self.reset_ode_inputs();
        }

        if self.ode_method != old_ode_method || self.ode_problem != old_ode_problem {
            self.ode_results.clear();
            self.ode_results_rk.clear();
            self.ode_exact.clear();
            self.ode_tune.clear();
            self.ode_tune_ref = None;
        }

        if self.method != old_method || self.algorithm != old_algorithm {
            self.results.clear();
            self.var_results = None;
        }

        if self.gauss_method != old_gauss_method {
            self.clear_gauss_results();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.task {
                TaskType::Integration => self.show_integration(ui),
                TaskType::DiffEq => self.show_ode(ui),
                TaskType::LinearSystem => self.show_gauss(ui),
            }

        });
    }
}

impl MathApp {
    fn show_integration(&mut self, ui: &mut egui::Ui) {
        ui.heading("Численное интегрирование");

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Пределы интегрирования: a =");
            ui.add(egui::TextEdit::singleline(&mut self.a_input).desired_width(80.0));
            ui.label("b =");
            ui.add(egui::TextEdit::singleline(&mut self.b_input).desired_width(80.0));
        });

        let mut limits_ok = false;

        match (parse_number(&self.a_input), parse_number(&self.b_input)) {
            (Ok(a), Ok(b)) if a < b => {
                limits_ok = true;
            }
            (Ok(_), Ok(_)) => {
                ui.colored_label(egui::Color32::RED, "Нижний предел должен быть меньше верхнего");
            }
            _ => {
                ui.colored_label(egui::Color32::RED, "Пределы должны быть числами");
            }
        }

        let mut has_odd = false;
        let mut var_ok = false;

        if self.algorithm == Algorithm::ConstantStep {
            ui.horizontal(|ui| {
                ui.label("Число разбиений отрезка");
                ui.add(egui::TextEdit::singleline(&mut self.n_input).desired_width(80.0));

                if ui.button("Добавить").clicked() {
                    if let Ok(n) = self.n_input.trim().parse::<usize>() {
                        if n > 0 && !self.partitions.contains(&n) {
                            self.partitions.push(n);
                            self.partitions.sort();
                        }
                        self.n_input.clear();
                    }
                }
            });

            ui.add_space(10.0);

            if self.n_input.trim().is_empty() {
                ui.label("Введите число разбиений");
            } else {
                match self.n_input.trim().parse::<usize>() {
                    Ok(0) => {
                        ui.colored_label(egui::Color32::RED, "Число разбиений должно быть больше нуля!");
                    }
                    Ok(n) => {
                        ui.label(format!("Будет использовано разбиений: {n}"));
                    }
                    Err(_) => {
                        ui.colored_label(egui::Color32::RED, "Введите целое положительное число");
                    }
                }
            }

            if self.partitions.is_empty() {
                ui.label("Список разбиений пуст");
            } else {
                ui.horizontal_wrapped(|ui| {
                    //ui.label(format!("Разбиения: {:?}", self.partitions));

                    let mut to_remove = None;

                    for (i, n) in self.partitions.iter().enumerate() {
                        let mut text = egui::RichText::new(format!("{n} \u{1F5D9}"));
                        if self.method == NumericalMethod::Simpson && n % 2 != 0 {
                            text = text.color(egui::Color32::RED);
                            has_odd = true;
                        }
                        if ui.button(text).clicked() {
                            to_remove = Some(i);
                        }
                    }

                    if has_odd {
                        ui.colored_label(egui::Color32::RED, "Для метода парабол число разбиений должно быть чётным");
                    }

                    if let Some(i) = to_remove {
                        self.partitions.remove(i);
                    }

                    if ui.button("Очистить").clicked() {
                        self.partitions.clear();
                    }

                });
            }
        } else {
            ui.horizontal(|ui| {
                ui.label("Начальное n =");
                ui.add(egui::TextEdit::singleline(&mut self.n0_input).desired_width(80.0));
                ui.label("Точность eps =");
                ui.add(egui::TextEdit::singleline(&mut self.eps_input).desired_width(80.0));
            });

            match (self.n0_input.trim().parse::<usize>(), parse_number(&self.eps_input)) {
                (Ok(n0), Ok(eps)) if n0 > 0 && eps > 0.0 => {
                    var_ok = true;
                }
                _ => {
                    ui.colored_label(egui::Color32::RED, "Начальное n - целое больше нуля, eps - число больше нуля");
                }
            }
        }

        let can_run = match self.algorithm {
            Algorithm::ConstantStep => !self.partitions.is_empty() && !has_odd && limits_ok,
            Algorithm::VariableStep => var_ok && limits_ok,
        };

        if ui.add_enabled(can_run, egui::Button::new("Вычислить")).clicked() {
            self.results.clear();
            let a = parse_number(&self.a_input).unwrap();
            let b = parse_number(&self.b_input).unwrap();

            match self.algorithm {
                Algorithm::ConstantStep => {
                    for &n in &self.partitions {
                        let value = match self.method {
                            NumericalMethod::LeftRectangle => integration::left_rectangle(integration::task_function, a, b, n),
                            NumericalMethod::RightRectangle => integration::right_rectangle(integration::task_function, a, b, n),
                            NumericalMethod::Trapezoid => integration::trapezoid(integration::task_function, a, b, n),
                            NumericalMethod::Simpson => integration::simpson(integration::task_function, a, b, n),
                        };
                        self.results.push((n, value));
                    }
                }
                Algorithm::VariableStep => {
                    let n0 = self.n0_input.trim().parse::<usize>().unwrap();
                    let eps = parse_number(&self.eps_input).unwrap();

                    let r1 = integration::left_rect_var_step_alg1(integration::task_function, a, b, n0, eps);
                    let r2 = integration::left_rect_var_step_alg2(integration::task_function, a, b, n0, eps);
                    self.var_results = Some((r1, r2));
                }
            }
        }

        if self.algorithm == Algorithm::ConstantStep{
            if !self.results.is_empty() {
                ui.add_space(10.0);
                ui.strong(format!("Метод: {}", self.method.label()));

                egui::Grid::new("const_results").striped(true).show(ui, |ui| {
                    ui.strong("n");
                    ui.strong("Значение интеграла");
                    ui.end_row();

                    for &(n, value) in &self.results {
                        ui.label(n.to_string());
                        ui.label(format!("{value:.8}"));
                        ui.end_row();
                    }
                });
            }
        } else {
            if let Some((r1, r2)) = &self.var_results {
                egui::Grid::new("var_results").striped(true).show(ui, |ui| {
                    ui.strong("Параметр");
                    ui.strong("Алгоритм 1");
                    ui.strong("Алгоритм 2");
                    ui.end_row();

                    ui.label("Значение интеграла");
                    ui.label(format!("{:.8}", r1.integral));
                    ui.label(format!("{:.8}", r2.integral));
                    ui.end_row();

                    ui.label("Итоговое n");
                    ui.label(r1.n.to_string());
                    ui.label(r2.n.to_string());
                    ui.end_row();

                    ui.label("Вычислений функции");
                    ui.label(r1.evaluations.to_string());
                    ui.label(r2.evaluations.to_string());
                    ui.end_row();

                    ui.label("Итераций");
                    ui.label(r1.iterations.to_string());
                    ui.label(r2.iterations.to_string());
                    ui.end_row();

                    ui.label("Точность достигнута");
                    ui.label(if r1.converged {"да"} else {"нет"});
                    ui.label(if r2.converged {"да"} else {"нет"});
                    ui.end_row();
                });
            }
        }
    }

    fn show_ode(&mut self, ui: &mut egui::Ui) {
        ui.heading("Дифференциальные уравнения");
        ui.add_space(16.0);
        ui.label(format!("{}. Метод: {}", self.ode_problem.label(), self.ode_method.label()));

        ui.add_space(10.0);

        // ---- одиночное n (в режиме подбора это n для Рунге-Кутта) ----
        ui.horizontal(|ui| {
            let text = if self.ode_method == OdeMethod::Tune {
                "n для Рунге-Кутта (эталон) ="
            } else {
                "Число разбиений n ="
            };
            ui.label(text);
            ui.add(egui::TextEdit::singleline(&mut self.ode_n_input).desired_width(80.0));
        });

        let n = self.ode_n_input.trim().parse::<usize>().ok().filter(|&v| v > 0);
        if n.is_none() {
            ui.colored_label(egui::Color32::RED, "n - целое число больше нуля");
        }

        // ---- список n для Эйлера (только в режиме подбора) ----
        if self.ode_method == OdeMethod::Tune {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Список n для Эйлера:");
                ui.add(egui::TextEdit::singleline(&mut self.ode_n_list_input).desired_width(80.0));

                if ui.button("Добавить").clicked() {
                    if let Ok(v) = self.ode_n_list_input.trim().parse::<usize>() {
                        if v > 0 && v <= 3_000_000 && !self.ode_n_list.contains(&v) {
                            self.ode_n_list.push(v);
                            self.ode_n_list.sort();
                        }
                        self.ode_n_list_input.clear();
                    }
                }
            });

            if self.ode_n_list.is_empty() {
                ui.label("Список разбиений пуст");
            } else {
                ui.horizontal_wrapped(|ui| {
                    let mut to_remove = None;

                    for (i, v) in self.ode_n_list.iter().enumerate() {
                        if ui.button(format!("{v} \u{1F5D9}")).clicked() {
                            to_remove = Some(i);
                        }
                    }

                    if let Some(i) = to_remove {
                        self.ode_n_list.remove(i);
                    }

                    if ui.button("Очистить").clicked() {
                        self.ode_n_list.clear();
                    }
                });
            }
        }

        // ---- отрезок и начальные значения ----
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Отрезок: a =");
            ui.add(egui::TextEdit::singleline(&mut self.ode_a_input).desired_width(60.0));
            ui.label("b =");
            ui.add(egui::TextEdit::singleline(&mut self.ode_b_input).desired_width(60.0));

            let names = self.ode_problem.names();
            for (i, name) in names.iter().enumerate() {
                ui.label(format!("{name}(a) ="));
                ui.add(egui::TextEdit::singleline(&mut self.ode_y0_inputs[i]).desired_width(60.0));
            }
        });

        let mut y0s: Vec<f64> = Vec::new();
        let mut y0_ok = true;
        for s in &self.ode_y0_inputs {
            match parse_number(s) {
                Ok(v) => y0s.push(v),
                Err(_) => y0_ok = false,
            }
        }

        let base = match (parse_number(&self.ode_a_input), parse_number(&self.ode_b_input)) {
            (Ok(a), Ok(b)) if a < b && y0_ok => Some((a, b)),
            _ => {
                ui.colored_label(egui::Color32::RED, "Проверьте поля: числа и a < b");
                None
            }
        };

        let n_ok = match self.ode_method {
            OdeMethod::Tune => n.is_some() && !self.ode_n_list.is_empty(),
            _ => n.is_some(),
        };

        // ---- вычисление ----
        if ui.add_enabled(base.is_some() && n_ok, egui::Button::new("Вычислить")).clicked() {
            let (a, b) = base.unwrap();
            let n = n.unwrap();
            let h = (b - a) / n as f64;
            let f = self.ode_problem.rhs();
            let is_ex1 = self.ode_problem == OdeProblem::Example1;

            match self.ode_method {
                OdeMethod::Euler => {
                    self.ode_results = ode::euler(f, a, &y0s, h, n);
                }
                OdeMethod::RungeKutta => {
                    self.ode_results = ode::runge_kutta(f, a, &y0s, h, n);
                }
                OdeMethod::Compare => {
                    self.ode_results = ode::euler(f, a, &y0s, h, n);
                    self.ode_results_rk = ode::runge_kutta(f, a, &y0s, h, n);

                    self.ode_exact.clear();
                    if is_ex1 {
                        for (x, _) in &self.ode_results {
                            self.ode_exact.push(ode::exact1(*x, a, y0s[0]));
                        }
                    }
                }
                OdeMethod::Tune => {
                    // эталон: Рунге-Кутта на n шагах
                    let rk = ode::runge_kutta(f, a, &y0s, h, n);
                    let rk_end = rk.last().unwrap().1.clone();
                    let exact = if is_ex1 { Some(ode::exact1(b, a, y0s[0])) } else { None };

                    self.ode_tune.clear();
                    for &m in &self.ode_n_list {
                        let hm = (b - a) / m as f64;
                        let e = ode::euler(f, a, &y0s, hm, m);
                        self.ode_tune.push((m, e.last().unwrap().1.clone()));
                    }
                    self.ode_tune_ref = Some((n, rk_end, exact));
                }
            }
        }

        // ---- таблицы ----
        let names = self.ode_problem.names();
        let arg = self.ode_problem.arg();

        let table_h = (ui.available_height() * 0.5).max(150.0);

        match self.ode_method {
            OdeMethod::Euler | OdeMethod::RungeKutta => {
                if !self.ode_results.is_empty() {
                    egui::ScrollArea::both().max_height(table_h).show(ui, |ui| {
                        egui::Grid::new("ode_results").striped(true).show(ui, |ui| {
                            ui.strong(arg);
                            for name in names {
                                ui.strong(format!("{name} ({})", self.ode_method.label()));
                            }
                            ui.end_row();

                            for (x, y) in &self.ode_results {
                                ui.label(format!("{x:.4}"));
                                for v in y {
                                    ui.label(format!("{v:.6}"));
                                }
                                ui.end_row();
                            }
                        });
                    });
                }
            }
            OdeMethod::Compare => {
                if !self.ode_results.is_empty() {
                    let has_exact = !self.ode_exact.is_empty();

                    egui::ScrollArea::both().max_height(table_h).show(ui, |ui| {
                        egui::Grid::new("ode_compare").striped(true).show(ui, |ui| {
                            ui.strong(arg);
                            for name in names {
                                ui.strong(format!("{name} Эйлер"));
                                ui.strong(format!("{name} РК"));
                                ui.strong(format!("|Δ {name}|"));
                            }
                            if has_exact {
                                ui.strong("Точное");
                                ui.strong("|Эйлер − точное|");
                                ui.strong("|РК − точное|");
                            }
                            ui.end_row();

                            for i in 0..self.ode_results.len() {
                                ui.label(format!("{:.4}", self.ode_results[i].0));

                                for j in 0..names.len() {
                                    let e = self.ode_results[i].1[j];
                                    let r = self.ode_results_rk[i].1[j];
                                    ui.label(format!("{e:.6}"));
                                    ui.label(format!("{r:.6}"));
                                    ui.label(format!("{:.2e}", (e - r).abs()));
                                }

                                if has_exact {
                                    let t = self.ode_exact[i];
                                    let e = self.ode_results[i].1[0];
                                    let r = self.ode_results_rk[i].1[0];
                                    ui.label(format!("{t:.6}"));
                                    ui.label(format!("{:.2e}", (e - t).abs()));
                                    ui.label(format!("{:.2e}", (r - t).abs()));
                                }
                                ui.end_row();
                            }
                        });
                    });
                }
            }
            OdeMethod::Tune => {
                if !self.ode_tune.is_empty() {
                    if let Some((n_ref, rk_end, exact)) = &self.ode_tune_ref {
                        let mut line = format!("Рунге-Кутта (n = {n_ref}):");
                        for (j, name) in names.iter().enumerate() {
                            line += &format!("  {name}(b) = {:.8}", rk_end[j]);
                        }
                        ui.label(line);

                        if let Some(t) = exact {
                            ui.label(format!("Точное решение: y(b) = {t:.8}"));
                        }
                        ui.add_space(6.0);
                    }

                    egui::ScrollArea::both().max_height(table_h).show(ui, |ui| {
                        egui::Grid::new("ode_tune").striped(true).show(ui, |ui| {
                            ui.strong("n");
                            for name in names {
                                ui.strong(format!("{name}(b), Эйлер"));
                                ui.strong(format!("|Δ {name}| от РК"));
                            }
                            if matches!(&self.ode_tune_ref, Some((_, _, Some(_)))) {
                                ui.strong("|Эйлер − точное|");
                            }
                            ui.end_row();

                            for (m, e) in &self.ode_tune {
                                ui.label(m.to_string());

                                for j in 0..names.len() {
                                    let rk_j = self.ode_tune_ref.as_ref().unwrap().1[j];
                                    ui.label(format!("{:.8}", e[j]));
                                    ui.label(format!("{:.2e}", (e[j] - rk_j).abs()));
                                }

                                if let Some((_, _, Some(t))) = &self.ode_tune_ref {
                                    ui.label(format!("{:.2e}", (e[0] - t).abs()));
                                }
                                ui.end_row();
                            }
                        });
                    });
                }
            }
        }
        self.show_ode_plot(ui);
    }

    fn show_gauss(&mut self, ui: &mut egui::Ui) {
        ui.heading("Системы линейных уравнений");
        ui.add_space(16.0);
        ui.label(format!("Выбранный метод: {}", self.gauss_method.label()));

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Размер матрицы n =");
            let resp = ui.add(egui::DragValue::new(&mut self.gauss_n).range(1..=10));
            if resp.changed() {
                self.gauss_a.resize(self.gauss_n, Vec::new());

                for row in self.gauss_a.iter_mut() {
                    row.resize(self.gauss_n, String::new());
                }

                self.gauss_b.resize(self.gauss_n, String::new());

                self.clear_gauss_results();
            }
            ui.label("Дельта =");
            ui.add(egui::TextEdit::singleline(&mut self.gauss_delta).desired_width(60.0));
        });
        ui.add_space(10.0);

        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("matrix").striped(true).show(ui, |ui| {
                for i in 0..self.gauss_n {
                    for j in 0..self.gauss_n {
                        ui.add(egui::TextEdit::singleline(&mut self.gauss_a[i][j]).desired_width(60.0));
                    }
                    ui.label("|");
                    ui.add(egui::TextEdit::singleline(&mut self.gauss_b[i]).desired_width(60.0));
                    ui.end_row();
                }
            });
        });

        ui.add_space(10.0);
        if ui.button("Решить").clicked() {
            self.clear_gauss_results();
            match self.build_matrix() {
                Err(e) => {
                    self.gauss_error = Some(e);
                }
                Ok(mut m) => {
                    let res = match self.gauss_method {
                        GaussMethod::Classic => forward_pass_classic(&mut m),
                        GaussMethod::Optimal => forward_pass(&mut m),
                        GaussMethod::GaussJordan => gauss_jordan(&mut m),
                    };
                    match res {
                        Err(e) => {
                            self.gauss_error = Some(e);
                        }
                        Ok(()) => {
                            self.gauss_triangle = Some(m.clone());
                            let x: Vec<f64> = if self.gauss_method == GaussMethod::GaussJordan {
                                let mut answ = Vec::new();
                                for row in &m {
                                    answ.push(row[m.len()]);
                                }
                                answ
                            } else {
                                backward_pass(&m)
                            };
                            self.gauss_x = Some(x);
                        }
                    }
                }
            }
        }

        if let Some(err) = &self.gauss_error {
            ui.colored_label(egui::Color32::RED, err);
        }

        if let Some(tri) = &self.gauss_triangle {
            let title = if self.gauss_method == GaussMethod::GaussJordan {
                "Приведение к единичной матрице"
            } else {
                "Прямой ход"
            };
            ui.strong(title);
            egui::Grid::new("triangle").striped(true).show(ui, |ui| {
                for row in tri {
                    for v in row {
                        ui.label(format!("{:.4}", v));
                    }
                    ui.end_row();
                }
            });
        }

        if let Some(x) = &self.gauss_x {
            let title = if self.gauss_method == GaussMethod::GaussJordan {
                "Решение"
            } else {
                "Обратный ход, решение"
            };
            ui.strong(title);
            egui::Grid::new("x").striped(true).show(ui, |ui| {
                ui.label("Элемент");
                ui.label("Значение");
                ui.end_row();

                for (i, v) in x.iter().enumerate() {
                    ui.label((i + 1).to_string());
                    ui.label(format!("{:.4}", v));
                    ui.end_row();
                }
            });
        }
    }

    fn build_matrix(&self) -> Result<Vec<Vec<f64>>, String> {
        let mut m = Vec::new();
        let delta = match parse_number(&self.gauss_delta) {
            Ok(v) => v,
            Err(_) => return Err("Введите число в поле δ".to_string()),
        };
        for i in 0..self.gauss_a.len() {
            let mut row = Vec::new();
            for j in 0..self.gauss_a.len() {
                match parse_number(&self.gauss_a[i][j]) {
                    Ok(v) => row.push(v),
                    Err(_e) => return Err(format!("Введите число: строка {}, столбец {}", i + 1, j + 1)),
                }
            }
            match parse_number(&self.gauss_b[i]) {
                Ok(v) => row.push(v + delta),
                Err(_) => return Err(format!("Введите свободный член в строке {}", i + 1).to_string()),
            }
            m.push(row);
        }
        Ok(m)
    }

    fn clear_gauss_results(&mut self) {
        self.gauss_error = None;
        self.gauss_triangle = None;
        self.gauss_x = None;
    }

    fn show_ode_plot(&self, ui: &mut egui::Ui) {
        use egui_plot::{Legend, Line, Plot, PlotPoints};

        if self.ode_results.is_empty() || self.ode_method == OdeMethod::Tune {
            return;
        }
        if self.ode_method == OdeMethod::Compare && self.ode_results_rk.len() != self.ode_results.len() {
            return;
        }

        let names = self.ode_problem.names();
        let height = ui.available_height().max(200.0);
        ui.add_space(8.0);

        ui.columns(names.len(), |cols| {
            for (j, name) in names.iter().enumerate() {
                Plot::new(format!("ode_plot_{j}"))
                    .legend(Legend::default())
                    .height(height)
                    .show(&mut cols[j], |plot_ui| {
                        match self.ode_method {
                            OdeMethod::Euler | OdeMethod::RungeKutta => {
                                let pts = PlotPoints::from(series(&self.ode_results, j));
                                plot_ui.line(Line::new(pts).name(format!("{name} ({})", self.ode_method.label())));
                            }
                            OdeMethod::Compare => {
                                let e = PlotPoints::from(series(&self.ode_results, j));
                                let r = PlotPoints::from(series(&self.ode_results_rk, j));
                                plot_ui.line(Line::new(e).name(format!("{name} Эйлер")));
                                plot_ui.line(Line::new(r).name(format!("{name} РК")));

                                if j == 0 && !self.ode_exact.is_empty() {
                                    let mut pts = Vec::new();
                                    for i in 0..self.ode_exact.len() {
                                        pts.push([self.ode_results[i].0, self.ode_exact[i]]);
                                    }
                                    plot_ui.line(Line::new(PlotPoints::from(pts)).name("Точное"));
                                }
                            }
                            OdeMethod::Tune => {}
                        }
                    });
            }
        });
    }

    fn reset_ode_inputs(&mut self) {
        let (a, b, y0) = self.ode_problem.defaults();
        self.ode_a_input = a.to_string();
        self.ode_b_input = b.to_string();

        self.ode_y0_inputs.clear();
        for s in y0 {
            self.ode_y0_inputs.push(s.to_string());
        }

        self.ode_n_input = match self.ode_problem {
            OdeProblem::Example2 => "5".to_string(),
            _ => "10".to_string(),
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Вычислительная математика",
        options,
        Box::new(|_cc| Ok(Box::new(MathApp::default()))),
    )
}

fn parse_number(text: &str) -> Result<f64, std::num::ParseFloatError> {
    text.trim().replace(",", ".").parse::<f64>()
}

fn series(data: &[(f64, Vec<f64>)], j: usize) -> Vec<[f64; 2]> {
    let mut pts = Vec::new();
    for (x, y) in data {
        pts.push([*x, y[j]]);
    }
    pts
}