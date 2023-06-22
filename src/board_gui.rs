use eframe::egui::{self, ScrollArea};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;

use crate::board::Board;

pub struct BoardGUI {
    pub obj: Board,
    pub edit_node_key: Option<i32>,
    pub edit_edge_key: Option<i32>,
    pub to_node_key: String,
}

impl BoardGUI {
    fn default() -> Self {
        Self {
            obj: Board::new(),
            edit_node_key: None,
            edit_edge_key: None,
            to_node_key: String::new(),
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> BoardGUI {
        let ctx = &cc.egui_ctx;
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(18.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(14.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        BoardGUI::default()
    }

    fn edit_node(&mut self, ctx: &eframe::egui::Context, key: i32) {
        egui::Window::new(format!("Edit node, id: {}", key)).show(ctx, |ui| {
            let node = self
                .obj
                .nodes
                .get_mut(&key)
                .expect(&format!("Node {} not found!", key));

            ui.horizontal(|ui| {
                ui.label("name:");
                ui.text_edit_singleline(&mut node.name);
            });
            ui.label("notes:");
            ui.text_edit_multiline(&mut node.notes);
            ui.horizontal(|ui| {
                ui.label("to key:");
                ui.text_edit_singleline(&mut self.to_node_key);
                if ui.button("new edge").clicked() {
                    if let Ok(to_key) = self.to_node_key.parse::<i32>() {
                        self.obj.add_edge(key, to_key);
                    }
                }
            });
            ui.label("edges:");

            ScrollArea::vertical()
                .auto_shrink([true, true])
                .show(ui, |ui| self.edge_lines(ui, &key));

            if ui.button("close").clicked() {
                self.edit_node_key = None;
            }
        });
    }

    fn edit_edge(&mut self, ctx: &eframe::egui::Context, key: i32) {
        let edge = self
            .obj
            .edges
            .get_mut(&key)
            .expect(&format!("Edge {} not found!", key));

        egui::Window::new(format!(
            "Edit edge, id: {}, from: {} to: {}",
            key, edge.from, edge.to
        ))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("name:");
                ui.text_edit_singleline(&mut edge.name);
            });
            ui.label("notes:");
            ui.text_edit_multiline(&mut edge.notes);
            ui.horizontal(|ui| {
                if ui.button("edit from").clicked() {
                    self.edit_edge_key = None;
                    self.edit_node_key = Some(edge.from);
                }

                if ui.button("edit to").clicked() {
                    self.edit_edge_key = None;
                    self.edit_node_key = Some(edge.to);
                }
            });

            if ui.button("close").clicked() {
                self.edit_edge_key = None;
            }
        });
    }

    fn node_lines(&mut self, ui: &mut eframe::egui::Ui) {
        let mut sorted: Vec<i32> = self.obj.nodes.keys().copied().collect();
        sorted.sort_by_key(|a| -a);

        for key in &sorted {
            ui.horizontal(|ui| {
                ui.label(format!("id: {}, name:", key));
                ui.text_edit_singleline(&mut self.obj.nodes.get_mut(key).expect("").name);

                if ui.button("del").clicked() {
                    self.obj.remove_node(key);
                }

                if ui.button("edit").clicked() {
                    self.edit_node_key = Some(*key);
                }
            });
        }
    }

    fn edge_lines(&mut self, ui: &mut eframe::egui::Ui, node_key: &i32) {
        let node = self.obj.nodes.get_mut(node_key).expect("");

        let mut sorted: Vec<i32> = node.edges.iter().copied().collect();
        sorted.sort_by_key(|a| -a);

        for key in &sorted {
            ui.horizontal(|ui| {
                ui.label(format!("id: {}, name:", key));

                let edge = self.obj.edges.get_mut(key).expect("No edge!");
                ui.text_edit_singleline(&mut edge.name);

                let to_key = if *node_key != edge.to {
                    edge.to
                } else {
                    edge.from
                };
                let to_node = self.obj.nodes.get(&to_key).expect("No node!");

                ui.label(format!("to id: {}, name: {}", edge.to, to_node.name));

                if ui.button("edit to").clicked() {
                    self.edit_node_key = Some(to_key);
                }

                if ui.button("del").clicked() {
                    self.obj.remove_edge(key);
                }

                if ui.button("edit").clicked() {
                    self.edit_edge_key = Some(*key);
                }
            });
        }
    }
}

impl eframe::App for BoardGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Nodes ({}):", self.obj.nodes.len()));
            ui.horizontal(|ui| {
                if ui.button("new node").clicked() {
                    self.obj.add_node();
                }
            });

            ScrollArea::vertical()
                .auto_shrink([true, true])
                .show(ui, |ui| self.node_lines(ui));

            ui.horizontal(|ui| {
                if ui.button("save").clicked() {
                    self.obj.save(None);
                }

                if ui.button("load").clicked() {
                    self.obj = Board::load(None);
                }
            });
        });

        if let Some(key) = self.edit_node_key {
            self.edit_node(ctx, key);
        }

        if let Some(key) = self.edit_edge_key {
            self.edit_edge(ctx, key);
        }
    }
}
