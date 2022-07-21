use crate::widgets::{
    header::{BrushFilter, ExtraFilter},
    property::GeometryForm,
    tools::drawing::Strokes,
};

// Application
#[derive(Debug, Clone)]
pub enum FreezeFrameMessage {
    HeaderInteraction(HeaderMessage),
    CanvasInteraction(CanvasMessage),
    PropertyInteraction(PropertyMessage),
    Ignore,
}

//  Header
#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SceneTitleChange(String),
    BrushControlsChange(BrushFilter),
    GridToolSelected(ExtraFilter),
    ChangePalette,
    ChangeColor((usize, usize)),
    AddColor,
    Scrolled(f32),
}

// Canvas
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddStrokes(Strokes),
}

// Layers
#[derive(Debug, Clone)]
pub enum LayerMessage {
    LayerChanged(usize),
}

// Property
#[derive(Debug, Clone)]
pub enum PropertyMessage {
    SliderChanged(f32),
    ResolutionXChanged(String),
    ResolutionYChanged(String),
    GeometryFormChanged(GeometryForm),
}
