use eframe::{
    egui::{
        self, CtxRef, Hyperlink, Label, Separator, TextStyle, TopBottomPanel, Ui, Vec2, Visuals,
    },
    epi,
};
use headlines::PADDING;

mod headlines;

impl epi::App for headlines::Headlines {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_fonts(ctx)
    }
    fn name(&self) -> &str {
        "Headlines"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }
        self.render_top_panel(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }
}
fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org").monospace());
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("Made with egui")
                    .text_style(TextStyle::Monospace),
            );
            ui.add(
                Hyperlink::new("https://github.com/creativcoder/headlines")
                    .text("creativcoder/headlines")
                    .text_style(TextStyle::Monospace),
            );
            ui.add_space(10.);
        })
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}
fn main() {
    let app = headlines::Headlines::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native(Box::new(app), native_options);
}
