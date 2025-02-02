use iced::widget::{button, Button, Column, Row, Text, container};
use iced::{Alignment, Command, Element, Length, Sandbox, Theme};
use iced::alignment::Horizontal;
use iced::theme;
use iced::theme::Button as ThemeButton;
use iced::Background;
use iced::Color;

const LEN: u32 = 450;

#[derive(Debug, Clone)]
pub enum Message {
    Num(char),
    Sign(char),
    Ans,
    Dot,
    Neg,
    Clear,
    ClearEnd,
    Backspace,
    Memory(char),  // MC, MR, MS, M+
    Special(char), // sqrt, 1/x, %
    }

#[derive(Debug, Clone, Copy)]
pub enum ButtonType {
    Number,
    Memory,     // MC, MR, MS, M+
    Operation,  // /, *, -, +
    Special,    // sqrt, 1/x, %
    Equal,      // =
    Clear,      // C, CE, Backspace
}

pub struct CustomButtonStyle {
    pub button_type: ButtonType,
}


// Ajustamos los colores del botón para el estilo Windows XP
impl button::StyleSheet for CustomButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.88, 1.0, 0.88))), // Verde claro
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(0.75, 0.75, 0.85), // Borde azulado
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let background = active.background.unwrap();
        if let Background::Color(color) = background {
            button::Appearance {
                background: Some(Background::Color(Color::from_rgba(
                    color.r - 0.05,
                    color.g - 0.05,
                    color.b - 0.05,
                    color.a
                ))),
                ..active
            }
        } else {
            active
        }
    }


    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.95, 0.99))),
            ..active
        }
    }
}   


pub struct Calculator {
    left: String,
    right: String,
    sign: String,
    shadow: bool,
    memory: f64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            left: String::from("0"),
            right: String::new(),
            sign: String::new(),
            shadow: false,
            memory: 0.0,
        }
    }
}

impl Calculator {
    fn calculate(&mut self) -> Result<(), &'static str> {
        self.shadow = true;
        self.left = match self.sign.as_str() {
            "+" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() + self.right.parse::<f64>().unwrap())
            },
            "-" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() - self.right.parse::<f64>().unwrap())
            },
            "×" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() * self.right.parse::<f64>().unwrap())
            },
            "÷" => {
                let r = self.right.parse::<f64>().unwrap();
                if r == 0.0 {
                    "Error: División por cero".into()
                } else {
                    format!("{:.10}", self.left.parse::<f64>().unwrap() / r)
                }
            },
            _ => unreachable!()
        };
        self.cleanup_number();
        Ok(())
    }

    fn cleanup_number(&mut self) {
        while self.left.ends_with('0') && self.left.contains('.') {
            self.left.pop();
        }
        if self.left.ends_with('.') {
            self.left.pop();
        }
    }

    fn clear(&mut self, c: char) {
        self.left = c.into();
        self.sign.clear();
        self.right.clear();
        self.shadow = false;
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.9))),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
            ..Default::default()
        }
    }
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("RustiCalculadora")
    }

    fn update(&mut self, message: Message) -> () {
        match message {
            Message::Num(n) => {
                if self.sign.is_empty() {
                    if &self.left == "0" {
                        self.left = n.to_string();
                    } else {
                        self.left.push(n);
                    }
                } else if !self.left.is_empty() && self.shadow {
                    self.clear(n);
                } else if &self.right == "0" {
                    self.right = n.to_string();
                } else {
                    self.right.push(n);
                }
            },
            Message::Sign(s) => {
                if self.sign.is_empty() {
                    self.sign.push(s);
                } else {
                    if !self.shadow && !self.right.is_empty() {
                        let _ = self.calculate();
                    }
                    self.sign = s.to_string();
                    self.right.clear();
                    self.shadow = false;
                }
            },
            Message::Ans => {
                if !self.sign.is_empty() && !self.left.is_empty() && !self.right.is_empty() {
                    let _ = self.calculate();
                }
            },
            Message::Clear => self.clear('0'),
            Message::ClearEnd => {
                if self.right.is_empty() {
                    self.left = "0".to_string();
                    self.sign.clear();
                    self.shadow = false;
                } else {
                    self.right.clear();
                }
            },
            Message::Backspace => {
                if self.sign.is_empty() {
                    self.left.pop();
                    if self.left.is_empty() {
                        self.left.push('0');
                    }
                } else {
                    self.right.pop();
                }
            },
            Message::Dot => {
                if self.sign.is_empty() && !self.left.contains('.') {
                    self.left.push('.');
                } else if !self.sign.is_empty() && !self.right.contains('.') {
                    if self.right.is_empty() {
                        self.right.push('0');
                    }
                    self.right.push('.');
                }
            },
            Message::Neg => {
                if self.sign.is_empty() {
                    if &self.left != "0" && &self.left != "0." {
                        if !self.left.starts_with('-') {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                } else if !self.sign.is_empty() && self.shadow {
                    if &self.left != "0" {
                        if !self.left.starts_with('-') {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                }
            },
            Message::Memory(m) => {
                match m {
                    'C' => self.memory = 0.0, // MC
                    'R' => self.left = self.memory.to_string(), // MR
                    'S' => self.memory = self.left.parse().unwrap_or(0.0), // MS
                    '+' => self.memory += self.left.parse::<f64>().unwrap_or(0.0), // M+
                    _ => {}
                }
            },
            Message::Special(s) => {
                match s {
                    's' => { // sqrt
                        let num = self.left.parse::<f64>().unwrap_or(0.0);
                        if num >= 0.0 {
                            self.left = num.sqrt().to_string();
                        }
                    },
                    'i' => { // 1/x
                        let num = self.left.parse::<f64>().unwrap_or(0.0);
                        if num != 0.0 {
                            self.left = (1.0/num).to_string();
                        }
                    },
                    '%' => {
                        let num = self.left.parse::<f64>().unwrap_or(0.0);
                        self.left = (num/100.0).to_string();
                    },
                    _ => {}
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let display_text = if self.shadow {
            self.left.clone()
        } else {
            format!("{} {} {}", self.left, self.sign, self.right)
        };
    
        container(
            Column::new()
                .spacing(1)
                .padding(1)
                .push(
                    Row::new()
                        .push(Text::new("Ver").size(11))
                        .push(Text::new("Edición").size(11))
                        .push(Text::new("Ayuda").size(11))
                        .spacing(5)
                        .padding(2)
                )
                .push(
                    container(
                        Text::new(display_text)
                            .size(16)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Right)
                    )
                    .padding(3)
                    .style(theme::Container::Custom(Box::new(ContainerStyle)))
                    .width(Length::Fill)
                    .height(Length::Fixed(25.0))
                )
                .push(
                    Row::new()
                        .spacing(1)
                        .push(
                            Button::new(Text::new("←").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Clear })))
                                .on_press(Message::Backspace)
                        )
                        .push(
                            Button::new(Text::new("CE").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Clear })))
                                .on_press(Message::ClearEnd)
                        )
                        .push(
                            Button::new(Text::new("C").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Clear })))
                                .on_press(Message::Clear)
                        )
                )
                .push(
                    Row::new()
                        .spacing(1)
                        .push(
                            Button::new(Text::new("MC").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Memory })))
                                .on_press(Message::Memory('C'))
                        )
                        .push(
                            Button::new(Text::new("7").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('7'))
                        )
                        .push(
                            Button::new(Text::new("8").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('8'))
                        )
                        .push(
                            Button::new(Text::new("9").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('9'))
                        )
                        .push(
                            Button::new(Text::new("/").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Operation })))
                                .on_press(Message::Sign('÷'))
                        )
                        .push(
                            Button::new(Text::new("sqrt").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Special })))
                                .on_press(Message::Special('s'))
                        )
                )
                .push(
                    Row::new()
                        .spacing(1)
                        .push(
                            Button::new(Text::new("MR").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Memory })))
                                .on_press(Message::Memory('R'))
                        )
                        .push(
                            Button::new(Text::new("4").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('4'))
                        )
                        .push(
                            Button::new(Text::new("5").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('5'))
                        )
                        .push(
                            Button::new(Text::new("6").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('6'))
                        )
                        .push(
                            Button::new(Text::new("*").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Operation })))
                                .on_press(Message::Sign('×'))
                        )
                        .push(
                            Button::new(Text::new("%").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Special })))
                                .on_press(Message::Special('%'))
                        )
                )
                .push(
                    Row::new()
                        .spacing(1)
                        .push(
                            Button::new(Text::new("MS").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Memory })))
                                .on_press(Message::Memory('S'))
                        )
                        .push(
                            Button::new(Text::new("1").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('1'))
                        )
                        .push(
                            Button::new(Text::new("2").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('2'))
                        )
                        .push(
                            Button::new(Text::new("3").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('3'))
                        )
                        .push(
                            Button::new(Text::new("-").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Operation })))
                                .on_press(Message::Sign('-'))
                        )
                        .push(
                            Button::new(Text::new("1/x").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Special })))
                                .on_press(Message::Special('i'))
                        )
                )
                .push(
                    Row::new()
                        .spacing(1)
                        .push(
                            Button::new(Text::new("M+").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Memory })))
                                .on_press(Message::Memory('+'))
                        )
                        .push(
                            Button::new(Text::new("0").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Num('0'))
                        )
                        .push(
                            Button::new(Text::new("+/-").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Neg)
                        )
                        .push(
                            Button::new(Text::new(".").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Number })))
                                .on_press(Message::Dot)
                        )
                        .push(
                            Button::new(Text::new("+").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Operation })))
                                .on_press(Message::Sign('+'))
                        )
                        .push(
                            Button::new(Text::new("=").size(12))
                                .width(Length::Fixed(35.0))
                                .style(theme::Button::Custom(Box::new(CustomButtonStyle { button_type: ButtonType::Equal })))
                                .on_press(Message::Ans)
                        )
                )
        )
        .style(theme::Container::Custom(Box::new(MainContainerStyle)))
        .into()
    }

}
pub struct MainContainerStyle;

impl container::StyleSheet for MainContainerStyle {
    type Style = Theme;
    


    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.95, 0.97, 1.0, 0.95))), // Fondo semi-transparente
            ..Default::default()
        }
    }
}
