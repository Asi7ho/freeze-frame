use iced::{
    alignment::Horizontal,
    theme,
    widget::{column, container, pick_list, row, slider, text, text_input},
    Element, Length,
};

use crate::{
    styles::{RightBarStyle, SliderStyle},
    tools::filters::{BrushFilter, GeometryForm},
    FreezeFrameMessage,
};

use super::{MainViewMessage, PropertyMessage};

#[derive(Debug, Default)]
pub struct PropertyState {
    pub filter: BrushFilter,
    pub dialog_text: String,
    pub action_text: String,
    pub note_text: String,
    pub brush_slider_value: f32,
    pub eraser_slider_value: f32,
    pub geometry_form: Option<GeometryForm>,
}

fn _message(message: PropertyMessage) -> FreezeFrameMessage {
    FreezeFrameMessage::MainView(MainViewMessage::Property(message))
}

pub fn view(state: &PropertyState) -> Element<FreezeFrameMessage> {
    let heading = text("Properties")
        .size(22)
        .width(Length::Fixed(225.0))
        .horizontal_alignment(Horizontal::Center);

    let empty_column = column![].into();

    let properties = match state.filter {
        BrushFilter::Pointer => scene_properties(state),
        BrushFilter::Brush => brush_properties(&state.brush_slider_value),
        BrushFilter::Eraser => brush_properties(&state.eraser_slider_value),
        BrushFilter::Geometry => geometry_properties(&state.geometry_form),
        _ => empty_column,
    };

    let content = column![heading, properties].spacing(10);

    Element::from(
        container(content)
            .height(Length::Fill)
            .width(Length::Fixed(225.0))
            .padding(10)
            .style(theme::Container::Custom(Box::new(RightBarStyle))),
    )
}

fn scene_properties(state: &PropertyState) -> Element<FreezeFrameMessage> {
    let dialog_text = text("Dialog");
    let dialog_input = text_input("", &state.dialog_text, |s| {
        _message(PropertyMessage::ChangeDialogInput(s))
    });
    let dialog_widget = column![dialog_text, dialog_input].padding(10);

    let action_text = text("Action");
    let action_input = text_input("", &state.action_text, |s| {
        _message(PropertyMessage::ChangeActionInput(s))
    });
    let action_widget = column![action_text, action_input].padding(10);

    let note_text = text("Note");
    let note_input = text_input("", &state.note_text, |s| {
        _message(PropertyMessage::ChangeNoteInput(s))
    });
    let note_widget = column![note_text, note_input].padding(10);

    Element::from(column![dialog_widget, action_widget, note_widget])
}

fn brush_properties(slider_value: &f32) -> Element<FreezeFrameMessage> {
    let size_text = text("Size: ");
    let size_slider = slider(1.0..=50.0, *slider_value, |v| {
        _message(PropertyMessage::Slide(v))
    })
    .style(theme::Slider::Custom(Box::new(SliderStyle)));
    let size_ind = text(slider_value.to_string());

    let content = row![size_text, size_slider, size_ind].spacing(15);

    Element::from(column![content].padding(10).width(Length::Fixed(225.0)))
}

fn geometry_properties(geometry_form: &Option<GeometryForm>) -> Element<FreezeFrameMessage> {
    let form_text = text("Form: ");
    let form_picklist = pick_list(&GeometryForm::ALL[..], *geometry_form, |f| {
        _message(PropertyMessage::ChangeGeometryForm(f))
    });

    let content = row![form_text, form_picklist].spacing(15);

    Element::from(column![content].padding(10).width(Length::Fixed(225.0)))
}
