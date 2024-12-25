#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use eframe::egui;

#[derive(Default)]
struct ExampleApp {
    // dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    settings: bool,
    about: bool,
}
impl ExampleApp {
    fn name() -> &'static str {
        "egui boilerplate"
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // fixed pixel
        // ctx.set_pixels_per_point(1.25);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            // menu bar
            egui::menu::bar(ui, |ui| {
                // first menu
                ui.menu_button("File", |ui| {
                    if ui.button("Open new...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    if ui.button("Open recent...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    ui.separator();
                    if ui.button("Save...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    if ui.button("Save as...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.picked_path = Some(path.display().to_string());
                            println!("{:?}", self.picked_path);
                        }
                    }
                    // confirm on quit
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                // Tools
                ui.menu_button("Tools", |ui| {
                    if ui.button("Plugins").clicked() {
                        unimplemented!()
                    }
                    if ui.button("Preferences").clicked() {
                        unimplemented!()
                    }
                    if ui.button("Settings").clicked() {
                      self.settings =  true;
                    }
                });
                // Help
                ui.menu_button("Help", |ui| {
                    if ui.button("Get Help").clicked() {
                        // function
                    }
                    if ui.button("Report a bug").clicked() {
                        //functionality
                    }
                    if ui.button("Troubleshooting").clicked() {
                        //functionality
                    }
                    ui.separator();
                    if ui.button("About this program").clicked() {
                        //funtionality
                    }
                })
            });
        });
        // CentralPanel == Container
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Side menu");
                        ui.separator();
                        if ui.button("Click me!").clicked() {
                            // …
                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |_ui| {
                    });

                });
            // This literally creates the button AND checks to see if it was clicked
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Main sector");
                    ui.label("Main section of the program.");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::CentralPanel::default().show_inside(ui, |_ui| {
                    });
                });
            });
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
        if self.about {
          egui::Window::new("About")
              .resizable(false)
              .collapsible(false)
              .show(ctx, |ui| {
                  ui.label("change theme");
                  egui::widgets::global_dark_light_mode_buttons(ui);
                  ui.separator();
                  ui.label("another setting");
                  egui::widgets::global_dark_light_mode_buttons(ui);
              });
        }
        if self.settings {
          egui::Window::new("Setting")
              .resizable(false)
              .show(ctx, |ui| {
                  ui.label("change theme");
                  egui::widgets::global_dark_light_mode_buttons(ui);
                  ui.separator();
                  ui.label("another setting");
                  egui::widgets::global_dark_light_mode_buttons(ui);
              });
        }
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .pivot(egui::Align2::CENTER_TOP)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }
                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }

    }
}

// main loop
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((1280.0, 720.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
