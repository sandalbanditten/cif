use iced::Element;
use iced::widget::{button, column, row, text};
use tracing::{Level, event};
use tracing_subscriber::FmtSubscriber;

pub fn main() -> iced::Result {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG) // TODO: Control this via environment variables
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    iced::run(update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Decrement,
    Increment,
}

#[derive(Default, Debug)]
struct Counter {
    value: i64,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Decrement => counter.value -= 1,
        Message::Increment => counter.value += 1,
    }

    event!(Level::DEBUG, "{message:?}");
}

fn view(counter: &Counter) -> Element<'_, Message> {
    column![
        text(counter.value).size(20),
        row![
            button("Decrement").on_press(Message::Decrement),
            button("Increment").on_press(Message::Increment),
        ]
        .spacing(10)
    ]
    .spacing(10)
    .into()
}
