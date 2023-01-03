#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Button, Vec2, Key, Color32};
use twentyfortyeight::*;

mod twentyfortyeight;

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(440.0, 520.0)),

        ..Default::default()
    };
    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Box::new(TwentyFortyEightApp::default())),
    );
}

struct TwentyFortyEightApp {
    board: Board,
}

impl Default for TwentyFortyEightApp {
    fn default() -> Self { 
        TwentyFortyEightApp { board: Board::new() }
    }
}

impl eframe::App for TwentyFortyEightApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
                let res = ui.button("New Game").clicked();
                if res {
                    self.board = Board::new();
                }
            });
            egui::Grid::new("grid").spacing([10.0, 10.0]).show(ui, |ui| {
                for arr in self.board.cells {
                    for v in arr {
                        let text = if v == 0 { String::from("") } else { format!("{}", 2_u128.pow(v as u32)) };
                        let b = Button::new(text).min_size(Vec2 { x: 50.0, y: 50.0 }).fill(v.color());
                        ui.add(b);
                    }
                    ui.end_row();
                }
            });

            if ctx.input().key_pressed(Key::ArrowRight) {
                self.board.shift(Direction::Right);
            } else if ctx.input().key_pressed(Key::ArrowLeft) {
                self.board.shift(Direction::Left);
            } else if ctx.input().key_pressed(Key::ArrowUp) {
                self.board.shift(Direction::Up);
            } else if ctx.input().key_pressed(Key::ArrowDown) {
                self.board.shift(Direction::Down);
            }

        });
    }
}

pub trait Colorable {
    fn color(&self) -> Color32;
}

impl Colorable for u8 {
    fn color(&self) -> Color32 {
        match self {
            0 => Color32::GRAY,
            1 => Color32::BROWN,
            2 => Color32::KHAKI,
            3 => Color32::LIGHT_RED,
            4 => Color32::RED,
            5 => Color32::DARK_RED,
            6 => Color32::LIGHT_YELLOW,
            _ => Color32::YELLOW
        }
    }
}