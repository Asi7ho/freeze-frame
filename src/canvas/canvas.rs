use crate::{data::AppState, tool::brush};
use druid::{widget::prelude::*, Color, Point, Rect};

pub struct Canvas {
    width: f64,
    height: f64,
    background_color: Color,
    is_drawing: bool,
    start_drawing: bool,
    brush: brush::freehand::Freehand,
    point: Option<Point>,
}

impl Canvas {
    pub fn new(width: f64, height: f64, color: Color) -> Self {
        return Self {
            width,
            height,
            background_color: color,
            is_drawing: false,
            start_drawing: false,
            brush: brush::freehand::Freehand::new(1.0, Color::BLACK),
            point: None,
        };
    }
}

impl Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(cursor) => {
                self.point = Some(cursor.window_pos);
                self.is_drawing = true;
                self.start_drawing = true;
                ctx.request_paint();
            }
            Event::MouseUp(_) => self.is_drawing = false,
            Event::MouseMove(cursor) => {
                if self.is_drawing {
                    self.point = Some(cursor.window_pos);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

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
            let size = Size::new(self.width, self.height);
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
        ctx.fill(rect, &Color::grey(0.4));

        let center_x = size.width / 2.0;
        let center_y = size.height / 2.0;
        let rect = Rect::from_center_size((center_x, center_y), (self.width, self.height));
        ctx.fill(rect, &self.background_color);

        if self.is_drawing {
            let point = self.point.unwrap();
            if self.start_drawing {
                self.brush.start_draw(point);
                self.start_drawing = false;
            } else {
                self.brush.draw(point);
            }
            let path = &self.brush.path;
            ctx.stroke(path, &self.brush.color, self.brush.size);
        }
    }
}
