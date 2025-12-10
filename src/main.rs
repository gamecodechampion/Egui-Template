use eframe::Frame;
use egui::{Context, Ui};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        //include any useful options such as an inner size!
            .with_inner_size([200., 200.]),
        ..Default::default()
    };

    eframe::run_native(
        "App Title",
        options,
        Box::new(|cc| {
            /*
            fun fact: You can actually edit the visuals in this box! Here is a sample below

            let mut visuals = Visuals::dark();
            visuals.panel_fill = Color32::from_rgb(11,19,43);

            visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(58, 80, 107);
            visuals.widgets.inactive.fg_stroke = Stroke::new(20.0, egui::Color32::WHITE);

            visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(91, 192, 190);

            cc.egui_ctx.set_visuals(visuals);
             */
            Ok(Box::<App>::default())
        })
    )
}


struct App {
    // Variables here will allow you to have persistance!
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Give a default value for said variables
        }
    }
}


impl eframe::App for App {
    // Main app loop!
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            /*
            Add anything useful to this such as panels, widgets, etc.
            for this example I will add a vertical-centered panel
            with a simple heading!
             */

            ui.vertical_centered(|ui: &mut Ui| {
                ui.heading("Example heading!");
            })
        });
    }
}