use druid::{Widget, Data, Lens, AppLauncher, WindowDesc, WidgetExt};
use druid::widget::{Flex, TextBox, Button, Label, Checkbox};
use piet_common::UnitPoint;

#[derive(Clone, Data, Lens)]
struct AppData {
    left: String,
    right: String,
    text_enabled: bool,
    count: u64,
}

fn element(lens: impl Lens<AppData, String> + 'static, disabled: bool) -> Box<dyn Widget<AppData>> {
    if disabled {
        Box::new(
            TextBox::new()
                .lens(lens)
                .enable_if(|data, _|data.text_enabled)
        )
    } else {
        Box::new(
            TextBox::new()
                .lens(lens)
        )
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
                .on_click(|_, data: &mut u64, _|*data -= 1)
                .enable_if(|data, _|*data > 0)
                .lens(AppData::count)
        )
        .with_default_spacer()
        .with_child(
            Label::dynamic(|data: &AppData, _|data.count.to_string())
        )
        .with_default_spacer()
        .with_child(
            Button::new("+")
                .on_click(|_, data: &mut u64, _|*data += 1)
                .enable_if(|data, _|*data < 5)
                .lens(AppData::count)
        );

    Flex::column()
        .with_child(row(false))
        .with_default_spacer()
        .with_child(row(true))
        .with_default_spacer()
        .with_child(row(false))
        .with_default_spacer()
        .with_child(
            Checkbox::new("Text-boxes enabled")
                .lens(AppData::text_enabled)
        )
        .with_default_spacer()
        .with_child(counter)
        .align_horizontal(UnitPoint::CENTER)
}

fn main() {
    let window = WindowDesc::new(widget())
        .title("Disable Test");

    AppLauncher::with_window(window)
        .use_env_tracing()
        .launch(AppData{
            left: String::new(),
            right: String::from("test"),
            text_enabled: true,
            count: 0,
        }).unwrap();
}