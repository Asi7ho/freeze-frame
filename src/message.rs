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
    ChangeSceneTitle(String),
    ChangeBrushControls(BrushFilter),
    SelectGridTool(ExtraFilter),
    ChangePalette,
    ChangeColor((usize, usize)),
    AddColor,
    Scroll(f32),
}

// Canvas
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddStrokes(Strokes),
}

// Layers
#[derive(Debug, Clone)]
pub enum LayerMessage {
    ChangeLayer(usize),
}

// Property
#[derive(Debug, Clone)]
pub enum PropertyMessage {
    Slide(f32),
    ChangeResolutionX(String),
    ChangeResolutionY(String),
    ChangeGeometryForm(GeometryForm),
}
