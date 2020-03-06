use crossterm::style::{style, Attribute, Color, StyledContent};

pub fn stylize(
    text: &str,
    attribute: Attribute,
    color: Color,
    augment: bool,
) -> StyledContent<&str> {
    if augment {
        style(text).with(color).attribute(attribute)
    } else {
        style(text)
            .with(Color::White)
            .attribute(Attribute::NormalIntensity)
    }
}
