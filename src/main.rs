// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::kurbo::Circle;

use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, Point, WindowDesc};

struct CustomWidget {
    drawing: Vec<Point>,
    draw: bool,
}

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<String> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut String, _env: &Env) {
        match event {
            Event::MouseDown(cursor) => {
                self.drawing.push(cursor.window_pos);
                self.draw = true;
                ctx.request_paint();
            }
            Event::MouseUp(_) => self.draw = false,
            Event::MouseMove(cursor) => {
                if self.draw {
                    self.drawing.push(cursor.window_pos);
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
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
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
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        // Create an arbitrary bezier path
        for point in &self.drawing {
            let path = Circle::new(*point, 1.0);

            // Create a color
            let stroke_color = Color::rgb8(0, 0, 0);
            ctx.stroke(path, &stroke_color, 2.0);
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(|| CustomWidget {
        drawing: Vec::new(),
        draw: false,
    })
    .title(LocalizedString::new("Freeze Frame"))
    .window_size((800.0, 600.0));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("".to_string())
        .expect("launch failed");
}
