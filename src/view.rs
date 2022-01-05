use druid::widget::{prelude::*, Flex, SizedBox};
use druid::{Color, Widget, WidgetExt};

use crate::canvas::canvas::Canvas;
use crate::data::AppState;

struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(data);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

/// Build the canvas
fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    let canvas = Canvas::new(
        state.canvas_width,
        state.canvas_height,
        state.canvas_background_color.clone(),
    );

    let sized = SizedBox::new(canvas);
    sized.border(Color::BLACK, 5.0).center().boxed()
}

/// Build the ui
pub fn build_ui() -> impl Widget<AppState> {
    Flex::row().with_child(Rebuilder::new().center())
}
