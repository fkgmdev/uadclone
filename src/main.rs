use crate::packages::Package;
use eframe::egui;
use std::collections::HashMap;

mod adb;
mod packages;

struct App {
    installed_packages: Vec<String>,
    packages: HashMap<String, Package>,
    query: String,
}

impl App {
    fn new() -> Self {
        Self {
            installed_packages: adb::get_packages(),
            packages: packages::load_package_list(),
            query: String::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.query));
            egui::ScrollArea::vertical().show(ui, |ui| {
                for pkg in &self.installed_packages {
                    let pkgdb = self.packages.get(pkg);

                    match pkgdb {
                        Some(info) => {
                            if pkg.contains(&self.query)
                                || info.description.contains(&self.query)
                                || self.query.is_empty()
                            {
                                ui.label(format!(
                                    "Package: {}\n\nDescription: {}",
                                    pkg, info.description
                                ));
                                ui.add_space(10.0);
                                ui.label(format!("Removal: {}", info.removal));
                                ui.add_space(35.0);
                            }
                        }
                        None => {}
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let a = packages::load_package_list(); // HashMap<String, Package>
    /*
    for pkg in adb::get_packages() {
        // .contains_key works with &String or &str automatically
        if a.contains_key(&pkg) {
            println!("Found matching package: {}", pkg);
        }
    } */
    eframe::run_native(
        "placeholder_name",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

