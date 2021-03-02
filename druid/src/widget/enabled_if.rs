use crate::{WidgetPod, Env, Widget, Data, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, UpdateCtx, Point, Size};

pub struct EnabledIf<T, W> {
    inner: WidgetPod<T, W>,
    enabled_if: Box<dyn Fn(&T, &Env) -> bool>,
}

impl<T: Data, W: Widget<T>> EnabledIf<T, W> {
    pub fn new(inner: W, enabled_if: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        EnabledIf {
            inner: WidgetPod::new(inner),
            enabled_if: Box::new(enabled_if),
        }
    }
    pub fn boxed(inner: W, enabled_if: Box<dyn Fn(&T, &Env) -> bool>) -> Self {
        EnabledIf {
            inner: WidgetPod::new(inner),
            enabled_if,
        }
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for EnabledIf<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded {..} = event {
            let enabled = (self.enabled_if)(data, env);

            if !enabled {
                ctx.set_disabled_initially();
                self.inner.lifecycle(ctx, &LifeCycle::WidgetAdded {initially_enabled: false}, data, env);
                return;
            }
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let enabled = (self.enabled_if)(data, env);
        ctx.set_enabled(enabled);
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}