use druid::widget::{Button, Checkbox, Flex, Label, Radio, Slider, Stepper, Switch, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    left: String,
    right: String,
    state: bool,
    enabled: bool,
    count: f64,
}

fn element(lens: impl Lens<AppData, String> + 'static, disabled: bool) -> Box<dyn Widget<AppData>> {
    if disabled {
        Box::new(TextBox::new().lens(lens).enable_if(|data, _| data.enabled))
    } else {
        Box::new(TextBox::new().lens(lens))
    }
}

fn row(even: bool) -> impl Widget<AppData> {
    Flex::row()
        .with_boxed_child(element(AppData::left, even))
        .with_default_spacer()
        .with_boxed_child(element(AppData::right, !even))
}

fn widget() -> impl Widget<AppData> {
    let counter = Flex::row()
        .with_child(
            Button::new("-")
                .on_click(|_, data: &mut f64, _| *data = (*data - 1.0).max(0.0))
                .enable_if(|data, _| *data > 0.0)
                .lens(AppData::count),
        )
        .with_default_spacer()
        .with_child(Label::dynamic(|data: &AppData, _| {
            ((data.count * 100.0).round() / 100.0).to_string()
        }))
        .with_default_spacer()
        .with_child(
            Button::new("+")
                .on_click(|_, data: &mut f64, _| *data = (*data + 1.0).min(5.0))
                .enable_if(|data, _| *data < 5.0)
                .lens(AppData::count),
        );

    Flex::column()
        .with_child(row(false))
        .with_default_spacer()
        .with_child(row(true))
        .with_default_spacer()
        .with_child(row(false))
        .with_default_spacer()
        .with_child(
            Switch::new()
                .lens(AppData::state)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(
            Radio::new("True", true)
                .lens(AppData::state)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(
            Radio::new("False", false)
                .lens(AppData::state)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(
            Stepper::new()
                .with_range(0.0, 5.0)
                .lens(AppData::count)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(
            Checkbox::new("Data")
                .lens(AppData::state)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(
            Slider::new()
                .with_range(0.0, 5.0)
                .lens(AppData::count)
                .enable_if(|data: &AppData, _| data.enabled),
        )
        .with_default_spacer()
        .with_child(Checkbox::new("Controls enabled").lens(AppData::enabled))
        .with_default_spacer()
        .with_default_spacer()
        .with_child(counter)
        .align_horizontal(UnitPoint::CENTER)
}

pub fn main() {
    let window = WindowDesc::new(widget()).title("Disable Test");

    AppLauncher::with_window(window)
        .use_env_tracing()
        .launch(AppData {
            left: String::new(),
            right: String::from("test"),
            state: false,
            enabled: true,
            count: 0.0,
        })
        .unwrap();
}
