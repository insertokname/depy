use depy::{package::Package, parse_json};
use druid::{im::Vector, AppLauncher, LocalizedString, WindowDesc};

mod gui;

const WINDOW_TITLE: LocalizedString<gui::app_state::AppState> = LocalizedString::new("Depy");

fn main() {
    let main_window = WindowDesc::new(gui::elements::root_widget::root_widget())
        .title(WINDOW_TITLE)
        .window_size((600.0, 500.0));

    let mut initial_state = gui::app_state::AppState::default();

    initial_state.installed_packages = Vector::from(
        Package::multiple_packages_from_json(&parse_json::read_json_file("./depy.json").unwrap())
            .unwrap(),
    );

    AppLauncher::with_window(main_window)
        .configure_env(gui::theme::setup_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
