use druid::{
    theme::*,
    widget::{Button, Container, Either, Flex, Label, LensWrap, List, Scroll, TextBox},
    Color, Widget, WidgetExt,
};

use crate::gui::app_state::AppState;

pub fn root_widget() -> impl Widget<AppState> {
    let search_box = TextBox::new()
        .with_text_size(TEXT_SIZE_LARGE)
        .with_placeholder("Package name")
        .lens(AppState::search_term);

    let search_bar = Container::new(
        Container::new(search_box)
            .rounded(2.0)
            .expand_width()
            .padding(10.0),
    )
    .expand_width();

    let clean_error_button = Either::new(
        |data: &AppState, _| data.error_message.is_some(),
        Button::new("Clean errors").on_click(|_, data: &mut AppState, _| data.error_message = None),
        Flex::column(),
    );

    let error_box = Label::dynamic(|data: &AppState, _| {
        match &data.error_message {
            Some(some) => some,
            None => "Error while loading an error message!",
        }
        .to_string()
    })
    .with_text_size(TEXT_SIZE_LARGE)
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    .with_text_color(Color::RED)
    .scroll()
    .vertical();

    let list = Scroll::new(LensWrap::new(
        List::new(|| super::package_widget::package_widget()),
        AppState::package_list,
    ))
    .vertical();

    // this is either the list of packages or an error
    let message_box = Either::new(
        |data: &AppState, _| data.error_message.is_some(),
        error_box,
        list,
    );

    let no_packages_found_text = Either::new(
        |data: &AppState, _| data.no_packages_found,
        Label::dynamic(|data: &AppState, _| {
            format!("No packages found containing '{}'", data.last_search_term)
        })
        .with_text_size(TEXT_SIZE_NORMAL)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
        Flex::column(),
    );

    let search_buttons = Flex::row()
        .with_child(
            Button::dynamic(|data: &AppState, _| {
                if data.is_searching {
                    "Searching..."
                } else {
                    "Search Package"
                }
                .into()
            })
            .on_click(|ctx, data: &mut AppState, _| {
                super::controller::find_packages_async(data, ctx, false)
            })
            .disabled_if(|data: &AppState, _| data.is_searching),
        )
        .with_spacer(5.0)
        .with_child(Either::new(
            |data: &AppState, _| !data.is_searching,
            Button::new("Deep Search Package").on_click(|ctx, data: &mut AppState, _| {
                super::controller::find_packages_async(data, ctx, true)
            }),
            Flex::column(),
        ));

    Flex::column()
        .with_child(search_bar)
        .with_child(search_buttons)
        .with_child(no_packages_found_text)
        .with_child(clean_error_button)
        .with_flex_child(message_box, 1.0)
        .controller(super::controller::AppController)
}
