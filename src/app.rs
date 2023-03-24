use std::collections::HashMap;

use eframe::{App, epaint::tessellator::path};
use egui::Ui;

use crate::{misc, deserializer::{JsonStore, deserialize_json, payload_to_string}}; 

#[derive(Clone)]
pub struct WindowStore { // Per window data
    window_id: String,
    save_folder_path: String,
    is_window_open: bool,
    deseralized_jsons: Option<Vec<JsonStore>>
}

#[derive(Clone)]
pub struct SotfSaveAnalyserApp { // Essentially global vars
    label: String,
    is_open_file_clicked: bool,
    window_stores: HashMap<String,WindowStore>
}

impl Default for SotfSaveAnalyserApp {

    fn default() -> Self {
        Self {
            label: "Sons of the Forest Save Analyser".to_owned(),
            is_open_file_clicked: false,
            window_stores: HashMap::new(),
        }
    }
    
}

impl SotfSaveAnalyserApp { 

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn new_save_analyser_window(&mut self, path: String) {

        let post_append_id = self.window_stores.len().to_string();
        let mut window_id = String::from("Save Analysis #");
        window_id.push_str(&post_append_id);

        let store = WindowStore {
            window_id: window_id.clone(),
            save_folder_path: path,
            is_window_open: true,
            deseralized_jsons: Option::None,
        };

        self.window_stores.insert(window_id, store);

    }

    pub fn recursive_json_viewer(&self, json_stores: Vec<JsonStore>, indent_char: char, indent_counter: &mut u128, ctx: &egui::Context, ui: &mut Ui) {

        for j in json_stores {
            let mut indent_string = String::from("");
            for n in 0..=indent_counter.clone() {
                indent_string.push(indent_char);
            }

            if j.payload.is_some() {

                let mut ui_text = String::from(&indent_string);
                // ui_text.push_str(r#"""#);
                ui_text.push_str(&j.field);
                // ui_text.push_str(r#"""#);
                ui_text.push_str(r#": "#);
                
                let payload = payload_to_string(j.payload.unwrap());

                ui_text.push_str(&payload);

                ui.strong(ui_text);
                // if *indent_counter > 0 {
                //     *indent_counter -= 1;
                // }

            }

            if j.children.is_some() && j.field != "VailWorldSim" {
                let mut child_preamble = String::from("");
                child_preamble.push_str(&indent_string);
                child_preamble.push_str(&j.field);
                child_preamble.push(':');

                ui.strong(child_preamble);
                
                *indent_counter += 1;

                self.recursive_json_viewer(j.children.unwrap(), indent_char, indent_counter, ctx, ui);
            }


        }

        if *indent_counter > 0 {
            *indent_counter -= 1;
        }

    }

    pub fn window_backend(&mut self, ctx: &egui::Context) {
        let mut delete_window_data: Vec<String> = vec![]; // Ids of orphaned window data (window closed), entries here are essentially for marking data stores for removal
        let mut modified_window_stores = self.window_stores.clone(); // Modify this so that the data isn't changed while iterating; There's probably a better way

        for (window_id, window_contents) in &self.window_stores  { // Alternatively could clone here and get rid of the modified hashmap

            // When user closes the window, the flag is set to false, and we'll purge the stored data that window was using
            if window_contents.is_window_open == false {
                delete_window_data.push(window_id.clone());
                continue;
            }

            // Check if deserialization is complete
            if window_contents.deseralized_jsons.is_none() && window_contents.save_folder_path.len() > 0 { // Init deserialization
                let all_json_data = deserialize_json(window_contents.save_folder_path.clone());

                if all_json_data.is_empty() { // TODO: Corner case
                    continue;
                } else { // Make more efficent?
                    let mut modified_window_contents = window_contents.clone();
                    modified_window_contents.deseralized_jsons = Option::Some(all_json_data);

                    modified_window_stores.insert(window_id.clone(), modified_window_contents);
                    continue;
                }

                
            } else if !window_contents.deseralized_jsons.is_some() || window_contents.deseralized_jsons.is_none() { // Assuming path is empty somehow
                println!("WARN: Window Store of id {} has empty file path and no deseralized JSON data.", window_id);
                // delete_window_data.push(window_id.clone()); // ? Should I just delete the window?
                continue;
            }

            // BUILD THE WINDOW
            let mut window_open_flag = window_contents.is_window_open.clone();

            egui::Window::new(window_id.to_string().as_str())
            .vscroll(true)
            .collapsible(true)
            .open(&mut window_open_flag)
            .show(ctx, |ui| {

                // Assume not None
                let json = window_contents.clone().deseralized_jsons.unwrap();

                let mut tab_counter: u128 = 0;

                self.recursive_json_viewer(json, '\t', &mut tab_counter, ctx, ui);
            
            });
            if window_open_flag != window_contents.is_window_open {
                let x = modified_window_stores.get_mut(&window_id.clone());
                if x.is_some() {
                    let y = x.unwrap();
                    y.is_window_open = window_open_flag;
                    // TODO: See if I need to re-insert
                }
            }
                


        }

        // Updates the window store
        self.window_stores = modified_window_stores;

        // Delete closed window data
        for id in delete_window_data {
            self.window_stores.remove(&id);
        }

    }

    // pub fn file_picker_window(&mut self, ctx: &egui::Context) {

    //     egui::Window::new("Open File")
    //         .open(&mut self.is_open_file_clicked)
    //         .show(ctx, |ui2| {

    //             let mut picked_path = Option::None;

    //             if ui2.button("Open file…").clicked() {
    //                 if let Some(path) = rfd::FileDialog::new().pick_file() { // TODO: Pontentially make Async for wasm
    //                     picked_path = Some(path.display().to_string());
    //                 }
    //             }


    //             let mut is_valid = false;
    //             if picked_path.is_some() {

    //                is_valid = misc::validate_path_gui(&picked_path.clone().unwrap()); // Atm only checks if file exists (It should, but you know, users...)

    //                if !is_valid {
    //                    ui2.strong("An error has occured with the file you provided at this path:");
    //                    ui2.monospace(&picked_path.clone().unwrap());
    //                    ui2.label("Please open a file again, and select a valid `SaveData.json` file from your Sons of the Forest save folder.");
    //                } else {
    //                    self.new_save_analyser_window(picked_path.unwrap());
    //                }

    //             }

    //             // ui.strong("ERROR Loading file with path: ");
    //             // ui.monospace(picked_path);
    //             // ui.label("Please provide the SaveData.json file.");
    //         });

    // }

}

impl App for SotfSaveAnalyserApp {

    // Draws every frame
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::SidePanel::left("side_panel").show(ctx, |ui| {

            if ui.button("Open save folder…").clicked() {
                self.is_open_file_clicked = !self.is_open_file_clicked;
            }


            if self.is_open_file_clicked {
            
                let mut picked_path = Option::None;

                if let Some(path) = rfd::FileDialog::new().pick_folder() { // TODO: Pontentially make Async for wasm
                    picked_path = Some(path.display().to_string());
                }


                let mut is_valid = false;
                if picked_path.is_some() {

                   is_valid = misc::validate_path_gui(&picked_path.clone().unwrap()); // Atm only checks if file exists (It should, but you know, users...)

                   if !is_valid {
                       ui.strong("An error has occured with the file you provided at this path:");
                       ui.monospace(&picked_path.clone().unwrap());
                       ui.label("Please open a file again, and select a valid `SaveData.json` file from your Sons of the Forest save folder.");
                   } else {
                       self.new_save_analyser_window(picked_path.unwrap());
                   }

                }

                self.is_open_file_clicked = false;
            }

        });



        self.window_backend(ctx);

    }
}