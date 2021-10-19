// MIT License

#![windows_subsystem = "windows"]
mod mojang;

use druid::widget::{Svg, SvgData, Button, Flex, ViewSwitcher, Label};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc, Data, Lens, PlatformError, LocalizedString};

static DEFAULT_USERNAME: &str = "Toto";
static DEFAULT_UUID: &str = "01234567890123456";
static DEFAULT_TOKEN: &str = "0";

#[derive(Clone, Data ,Lens)]
struct AppData {
    // Launch view (0) / profile edit view (1x)
    view: usize,

    username: String,
    token: String,
    uuid: String,
}
/*
fn svg_widget(svg: Svg) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        
        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));
        
        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(format!("{}", digit))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| data.digit(digit))
}
*/


fn build_ui() -> impl Widget<AppData> {
    let mut all = Flex::column();

    let mut top_banner = Flex::row();

    let status = Box::new(Label::new("ready"));

    let svg_profile =  Svg::new(
            match include_str!("../resources/img/profile.svg")
                .parse::<SvgData>() {
                    Ok(svg) => svg,
                    Err(err) => {
                        println!("ERROR loading svg:\n{}", err);
                        SvgData::default()
                    }
                })
            .expand_height()
            .center(); 

    let mut profile_button = Flex::column();
    profile_button.add_flex_spacer(1.0);
    profile_button.add_flex_child(
        svg_profile,
        6.0);
    profile_button.add_flex_spacer(1.0);

    

    top_banner.add_flex_child(
        profile_button
            .expand_height()
            .expand_width()
            .background(Color::rgb8(0, 0, 0x99))
            ,
        1.0
    );

    top_banner.add_flex_spacer(5.0);

    top_banner.add_flex_child(
        Svg::new(
            match include_str!("../resources/img/settings.svg")
                .parse::<SvgData>() {
                    Ok(svg) => svg,
                    Err(err) => {
                        println!("ERROR loading svg:\n{}", err);
                        SvgData::default()
                    }
                })
            //.fix_width(50.)
            .lens(AppData::view)
            .expand_height()
            .background(Color::rgb8(0, 0, 0x99))
            //.expand_width()
            .center(),
        1.0
    );

    let view_switcher = ViewSwitcher::new(
        |data: &AppData, _env| data.view,
        |selector, _data, _env| match selector {
            1 => Box::new(Label::new("LAUNCH")),
            2 => Box::new(Label::new("PROFILE_EDIT")),
            _ => Box::new(Label::new(
                LocalizedString::new("ninjalauncher-fatal-error").
                    with_placeholder("An error occurred, please close launcher")
            )),
        }
    );

    all.add_flex_child(
        top_banner
            .expand_height()
            .expand_width()
            .background(Color::rgb8(0, 0x88, 0x55)),
        2.0
    );

    all.add_flex_child(
        view_switcher
            .expand_height()
            .expand_width()
            .background(Color::rgb8(0x99, 0x22, 0x22)),
        12.0
    );

    all.add_child(
        status
            .fix_height(20.0)
            .expand_width()
            .background(Color::rgb8(0x00, 0x00, 0x00))
    );
    
    all.debug_paint_layout()
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui)
        .window_size((1000., 600.))
        .title(LocalizedString::new("ninjalauncher-window-title")
               .with_placeholder("Ninjalauncher for Minecraft"))
        .resizable(false);

    let data = AppData {
        view: 1,
        username: DEFAULT_USERNAME.to_string(),
        token: DEFAULT_TOKEN.to_string(),
        uuid: DEFAULT_UUID.to_string(),
    };

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}

//fn set_theme(env: Env) -> Env {
//    
//}
