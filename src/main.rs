use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();

    // This is much less succinct than the original code:
    //   `Box::new(|cc| Box::new(Simple::new(cc))),`
    // But I wanted to get a better idea of what was really going on and
    // I couldn't come up with the right syntax. So I had ChatGPT-4
    // help me out, see the end of this conversation:
    //   https://chat.openai.com/share/945e3201-1198-4122-9650-261c9fd5c55b
    let simple_creator = move |cc: &eframe::CreationContext<'_>| {
        let simple_instance = Simple::new(cc);
        Box::new(simple_instance)
    };
    // Subsequently, I figured out that using type `eframe::AppCreator` removed
    // the need for the `#[allow(clippy::type_complexity)]` attribute.
    let app_creator: eframe::AppCreator = Box::new(move |cc| simple_creator(cc));

    eframe::run_native("Simple Example", native_options, app_creator).unwrap();
}

struct Simple;

impl Simple {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self
    }
}

impl eframe::App for Simple {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let root = EguiBackend::new(ui).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .margin_right(20) // Extend the right margin so the last x-axis label value, 1.0, is not clipped
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    RED,
                ))
                .unwrap()
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .border_style(BLACK)
                .draw()
                .unwrap();

            root.present().unwrap();
        });
    }
}
