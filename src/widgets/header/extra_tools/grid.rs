use iced::{button, svg, Alignment, Button, Container, Element, Length, Row, Svg};

use crate::{
    widgets::header::{HeaderButtonStyle, HeaderMessage},
    FreezeFrameMessage, InteractionMessage,
};

use crate::utils::svg::{GRID, ICON_SIZE};

use super::ExtraFilter;

#[derive(Debug, Default, Clone)]
pub struct GridControls {
    pub grid_state: button::State,
}

impl GridControls {
    pub fn view(&mut self, selected: ExtraFilter) -> Element<FreezeFrameMessage> {
        let GridControls { grid_state } = self;

        let filter_button = |state, icon_byte, filter, selected| {
            let handle = svg::Handle::from_memory(icon_byte);
            let icon = Svg::new(handle)
                .height(Length::Units(ICON_SIZE))
                .width(Length::Units(ICON_SIZE));
            let button = Button::new(state, icon)
                .style(if filter == selected {
                    HeaderButtonStyle::HeaderButtonSelectedStyle
                } else {
                    HeaderButtonStyle::HeaderButtonDefaultStyle
                })
                .padding(10);

            button.on_press(FreezeFrameMessage::Interaction(
                InteractionMessage::HeaderInteraction(HeaderMessage::ExtraToolSelected(filter)),
            ))
        };

        let grid = Container::new(
            Row::new()
                .spacing(5)
                .push(filter_button(grid_state, GRID, ExtraFilter::Grid, selected))
                .align_items(Alignment::Center),
        );

        return grid.into();
    }
}
