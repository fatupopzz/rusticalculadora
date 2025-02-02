use iced::widget::button;
use iced::{Background, Color, Vector, theme};

#[derive(Debug, Clone, Copy)]
pub enum Button {
    Ans,
    Num,
    Func,
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Button::Ans => Color::from_rgb(0.27, 0.60, 0.86),  // Azul
                Button::Num => Color::from_rgb(0.56, 0.93, 0.56),  // Verde claro
                Button::Func => Color::from_rgb(0.85, 0.85, 0.85), // Gris
            })),
            border_radius: iced::BorderRadius::from(4.0),
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: match self {
                Button::Ans => Color::WHITE,
                _ => Color::BLACK,
            },
            ..button::Appearance::default()
        }
    }
}