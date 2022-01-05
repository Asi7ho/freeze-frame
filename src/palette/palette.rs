use crate::data::AppState;
use druid::widget::prelude::*;
use druid::{Color, Rect, Widget};

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new() -> Self {
        let default_colors = vec![Color::BLACK];
        return Self {
            colors: default_colors,
        };
    }
}

impl Widget<AppState> for Palette {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(250.0, 800.0);
            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::grey(0.9));

        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        let color_width = 50.0;
        let color_height = 25.0;
        for color in &self.colors {
            let rect = Rect::from_origin_size(
                (offset_x + 5.0, offset_y + 5.0),
                (color_width, color_height),
            );
            ctx.fill(rect, color);
            offset_x += color_width;
            offset_y += color_height
        }
    }
}
