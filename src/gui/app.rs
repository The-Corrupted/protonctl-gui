use crate::mock::get_mocked_install_versions;
use crate::plib::db::{AppDB, InstallType, InstalledVersion};
use eframe;
use egui;

#[derive(PartialEq, PartialOrd)]
enum FilterEnum {
    PROTON,
    WINE,
    UMU,
    CUSTOM,
    ALL,
}

pub struct ProtonCtlApp {
    label: String,
    value: f64,
    entries: Vec<InstalledVersion>,
    radio_filter: Option<InstallType>,
    db: AppDB,
}

impl ProtonCtlApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Use rusqlite to get currently installed compatibility tools
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut this: ProtonCtlApp = Default::default();
        this.db.create_db_or_do_nothing().unwrap();
        this.entries = get_mocked_install_versions(50);
        // this.entries = this.db.get_entries().unwrap();
        this
    }
}

impl Default for ProtonCtlApp {
    fn default() -> Self {
        Self {
            label: "ProtonCtl".to_owned(),
            value: 0.0,
            entries: Vec::new(),
            radio_filter: None,
            db: AppDB::new().unwrap(),
        }
    }
}

fn render_header(ui: &mut egui::Ui) {
    ui.heading("ProtonCtl");
    ui.separator();
}

fn render_combobox(ui: &mut egui::Ui, radio_filter: &mut Option<InstallType>) {
    egui::ComboBox::from_id_salt("Filter")
        .truncate()
        .selected_text(format!(
            "{}",
            if radio_filter.as_ref() == None {
                "None"
            } else {
                radio_filter.as_ref().unwrap().to_str()
            }
        ))
        .show_ui(ui, |ui| {
            ui.selectable_value(radio_filter, None, "All");
            ui.selectable_value(radio_filter, Some(InstallType::PROTON), "Proton");
            ui.selectable_value(radio_filter, Some(InstallType::WINE), "Wine");
            ui.selectable_value(radio_filter, Some(InstallType::UMU), "UMU");
            ui.selectable_value(radio_filter, Some(InstallType::CUSTOM), "Custom");
        });
}

impl eframe::App for ProtonCtlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let grid_width = ui.available_width() * 0.9;
            render_header(ui);
            render_combobox(ui, &mut self.radio_filter);

            egui::ScrollArea::vertical()
                .hscroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(grid_width, ui.available_width()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::Grid::new("install_grid")
                                .striped(true)
                                .spacing(egui::Vec2::new(1.0, 1.0))
                                .show(ui, |ui| {
                                    let mut delete_index: Option<usize> = None;
                                    for (index, value) in self.entries.iter_mut().enumerate() {
                                        if let Some(install_type) = self.radio_filter.as_ref() {
                                            if *install_type != value.install_type {
                                                continue;
                                            }
                                        }
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new("-")
                                                        .color(egui::Color32::WHITE)
                                                        .size(28.0)
                                                        .strong(),
                                                )
                                                .rounding(10.0)
                                                .min_size(egui::Vec2::new(26.0, 20.0))
                                                .fill(egui::Color32::RED),
                                            )
                                            .clicked()
                                        {
                                            delete_index = Some(index);
                                        };
                                        ui.add_sized(
                                            [grid_width * 0.4, 30.0],
                                            egui::TextEdit::singleline(&mut value.name)
                                                .frame(false)
                                                .font(egui::FontId::new(
                                                    20.0,
                                                    egui::FontFamily::Proportional,
                                                ))
                                                .horizontal_align(egui::Align::Center),
                                        );
                                        ui.add_sized(
                                            [grid_width * 0.4, 30.0],
                                            egui::TextEdit::singleline(&mut value.location)
                                                .frame(false)
                                                .font(egui::FontId::new(
                                                    20.0,
                                                    egui::FontFamily::Proportional,
                                                ))
                                                .horizontal_align(egui::Align::Center),
                                        );

                                        ui.end_row();
                                    }
                                    ui.add(
                                        egui::Button::new(
                                            egui::RichText::new("+")
                                                .color(egui::Color32::WHITE)
                                                .size(28.0)
                                                .strong(),
                                        )
                                        .rounding(10.0)
                                        .min_size(egui::Vec2::new(26.0, 20.0))
                                        .fill(egui::Color32::GREEN),
                                    );

                                    // Remove marked entries
                                    if let Some(index) = delete_index {
                                        self.entries.remove(index);
                                    }
                                });
                        },
                    );
                });
        });
    }
}
