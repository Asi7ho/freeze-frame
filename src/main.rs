use iced::{
    executor,
    pure::{
        widget::{Column, Container, Row},
        Application, Element,
    },
    window, Color, Command, Length, Settings,
};

use widgets::{
    canvas::{CanvasMessage, CanvasState, Strokes},
    header::{BrushFilter, ExtraFilter, HeaderMessage, HeaderState},
    layers::{LayerMessage, LayerState},
    property::{PropertyMessage, PropertyState},
    timeline::TimelineState,
};

use rand::{distributions::Uniform, prelude::Distribution};

mod utils;
mod widgets;

// Launch desktop app
fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            min_size: Some((1320, 700)),
            ..window::Settings::default()
        },
        // antialiasing: true,
        ..Settings::default()
    };

    FreezeFrame::run(settings)
}

#[derive(Default)]
pub struct FreezeFrame {
    header_state: HeaderState,
    canvas_state: CanvasState,
    strokes: Vec<Strokes>,
    timeline_state: TimelineState,
    property_state: PropertyState,
    layer_state: LayerState,
}

#[derive(Debug, Clone)]
pub enum FreezeFrameMessage {
    HeaderInteraction(HeaderMessage),
    CanvasInteraction(CanvasMessage),
    PropertyInteraction(PropertyMessage),
    Ignore,
}

impl Application for FreezeFrame {
    type Executor = executor::Default;
    type Message = FreezeFrameMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<FreezeFrameMessage>) {
        let header_state = HeaderState {
            scene_title_input: String::from("Scene title"),
            ..HeaderState::default()
        };
        let canvas_state = CanvasState {
            canvas_width: 750.0,
            canvas_height: 435.0,
            brush_color: header_state.color_palette.colors[0],
            brush_size: 1.0,
            ..CanvasState::default()
        };
        let property_state = PropertyState {
            brush_slider_value: 1.0,
            eraser_slider_value: 1.0,
            resolution: (canvas_state.canvas_width, canvas_state.canvas_height),
            ..PropertyState::default()
        };
        (
            Self {
                header_state,
                canvas_state,
                property_state,
                ..FreezeFrame::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Freeze Frame")
    }

    fn background_color(&self) -> Color {
        return Color::from_rgb8(34, 34, 34);
    }

    fn update(&mut self, message: FreezeFrameMessage) -> Command<FreezeFrameMessage> {
        match message {
            FreezeFrameMessage::HeaderInteraction(m) => match m {
                widgets::header::HeaderMessage::SceneTitleChange(scene_title) => {
                    self.header_state.scene_title_input = scene_title;
                }
                widgets::header::HeaderMessage::BrushControlsChange(filter) => {
                    self.header_state.brush_filter = filter;
                    self.canvas_state.brush_filter = filter;
                    self.property_state.filter = filter;
                }
                widgets::header::HeaderMessage::GridToolSelected(tool) => {
                    if tool == ExtraFilter::Trash {
                        self.canvas_state = CanvasState {
                            canvas_width: self.canvas_state.canvas_width,
                            canvas_height: self.canvas_state.canvas_height,
                            brush_color: self.header_state.color_palette.colors[0],
                            brush_filter: self.header_state.brush_filter,
                            brush_size: 1.0,
                            ..CanvasState::default()
                        };
                        self.strokes.clear();
                        self.header_state.extra_filter = ExtraFilter::Ignore;
                    } else {
                        if tool == self.header_state.extra_filter {
                            self.header_state.extra_filter = ExtraFilter::Ignore;
                        } else {
                            self.header_state.extra_filter = tool;
                        }
                    }
                }
                widgets::header::HeaderMessage::ChangePalette => (),
                widgets::header::HeaderMessage::ChangeColor(id_row_col) => {
                    self.header_state.brush_color_id = id_row_col;
                    self.canvas_state.brush_color =
                        self.header_state.color_palette.colors[id_row_col.0 * 5 + id_row_col.1];
                }
                widgets::header::HeaderMessage::AddColor => {
                    let step = Uniform::new(0, 256);
                    let mut rng = rand::thread_rng();
                    let red = step.sample(&mut rng) as u8;
                    let green = step.sample(&mut rng) as u8;
                    let blue = step.sample(&mut rng) as u8;

                    let color = Color::from_rgb8(red, green, blue);
                    self.header_state.color_palette.colors.push(color);
                }
                widgets::header::HeaderMessage::Scrolled(offset) => {
                    self.header_state.color_scroll_offset = offset;
                }
            },
            FreezeFrameMessage::CanvasInteraction(m) => match m {
                CanvasMessage::AddStrokes(stroke) => {
                    self.strokes.push(stroke);
                    self.canvas_state.request_redraw();
                }
            },
            FreezeFrameMessage::PropertyInteraction(m) => match m {
                PropertyMessage::SliderChanged(value) => {
                    if self.property_state.filter == BrushFilter::Brush {
                        self.property_state.brush_slider_value = value;
                    } else if self.property_state.filter == BrushFilter::Eraser {
                        self.property_state.eraser_slider_value = value;
                    }
                    self.canvas_state.brush_size = value;
                }
                PropertyMessage::ResolutionXChanged(x) => {
                    let resolution_x = x.parse::<f32>();
                    if resolution_x.is_ok() {
                        self.canvas_state.canvas_width = resolution_x.unwrap();
                        self.property_state.resolution.0 = self.canvas_state.canvas_width;
                    }
                }
                PropertyMessage::ResolutionYChanged(y) => {
                    let resolution_y = y.parse::<f32>();
                    if resolution_y.is_ok() {
                        self.canvas_state.canvas_height = resolution_y.unwrap();
                        self.property_state.resolution.1 = self.canvas_state.canvas_height;
                    }
                }
            },
            FreezeFrameMessage::Ignore => (),
        };

        Command::none()
    }

    fn view(&self) -> Element<FreezeFrameMessage> {
        let header_view = widgets::header::view(&self.header_state);
        let canvas_view = self
            .canvas_state
            .view(&self.strokes)
            .map(|stroke| FreezeFrameMessage::CanvasInteraction(CanvasMessage::AddStrokes(stroke)));
        let timeline_view = widgets::timeline::view(&self.timeline_state);
        let property_view = widgets::property::view(&self.property_state);
        let layer_view = widgets::layers::view(&self.layer_state);

        let main_view = Column::new()
            .push(header_view)
            .push(canvas_view)
            .push(timeline_view)
            .height(Length::Fill)
            .width(Length::Fill);
        let right_bar_view = Column::new().push(property_view).push(layer_view);

        return Container::new(Row::new().push(main_view).push(right_bar_view)).into();
    }
}
