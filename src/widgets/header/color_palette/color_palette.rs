use iced::{button, svg, Button, Color, Container, Element, Length, Row, Svg, Text};

use crate::utils::svg::{ADD, BOTTOM_ARROW, ICON_SIZE};

use crate::widgets::header::{HeaderButtonStyle, HeaderColorButtonStyle, HeaderMessage};
use crate::{FreezeFrameMessage, InteractionMessage};

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub colors: Vec<Color>,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            colors: vec![Color::BLACK],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PaletteControls {
    pub choose_palette_state: button::State,
    pub brush_color_states: Vec<button::State>,
    pub add_color_state: button::State,
}

impl PaletteControls {
    pub fn view(
        &mut self,
        current_palette: Vec<Color>,
        current_color_id: usize,
    ) -> Element<FreezeFrameMessage> {
        let PaletteControls {
            choose_palette_state,
            brush_color_states,
            add_color_state,
        } = self;

        let filter_button = |state, icon_byte, message| {
            let handle = svg::Handle::from_memory(icon_byte);
            let icon = Svg::new(handle)
                .height(Length::Units(ICON_SIZE))
                .width(Length::Units(ICON_SIZE));
            let button = Button::new(state, icon)
                .style(HeaderButtonStyle::HeaderButtonDefaultStyle)
                .padding(10);

            button.on_press(message)
        };

        brush_color_states.resize_with(current_palette.len(), Default::default);

        let colors = Row::with_children(
            current_palette
                .iter()
                .zip(brush_color_states)
                .enumerate()
                .map(|(i, (color, state))| {
                    Button::new(state, Text::new(""))
                        .height(Length::Units(34))
                        .width(Length::Units(34))
                        .on_press(FreezeFrameMessage::Interaction(
                            InteractionMessage::HeaderInteraction(HeaderMessage::ChangeColor(i)),
                        ))
                        .style(HeaderColorButtonStyle {
                            color: *color,
                            selected: i == current_color_id,
                        })
                        .padding(10)
                        .into()
                })
                .collect(),
        )
        .spacing(8);

        let color_tools = Container::new(
            Row::new()
                .spacing(5)
                .push(filter_button(
                    choose_palette_state,
                    BOTTOM_ARROW,
                    FreezeFrameMessage::Interaction(InteractionMessage::HeaderInteraction(
                        HeaderMessage::ChangePalette,
                    )),
                ))
                .push(colors)
                .push(filter_button(
                    add_color_state,
                    ADD,
                    FreezeFrameMessage::Interaction(InteractionMessage::HeaderInteraction(
                        HeaderMessage::AddColor(Color::WHITE),
                    )),
                )),
        );

        return color_tools.into();
    }
}
