#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, FontFamily, Key, TextStyle};
use pulldown_cmark::{Parser, Tag, Event};
use std::path::PathBuf;

struct MdViewer {
    markdown: String,
    file_path: Option<PathBuf>,
    cache: egui_commonmark::CommonMarkCache,
    dark_mode: bool,
    font_scale: f32,
    font_family: FontFamily,
    edit_mode: bool,
    base_style: Option<egui::Style>,
}

impl Default for MdViewer {
    fn default() -> Self {
        Self {
            markdown: String::new(),
            file_path: None,
            cache: egui_commonmark::CommonMarkCache::default(),
            dark_mode: true,
            font_scale: 1.0,
            font_family: FontFamily::Proportional,
            edit_mode: false,
            base_style: None,
        }
    }
}

impl MdViewer {
    fn open_file(&mut self, path: PathBuf) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            self.markdown = content;
            self.file_path = Some(path);
            self.edit_mode = false;
        }
    }

    fn save_file(&self) {
        if let Some(path) = &self.file_path {
            let _ = std::fs::write(path, &self.markdown);
        }
    }

    fn title(&self) -> String {
        let name = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("untitled");
        let mode = if self.edit_mode { " (editing)" } else { "" };
        format!("{}{} \u{2014} MdViewer", name, mode)
    }

    fn open_dialog(&mut self) {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "mdown", "mdwn"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            self.open_file(file);
        }
    }

    fn extract_links(text: &str) -> Vec<String> {
        let parser = Parser::new(text);
        let mut links = Vec::new();
        for event in parser {
            if let Event::Start(Tag::Link { dest_url, .. }) = event {
                links.push(dest_url.to_string());
            }
        }
        links
    }
}

impl eframe::App for MdViewer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.title()));

        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        if self.base_style.is_none() {
            self.base_style = Some((*ctx.global_style()).clone());
        }

        if let Some(base) = &self.base_style {
            let mut style = base.clone();
            for (ts, font_id) in style.text_styles.iter_mut() {
                let base_size = base.text_styles.get(ts).map_or(16.0, |f| f.size);
                font_id.size = base_size * self.font_scale;
                if *ts != TextStyle::Monospace {
                    font_id.family = self.font_family.clone();
                }
            }
            ctx.set_global_style(style);
        }

        let dropped = ctx.input(|i| {
            i.raw.dropped_files.first().and_then(|f| f.path.clone())
        });
        if let Some(path) = dropped {
            self.open_file(path);
        }

        let ctrl_o = ctx.input(|i| i.key_pressed(Key::O) && i.modifiers.ctrl);
        let ctrl_s = ctx.input(|i| i.key_pressed(Key::S) && i.modifiers.ctrl);

        if ctrl_o {
            self.open_dialog();
        }
        if ctrl_s && self.edit_mode {
            self.save_file();
        }

        egui::Panel::top("toolbar")
            .min_size(40.0)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(8.0, 3.0);
                    ui.strong("MdViewer");
                    ui.separator();

                    if ui.button("Open").clicked() || ctrl_o {
                        self.open_dialog();
                    }

                    if self.file_path.is_some() {
                        if self.edit_mode {
                            if ui.button("Save").clicked() || ctrl_s {
                                self.save_file();
                            }
                            ui.separator();
                        }

                        if ui.button("Reload").clicked() {
                            if let Some(path) = &self.file_path.clone() {
                                self.open_file(path.clone());
                            }
                        }

                        ui.separator();

                        let (mode_icon, mode_label) = if self.edit_mode {
                            ("\u{1F4D6}", "Read")
                        } else {
                            ("\u{270F}", "Edit")
                        };
                        if ui
                            .selectable_label(self.edit_mode, format!("{} {}", mode_icon, mode_label))
                            .clicked()
                        {
                            self.edit_mode = !self.edit_mode;
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.style_mut().spacing.button_padding = egui::vec2(4.0, 2.0);
                        ui.style_mut().spacing.item_spacing.x = 4.0;

                        let icon = if self.dark_mode { "\u{2600}" } else { "\u{1F319}" };
                        let label = if self.dark_mode { "Light" } else { "Dark" };
                        if ui.button(format!("{} {}", icon, label)).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }

                        ui.separator();

                        if ui.button("A-").clicked() {
                            self.font_scale = (self.font_scale - 0.1).max(0.6);
                        }
                        ui.label(format!("{:3.0}%", self.font_scale * 100.0))
                            .on_hover_text("Font size");
                        if ui.button("A+").clicked() {
                            self.font_scale = (self.font_scale + 0.1).min(2.0);
                        }

                        ui.separator();

                        let families = [FontFamily::Proportional, FontFamily::Monospace];
                        let family_names = ["Default", "Monospace"];
                        let current = families
                            .iter()
                            .position(|f| *f == self.font_family)
                            .unwrap_or(0);
                        egui::ComboBox::new("font_family", "")
                            .selected_text(family_names[current])
                            .show_ui(ui, |ui| {
                                for (i, name) in family_names.iter().enumerate() {
                                    if ui.selectable_label(current == i, *name).clicked() {
                                        self.font_family = families[i].clone();
                                    }
                                }
                            });
                    });
                });
            });

        if self.markdown.is_empty() {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() * 0.35);
                    ui.strong("Markdown Viewer");
                    ui.label("Open a .md file or drag-and-drop one here");
                    ui.add_space(8.0);
                    if ui.button("Open File").clicked() {
                        self.open_dialog();
                    }
                    ui.add_space(4.0);
                    ui.small("Supports .md, .markdown, .mdown files");
                    ui.small("Ctrl+O to open, Ctrl+S to save (edit mode)");
                });
            });
        } else if self.edit_mode {
            egui::CentralPanel::default()
                .frame(egui::Frame::central_panel(ui.style()).inner_margin(egui::Margin::symmetric(4, 4)))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let te = egui::TextEdit::multiline(&mut self.markdown)
                                .font(TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .lock_focus(true)
                                .code_editor();
                            ui.add(te);
                        });
                });
        } else {
            let links = Self::extract_links(&self.markdown);
            for url in &links {
                self.cache.add_link_hook(url.clone());
            }

            let ctrl = ctx.input(|i| i.modifiers.ctrl);

            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(4.0);
                        ui.style_mut().url_in_tooltip = true;
                        egui_commonmark::CommonMarkViewer::new()
                            .show(ui, &mut self.cache, &self.markdown);
                    });
            });

            for url in &links {
                if self.cache.get_link_hook(url) == Some(true) {
                    if ctrl {
                        ctx.open_url(egui::OpenUrl::new_tab(url.clone()));
                    }
                }
            }

            self.cache.link_hooks_clear();
        }

        egui::Panel::bottom("status")
            .min_size(24.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 4.0;
                    if let Some(path) = &self.file_path {
                        let mode_tag = if self.edit_mode { " [EDITING]" } else { "" };
                        ui.label(format!("\u{1F4C4} {}{}", path.display(), mode_tag));
                        ui.separator();
                        ui.label(format!("{} chars", self.markdown.len()));
                        let lines = self.markdown.lines().count();
                        ui.label(format!("{} lines", lines));
                        if self.edit_mode {
                            ui.separator();
                            ui.small("Ctrl+S to save");
                        }
                    } else {
                        ui.label("No file open");
                    }
                });
            });
    }
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let mut app = MdViewer::default();
    if args.len() > 1 {
        app.open_file(PathBuf::from(&args[1]));
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 700.0])
            .with_min_inner_size([480.0, 360.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "MdViewer",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
