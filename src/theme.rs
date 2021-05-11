use iced::{button, container, Color, Font, Vector};

const BG_COLOR: Color = Color::from_rgb(45. / 255., 45. / 255., 45. / 255.);
const PINK: Color = Color::from_rgb(1., 81. / 255., 248. / 255.);

pub const REGULAR: Font = Font::External {
    name: "Inter-Regular",
    bytes: include_bytes!("../fonts/Inter-Regular.ttf"),
};

pub const BOLD: Font = Font::External {
    name: "Inter-Bold",
    bytes: include_bytes!("../fonts/Inter-Bold.ttf"),
};

pub const _LIGHT: Font = Font::External {
    name: "Inter-Light",
    bytes: include_bytes!("../fonts/Inter-Light.ttf"),
};

#[allow(dead_code)]
pub enum Container {
    Basic,
    Debug,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Container::Basic => container::Style {
                background: BG_COLOR.into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            },
            Container::Debug => container::Style {
                background: PINK.into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            },
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::WHITE.into(),
            text_color: Color::BLACK,
            border_radius: 5.,
            border_color: Color::BLACK,
            border_width: 2.,
            shadow_offset: Vector::new(2., 2.),
            ..button::Style::default()
        }
    }
}
