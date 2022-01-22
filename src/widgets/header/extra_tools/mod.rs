use iced::Element;

use crate::FreezeFrameMessage;

pub mod grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraFilter {
    Grid,
    Ignore,
}

impl Default for ExtraFilter {
    fn default() -> Self {
        ExtraFilter::Ignore
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExtraControl {
    grid_control: grid::GridControls,
}

impl ExtraControl {
    pub fn view(&mut self, selected_tool: ExtraFilter) -> Element<FreezeFrameMessage> {
        let ExtraControl { grid_control } = self;
        grid_control.view(selected_tool)
    }
}
