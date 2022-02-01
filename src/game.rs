use std::{borrow::Cow, thread, sync::{mpsc, Mutex, Arc}};
use wikipedia::{http::default::Client, Wikipedia};

use super::history::History;

use eframe::egui::{
    menu, Align, Button, Color32, CtxRef, FontDefinitions, FontFamily, Label, Layout, Separator,
    Slider, TextStyle, TopBottomPanel, Ui,
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

pub struct Game {
    urls: History,
    total_time: f64,
    destination: String,
    game_started: bool,
    pub dark_mode: bool,
    start_time: Option<f64>,
    pause: bool,
    content: Option<String>,
    links: Option<Vec<String>>,
    wiki: Wikipedia<Client>
}

impl Game {
    pub fn new() -> Self {
        Self {
            urls: History::empty(),
            total_time: 120.,
            destination: String::from("?"),
            game_started: false,
            dark_mode: true,
            start_time: None,
            content: None,
            pause: false,
            links: None,
            wiki: Wikipedia::default()
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_main(&mut self, ui: &mut Ui) {
        if self.game_started {
            if self.start_time.is_none() {
                println!("{}", ui.input().time);
                self.start_time = Some(ui.input().time);
            }
            self.render_game(ui);
        } else {
            self.render_start(ui);
        }
    }

    pub fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.colored_label(
                        if self.dark_mode { WHITE } else { BLACK },
                        format!(
                            "Destination : {} -- Current : {}",
                            self.destination,
                            self.urls.url()
                        ),
                    );
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let closebtn = ui.add(Button::new("❌").text_style(TextStyle::Body));
                    if closebtn.clicked() {
                        frame.quit();
                    }
                    let themebtn = ui.add(
                        Button::new(if self.dark_mode { "🔆" } else { "🌙" })
                            .text_style(TextStyle::Body),
                    );
                    if ui
                        .add(Button::new("🔁").text_style(TextStyle::Body))
                        .clicked()
                    {
                        *self = Self::new();
                    }

                    if themebtn.clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                    if ui
                        .add(Button::new("🏠").text_style(TextStyle::Body))
                        .clicked()
                    {
                        self.urls.add_home();
                    }
                    if ui
                        .add(
                            Button::new("⏩")
                                .text_style(TextStyle::Body)
                                .enabled(!self.urls.is_last()),
                        )
                        .clicked()
                    {
                        self.urls.next();
                    }
                    if ui
                        .add(
                            Button::new("⏪")
                                .text_style(TextStyle::Body)
                                .enabled(!self.urls.is_first()),
                        )
                        .clicked()
                    {
                        self.urls.prev();
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    pub fn render_header(&self, ui: &mut Ui) {
        println!("{}", ui.input().time);
        ui.vertical_centered(|ui| {
            ui.add(
                Label::new(
                    if self.game_started && self.start_time.is_some() && !self.pause {
                        format!(
                            "Time : {}",
                            (self.total_time - ui.input().time + self.start_time.unwrap()) as u32
                        )
                    } else {
                        "WikiGame".to_owned()
                    },
                )
                .text_style(TextStyle::Heading)
                .text_color(if self.dark_mode { WHITE } else { BLACK }),
            );
        });
        ui.add_space(PADDING);
        ui.add(Separator::default().spacing(20.));
    }

    fn render_start(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.total_time, 60.0..=600.));
        if ui
            .add(
                Button::new("Start the Game!!")
                    .text_style(TextStyle::Heading)
                    .text_color(if self.dark_mode { WHITE } else { BLACK }),
            )
            .clicked()
        {
            self.start_game()
        }
    }

    fn render_game(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);

        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.colored_label(
                if self.dark_mode { WHITE } else { BLACK },
                self.content.as_ref().unwrap(),
            );

            for link in self.links.as_ref().unwrap().clone() {
                if ui
                    .add(
                        Button::new(&link)
                            .text_style(TextStyle::Body)
                            .text_color(if self.dark_mode { CYAN } else { RED }),
                    )
                    .clicked()
                {
                    self.goto_link(&link);
                }
            }
        });

        ui.add_space(PADDING);
        ui.add(Separator::default());
    }

    fn start_game(&mut self) {
        let new = Self::get_random_link();
        self.destination = Self::get_destination_link(new, 4);
        self.goto_link(new);
        self.game_started = true;
    }

    pub fn render_loss(&mut self, ui: &mut Ui) {
        ui.colored_label(if self.dark_mode { WHITE } else { BLACK }, "You Lost!");
        let current = self.urls.url();
        self.urls.clean();
        self.urls.new_next(&current);
        self.pause = true;
    }

    pub fn render_win(&mut self, ui: &mut Ui) {
        ui.colored_label(if self.dark_mode { WHITE } else { BLACK }, "You Won!");
        self.urls.clean();
        self.urls.new_next(&self.destination);
        self.pause = true;
    }

    pub fn goto_link(&mut self, link: &str) {
        self.urls.new_next(link);
        let link = link.to_owned();
        let stuff = Arc::new(Mutex::new(self));
        let handle = thread::spawn(move || {
            let mut s = stuff.lock().unwrap();
            let page = s.wiki.page_from_title(link);
            s.content = Some(page.get_content().unwrap());
            s.links = Some(page.get_links().unwrap().map(|l| l.title).collect());
            println!("Fetched page");
        });
    }

    pub fn get_random_link() -> &'static str {
        "Magnus Carlsen"
    }

    pub fn get_destination_link(_start_link: &str, _depth: u8) -> String {
        String::new()
    }

    pub fn render(&mut self, ui: &mut Ui) {
        if self.start_time.is_some()
            && self.total_time - ui.input().time + self.start_time.unwrap() <= 0.0
            && self.game_started
        {
            self.render_loss(ui);
        } else if self.destination != "?"
            && self.urls.url() == self.destination
            && self.game_started
        {
            self.render_win(ui);
        } else {
            self.render_main(ui);
        }
    }
}
