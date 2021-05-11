use iced::{button, Button, Element, Row, Text};

use crate::theme::{self, REGULAR};

pub fn my_button<'a, T: 'a + Clone>(
    text: &str,
    state: &'a mut button::State,
    message: T,
) -> Element<'a, T> {
    let cancel_button_content = Row::new()
        .spacing(10)
        .padding(10)
        .push(Text::new("X"))
        .push(Text::new(text).font(REGULAR).size(18));

    let cancel_button = Button::new(state, cancel_button_content)
        .style(theme::Button)
        .on_press(message);

    cancel_button.into()
}
