use notan::egui::{self, *};
use super::appstate::OculanteState;
use std::fs;
use super::utils::SUPPORTED_EXTENSIONS;

#[derive(Debug, Clone)]
pub struct FileBrowser {
    /// Whether or not to show the file browser
    pub open: bool,
    pub contents: Vec<i32>,
}


pub fn browse(state: &mut OculanteState, ui: &mut Ui) {

    let d = fs::read_dir(&state.persistent_settings.last_open_directory).ok();

    egui::Window::new("Browse").min_width(600.0).show(ui.ctx(), |ui| {

        ui.vertical(|ui| {
            ui.label("text");
            ui.label("text");
            ui.label("text");
            ui.label("text");
            ui.separator();
        });
    
    match d {
        Some(contents) => {

            if ui.button("up").clicked() {
                if let Some(p) = state.persistent_settings.last_open_directory.parent() {
                    state.persistent_settings.last_open_directory = p.into();
                }
            }

            for de in contents.into_iter()
            .flat_map(|x|x)
            .filter(|de| !de.file_name().to_string_lossy().starts_with(".")) 
            .filter(|de| de.path().is_dir() || SUPPORTED_EXTENSIONS.contains(&de.path().extension().map(|ext|ext.to_string_lossy().to_string()).unwrap_or_default().to_lowercase().as_str())) 
            
            {
                if de.path().is_dir() {
                    if ui.add(egui::Button::new(format!("🗀 {}", de.file_name().to_string_lossy())).frame(false)).clicked() {
                        state.persistent_settings.last_open_directory = de.path();

                    }

                    // if ui.button(format!("🗀 {}", de.file_name().to_string_lossy())).clicked() {
                    //     state.persistent_settings.last_open_directory = de.path();
                    // };
                } else {

                    // ui.label(format!("🖹 {}", de.file_name().to_string_lossy()));
                    if ui.add(egui::Button::new(format!("🖹 {}", de.file_name().to_string_lossy())).frame(false)).clicked() {
                        // debug!("Selected File Path = {:?}", file_path);
                        state.is_loaded = false;
                        state.current_image = None;
                        state
                            .player
                            .load(&de.path(), state.message_channel.0.clone());
                        if let Some(dir) = de.path().parent() {
                            state.persistent_settings.last_open_directory = dir.to_path_buf();
                        }
                        state.current_path = Some(de.path());

                    }
                }
            }
        },
        None => {ui.label("no contents");}
    }

     });
}