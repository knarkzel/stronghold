use std::f32::consts::PI;

use eframe::App;
use egui::{
    plot::{Line, Plot, Value, Values},
    DragValue, Vec2,
};

#[derive(Default)]
struct Input {
    x: f32,
    z: f32,
    angle: f32,
}

#[derive(Default)]
pub struct Stronghold {
    start: Input,
    end: Input,
}

impl Stronghold {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl App for Stronghold {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { start, end } = self;

            ui.heading("Stronghold");

            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(DragValue::new(&mut start.x));
                ui.label("Z:");
                ui.add(DragValue::new(&mut start.z));
                ui.label("Angle:");
                ui.add(DragValue::new(&mut start.angle));
            });

            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(DragValue::new(&mut end.x));
                ui.label("Z:");
                ui.add(DragValue::new(&mut end.z));
                ui.label("Angle:");
                ui.add(DragValue::new(&mut end.angle));
            });

            // Calculate intersection
            let x1 = start.x;
            let z1 = start.z;
            let mut d1 = start.angle;

            let x2 = end.x;
            let z2 = end.z;
            let mut d2 = end.angle;

            if d1 >= 90.0 {
                d1 = (180.0 - d1) + 90.0;
            } else {
                d1 = -1.0 * (d1 + 90.0);
            }

            if d2 >= 90.0 {
                d2 = (180.0 - d2) + 90.0;
            } else {
                d2 = -1.0 * (d2 + 90.0);
            }

            let m1 = -1.0 * (d1 * (PI / 180.0)).tan();
            let m2 = -1.0 * (d2 * (PI / 180.0)).tan();

            let i1 = z1 - m1 * x1;
            let i2 = z2 - m2 * x2;

            let x = (i2 - i1) / (m1 - m2);
            let z = m1 * x + i1;

            ui.horizontal(|ui| {
                ui.label(&format!("Result: {x}, {z}"));
            });

            // Show lines
            let length = 3000.0;

            // Start
            let start_line = Line::new(Values::from_values(vec![
                Value::new(start.x, start.z),
                Value::new(
                    start.x + (start.angle / 180.0 * PI).cos() * length,
                    start.z + (start.angle / 180.0 * PI).sin() * length,
                ),
            ]))
            .width(3.0);

            // End
            let end_line = Line::new(Values::from_values(vec![
                Value::new(end.x, end.z),
                Value::new(
                    end.x + (end.angle / 180.0 * PI).cos() * length,
                    end.z + (end.angle / 180.0 * PI).sin() * length,
                ),
            ]))
            .width(3.0);

            Plot::new("Stronghold")
                .center_x_axis(true)
                .center_y_axis(true)
                .data_aspect(1.0)
                .include_x(6000.0)
                .include_y(6000.0)
                .show(ui, |plot| {
                    plot.line(start_line);
                    plot.line(end_line);
                });
        });
    }
}
