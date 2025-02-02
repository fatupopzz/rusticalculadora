mod calculator;
mod ui;

use calculator::Calculator;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    Calculator::run(Settings {
        window: iced::window::Settings { 
            size: (230, 200), // Tamaño más compacto como la referencia
            resizable: false,
            decorations: true,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}