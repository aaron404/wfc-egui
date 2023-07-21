use egui::{ColorImage, Style};
use image::EncodableLayout;

const TILEMAP_PATH: &str = "res/tilemap.png";
const TILE_SIZE: u32 = 16;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WfcApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    tile_selection: Option<usize>,
    num_tiles: usize,
    #[serde(skip)]
    tile_textures: Vec<Option<egui::TextureHandle>>,
    #[serde(skip)]
    tile_images: Vec<ColorImage>,
}

impl Default for WfcApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            tile_selection: None,
            num_tiles: 0,
            tile_textures: vec![],
            tile_images: vec![],
        }
    }
}

impl WfcApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let s = include_bytes!("../res/tilemap.png");
        let img = image::load_from_memory(s).expect("Failed to load tilemap");
        // let img = image::open(TILEMAP_PATH).expect("Failed to load tilemap");
        println!("img size: {} {}", img.width(), img.height());
        for x in 0..img.width() / TILE_SIZE {
            let tile = img.crop_imm(x * TILE_SIZE, 0, TILE_SIZE, TILE_SIZE);
            let tex = egui::ColorImage::from_rgb([16, 16], tile.into_rgb8().as_bytes());
            println!("  num images:   {}", app.tile_images.len());
            app.tile_images.push(tex);
            app.tile_textures.push(None);
        }
        println!("  num images:   {}", app.tile_images.len());
        println!("  num textures: {}", app.tile_textures.len());

        app
    }
}

impl eframe::App for WfcApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            tile_selection: _,
            num_tiles: _,
            tile_textures: _,
            tile_images: _,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.label("Palette");

            ui.horizontal(|ui| {
                // let i  = egui::ImageData::Color(egui::ColorImage::)
                for i in 0..self.tile_images.len() {
                    let t: &egui::TextureHandle = self.tile_textures[i].get_or_insert_with(|| {
                        ui.ctx().load_texture(
                            format!("tile_{i}"),
                            self.tile_images[i].clone(),
                            // egui::ColorImage::example(),
                            Default::default(),
                        )
                    });
                    if ui
                        .add(
                            egui::ImageButton::new(t, t.size_vec2() * 2.0)
                                // .frame(false)
                                .selected(if let Some(s) = self.tile_selection {
                                    i == s
                                } else {
                                    false
                                }),
                        )
                        .clicked()
                    {
                        if let Some(s) = self.tile_selection {
                            if s == i {
                                self.tile_selection = None;
                            } else {
                                self.tile_selection = Some(i);
                            }
                        } else {
                            self.tile_selection = Some(i);
                        }
                    }
                }
            });

            if let Some(s) = self.tile_selection {
                ui.label(format!("tile selected: {s}"));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
