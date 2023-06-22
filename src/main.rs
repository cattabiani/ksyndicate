mod board;
mod board_gui;
mod edge;
mod node;
mod tag;

use board_gui::BoardGUI;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ksyndicate",
        native_options,
        Box::new(|cc| Box::new(BoardGUI::new(cc))),
    )
}
