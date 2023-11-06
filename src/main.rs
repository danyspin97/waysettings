use std::sync::OnceLock;

use iced::{
    executor,
    widget::{button, column, text},
    Application, Command, Element, Theme,
};

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

struct Waysettings {
    // The counter value
}

macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!(LANGUAGE_LOADER.get().unwrap(), $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!(LANGUAGE_LOADER.get().unwrap(), $message_id, $($args), *)
    }};
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    none,
}

impl Application for Waysettings {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Waysettings, iced::Command<Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        fl!("app")
    }

    fn view(&self) -> Element<Message> {
        column![
            text(fl!("welcome-page-button")),
            button("Hello").on_press(Self::Message::none)
        ]
        .into()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn theme(&self) -> self::Theme {
        iced::Theme::Dark
    }
}

fn main() -> iced::Result {
    LANGUAGE_LOADER.get_or_init(|| {
        let loader = fluent_language_loader!();
        loader
            .load_languages(&Localizations, &[loader.fallback_language()])
            .unwrap();
        loader
    });

    Waysettings::run(iced::Settings::default())
}
