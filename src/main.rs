use eframe::egui::{self, Context, CentralPanel, RawInput, FontDefinitions, FontFamily, FontData};
use chrono::prelude::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
	"Kiloseconds",
	options,
	Box::new(|cc| {
	    Box::<App>::default()
	})
    )
}

struct App {
    time: DateTime<Local>
}

impl Default for App {
    fn default() -> Self {
	Self {
	    time: Local::now()
	}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx, |ui| {
	    let mut fonts = FontDefinitions::default();
	    fonts.font_data.insert("JBMono".to_owned(),
				   FontData::from_static(include_bytes!("/usr/share/fonts/TTF/JetBrainsMonoNerdFontMono-Regular.ttf"))); // .ttf and .otf supported
	    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
		.insert(0, "JBMono".to_owned());
	    ctx.set_fonts(fonts);
	    let midnight = self.time.date().and_hms(0, 0, 0);
	    let duration_since_midnight = self.time - midnight;
	    let seconds_since_midnight = duration_since_midnight.num_seconds() as f64;
	    self.time = Local::now();
	    ui.centered_and_justified(|ui| {
		ui.label(egui::RichText::new((seconds_since_midnight / 1000.0).to_string()).size(50.0).color(egui::Color32::WHITE));
	    });
	    ui.ctx().request_repaint();
	});
    }
}
