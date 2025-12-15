use cif::App;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn main() -> iced::Result {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG) // TODO: Control this via environment variables
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    iced::run(App::update, App::view)
}
