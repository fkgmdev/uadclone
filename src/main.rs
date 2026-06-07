use eframe::egui;
mod adb;
mod packages;
fn main() -> eframe::Result {
    let installed_packages = adb::get_packages();
    let packages = packages::load_package_list();
    
    eframe::run_ui_native("placeholder_name", Default::default(), move |ctx, _frame| {
        egui::CentralPanel::default().show_inside(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for pkg in &installed_packages {
                let pkgdb = packages.get(pkg);
                
                match pkgdb {
                    Some(info) => {
                        ui.label(format!("Package: {}\n\nDescription: {}", pkg, info.description));
                        ui.add_space(10.0);
                        ui.label(format!("Removal: {}", info.removal));
                        ui.add_space(35.0);
                    }
                    None => {}
                };
            }
            });
            
        });
    })

}
