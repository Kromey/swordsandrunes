use bevy::prelude::*;
use itertools::Itertools;

#[derive(Debug, Default, Clone, Eq, PartialEq, Resource)]
pub struct Messages {
    messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<S: Into<String>>(&mut self, message: S) {
        self.messages.push(Message::new(message));
    }

    pub fn add_hostile<S: Into<String>>(&mut self, message: S) {
        self.messages.push(Message::hostile(message));
    }

    pub fn add_friendly<S: Into<String>>(&mut self, message: S) {
        self.messages.push(Message::friendly(message));
    }

    pub fn add_notice<S: Into<String>>(&mut self, message: S) {
        self.messages.push(Message::notice(message));
    }

    pub fn text_sections_rev(&self, font: Handle<Font>) -> impl Iterator<Item = TextSection> + '_ {
        self.messages
            .iter()
            .rev()
            .dedup_with_count()
            .map(move |(count, message)| {
                let mut msg = message.message.clone();
                if count > 1 {
                    msg.push_str(&format!(" (x{count})"));
                }
                msg.push('\n');
                TextSection::new(
                    msg,
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: message.level.color(),
                    },
                )
            })
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Message {
    message: String,
    level: MessageLevel,
}

impl Message {
    fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            level: MessageLevel::Default,
        }
    }

    fn hostile<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            level: MessageLevel::Hostile,
        }
    }

    fn friendly<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            level: MessageLevel::Friendly,
        }
    }

    fn notice<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            level: MessageLevel::Notice,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
enum MessageLevel {
    #[default]
    Default,
    Hostile,
    Friendly,
    Notice,
}

impl MessageLevel {
    fn color(&self) -> Color {
        match *self {
            MessageLevel::Default => Color::WHITE,
            MessageLevel::Hostile => Color::RED,
            MessageLevel::Friendly => Color::GREEN,
            MessageLevel::Notice => Color::ORANGE,
        }
    }
}
