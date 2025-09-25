use eframe::egui;
use egui_plot::{Line, Plot};

use std::os::raw::c_double;

unsafe extern "C" {
    fn fft(
        input_real: *const c_double,
        input_imag: *const c_double,
        out_real: *mut c_double,
        out_imag: *mut c_double,
        n: i32,
    );
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Interactive FFT Demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    signal_type: usize,
    n: usize,
    input_signal: Vec<f64>,
    fft_real: Vec<f64>,
    fft_imag: Vec<f64>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            signal_type: 0,
            n: 32,
            input_signal: Vec::new(),
            fft_real: Vec::new(),
            fft_imag: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Signal:");
                ui.radio_value(&mut self.signal_type, 0, "Linear");
                ui.radio_value(&mut self.signal_type, 1, "Sinus");
                ui.radio_value(&mut self.signal_type, 2, "Sinus + Sinus");
                ui.add(egui::Slider::new(&mut self.n, 4..=128).text("Length"));
            });

            if ui.button("Compute FFT").clicked() {
                self.input_signal = match self.signal_type {
                    0 => (0..self.n).map(|x| x as f64).collect(),
                    1 => (0..self.n)
                        .map(|x| {
                            ((x as f64 / self.n as f64) * std::f64::consts::PI * 2.0).sin() * 10.0
                        })
                        .collect(),
                    2 => (0..self.n)
                        .map(|x| {
                            let t = x as f64 / self.n as f64;
                            10.0 * (2.0 * std::f64::consts::PI * t).sin()
                                + 5.0 * (4.0 * std::f64::consts::PI * t).sin()
                        })
                        .collect(),
                    _ => vec![0.0; self.n],
                };

                let input_imag = vec![0.0; self.n];
                self.fft_real = vec![0.0; self.n];
                self.fft_imag = vec![0.0; self.n];

                unsafe {
                    fft(
                        self.input_signal.as_ptr(),
                        input_imag.as_ptr(),
                        self.fft_real.as_mut_ptr(),
                        self.fft_imag.as_mut_ptr(),
                        self.n.try_into().unwrap(),
                    );
                }
            }

            let input_line = Line::new(
                "Input",
                (0..self.input_signal.len())
                    .map(|i| [i as f64, self.input_signal[i]])
                    .collect::<Vec<_>>(), // Vec<[f64; 2]> now
            );

            Plot::new("Input Signal").show(ui, |plot_ui| {
                plot_ui.line(input_line);
            });

            if !self.fft_real.is_empty() {
                let fft_line = Line::new(
                    "FFT",
                    (0..self.fft_real.len())
                        .map(|i| {
                            let magnitude =
                                (self.fft_real[i].powi(2) + self.fft_imag[i].powi(2)).sqrt();
                            [i as f64, magnitude] // use [f64; 2] array instead of tuple
                        })
                        .collect::<Vec<_>>(),
                );
                Plot::new("FFT Amplitude").show(ui, |plot_ui| {
                    plot_ui.line(fft_line);
                });
            }
        });
    }
}
