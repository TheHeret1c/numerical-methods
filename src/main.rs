mod integration;

use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TaskType {
    #[default]
    Integration,
}

impl TaskType {
    fn label(self) -> &'static str {
        match self {
            TaskType::Integration => "Численное интегрирование",
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

impl Default for IntegrationApp {
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
        }
    }
}
struct IntegrationApp {
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
}


impl eframe::App for IntegrationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.algorithm == Algorithm::VariableStep {
            self.method = NumericalMethod::LeftRectangle;
        }

        let old_method = self.method;
        let old_algorithm = self.algorithm;

        egui::SidePanel::left("menu").show(ctx, |ui| {
            ui.heading("Меню");

            ui.strong("1. Тип задачи");
            ui.radio_value(&mut self.task, TaskType::Integration, TaskType::Integration.label());

            ui.add_space(10.0);
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
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
        });

        if self.method != old_method || self.algorithm != old_algorithm {
            self.results.clear();
            self.var_results = None;
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Вычислительная математика",
        options,
        Box::new(|_cc| Ok(Box::new(IntegrationApp::default()))),
    )
}

fn parse_number(text: &str) -> Result<f64, std::num::ParseFloatError> {
    text.trim().replace(",", ".").parse::<f64>()
}