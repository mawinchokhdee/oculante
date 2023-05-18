use super::utils::SUPPORTED_EXTENSIONS;
use anyhow::{Context, Result};
use dirs;
use notan::egui::{self, *};
use std::io::Write;
use std::{
    fs::{self, read_to_string, File},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref STATE: Arc<Mutex<FileBrowser>> = Arc::new(Mutex::new(
        FileBrowser {
            open: true,
            current_path: load_recent_dir().unwrap_or_default(),
            current_manual_path: load_recent_dir().unwrap_or_default().to_string_lossy().to_string()
        }));
}

fn load_recent_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(read_to_string(
        dirs::cache_dir()
            .context("Can't get temp dir")?
            .join(".efd_history"),
    )?))
}

fn save_recent_dir(p: &Path) -> Result<()> {
    let p = if p.is_file() {
        p.parent().context("Can't get parent")?.to_path_buf()
    } else {
        p.to_path_buf()
    };

    let mut f = File::create(
        dirs::cache_dir()
            .context("Can't get temp dir")?
            .join(".efd_history"),
    )?;
    write!(f, "{}", p.to_string_lossy())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct FileBrowser {
    /// Whether or not to show the file browser
    pub open: bool,
    // pub contents: Vec<?i32>,
    pub current_path: PathBuf,
    pub current_manual_path: String,
}

pub fn browse<F: FnMut(Option<&PathBuf>)>(mut callback: F, ui: &mut Ui) {
    let mut state = STATE.lock().unwrap();
    // if !state.open {
    //     return;
    // }

    let mut open = true;

    egui::Window::new("Browse")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .open(&mut open)
        .resizable(true)
        .default_width(600.)
        .default_height(600.)
        // .auto_sized()
        .show(ui.ctx(), |ui| {
            if ui
                .add(
                    egui::TextEdit::singleline(&mut state.current_manual_path)
                        .cursor_at_end(true)
                        .desired_width(ui.available_width()),
                )
                .changed()
            {
                let p = PathBuf::from(&state.current_manual_path);
                if p.exists() {
                    state.current_path = p;
                }
            }
            let d = fs::read_dir(&state.current_path).ok();
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(100., ui.available_height()),
                    Layout::top_down_justified(Align::LEFT),
                    |ui| {
                        if let Some(d) = dirs::desktop_dir() {
                            if ui.button("💻 Desktop").clicked() {
                                state.current_manual_path = d.to_string_lossy().to_string();
                                state.current_path = d;
                            }
                        }
                        if let Some(d) = dirs::home_dir() {
                            if ui.button("🏠 Home").clicked() {
                                state.current_manual_path = d.to_string_lossy().to_string();
                                state.current_path = d;
                            }
                        }
                        if let Some(d) = dirs::document_dir() {
                            if ui.button("🗋 Documents").clicked() {
                                state.current_manual_path = d.to_string_lossy().to_string();
                                state.current_path = d;
                            }
                        }
                        if let Some(d) = dirs::picture_dir() {
                            if ui.button("🖻 Pictures").clicked() {
                                state.current_manual_path = d.to_string_lossy().to_string();
                                state.current_path = d;
                            }
                        }
                    },
                );

                ui.vertical(|ui| {
                    if ui.button("🔺 Up").clicked() {
                        if let Some(d) = state.current_path.parent() {
                            let p = d.to_path_buf();
                            state.current_manual_path = p.to_string_lossy().to_string();
                            state.current_path = p;
                        }
                    }

                    egui::ScrollArea::new([false, true])
                        // .max_width(500.)
                        .min_scrolled_height(500.)
                        .auto_shrink([true, false])
                        .show(ui, |ui| match d {
                            Some(contents) => {
                                egui::Grid::new("browser")
                                    .striped(true)
                                    .num_columns(0)
                                    .min_col_width(ui.available_width())
                                    .show(ui, |ui| {
                                        for de in contents
                                            .into_iter()
                                            .flat_map(|x| x)
                                            .filter(|de| {
                                                !de.file_name().to_string_lossy().starts_with(".")
                                            })
                                            .filter(|de| {
                                                de.path().is_dir()
                                                    || SUPPORTED_EXTENSIONS.contains(
                                                        &de.path()
                                                            .extension()
                                                            .map(|ext| {
                                                                ext.to_string_lossy().to_string()
                                                            })
                                                            .unwrap_or_default()
                                                            .to_lowercase()
                                                            .as_str(),
                                                    )
                                            })
                                        {
                                            if de.path().is_dir() {
                                                if ui
                                                    .add(
                                                        egui::Button::new(format!(
                                                            "🗀 {}",
                                                            de.file_name()
                                                                .to_string_lossy()
                                                                .chars()
                                                                .take(100)
                                                                .collect::<String>()
                                                        ))
                                                        .frame(false),
                                                    )
                                                    .clicked()
                                                {
                                                    state.current_path = de.path();
                                                }
                                            } else {
                                                if ui
                                                    .add(
                                                        egui::Button::new(format!(
                                                            "🖹 {}",
                                                            de.file_name().to_string_lossy()
                                                        ))
                                                        .frame(false),
                                                    )
                                                    .clicked()
                                                {
                                                    callback(Some(&de.path()));
                                                    _ = save_recent_dir(&de.path());

                                                    // state.open = false;
                                                }
                                            }
                                            ui.end_row();
                                        }
                                    });
                            }
                            None => {
                                ui.label("no contents");
                            }
                        });
                });
            });
        });

    if !open {
        callback(None);
    }
}
