//! TODO: INCOMPLETE

mod game;
mod history;

use eframe::{
    egui::{CentralPanel, CtxRef, ScrollArea, Vec2, Visuals},
    epi::{App, Frame, Storage},
    run_native, NativeOptions,
};

use game::Game;

impl App for Game {
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        if self.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render(ui);
            });
        });
        ctx.request_repaint()
    }

    fn name(&self) -> &str {
        "WikiGame"
    }
}

fn main() {
    let app = Game::new();
    let options = NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some(Vec2::new(1240., 720.)),
        resizable: false,
        transparent: false,
    };

    run_native(Box::new(app), options);
}
