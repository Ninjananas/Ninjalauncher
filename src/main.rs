// MIT License

#![windows_subsystem = "windows"]
mod mojang;
mod util;

use std::fs::{self, create_dir_all};
use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

#[macro_use]
extern crate lazy_static;

use druid::im::{vector, Vector};
use druid::widget::{Button, Flex, ViewSwitcher, Label, Switch, LensWrap, Checkbox, TextBox, RadioGroup, List, Scroll};
use druid::{commands, AppLauncher, Color, Widget, WidgetExt, WindowDesc, Data, Lens, PlatformError, LocalizedString, TextAlignment, Scale, EventCtx, FileDialogOptions, FileSpec, AppDelegate, DelegateCtx, Env, Command, Handled, Target};

use druid::theme::{BORDER_LIGHT};
use druid::lens::{self, LensExt};


#[derive(Clone, Data ,Lens)]
struct GameOptions {
    launcher_name: String,
    launcher_version: String,

    user_type: String,
    auth_player_name: String,
    auth_uuid: String,
    auth_access_token: String,

    version_type: String,
    version_name: String,

    game_directory: String,
    assets_index_name: String,
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions {
            launcher_name: String::from("minecraft-launcher"),
            launcher_version: String::from("2.1.1349"),

            user_type: String::from("legacy"),
            auth_player_name: String::from("Toto"),
            auth_uuid: String::from("00000000-0000-0000-0000-000000000000"),
            auth_access_token: String::from(""),

            version_type: String::from("release"),
            version_name: String::from("latest"),

            game_directory: String::from("todo"),
            assets_index_name: String::from("todo"),
        }
    }
}

#[derive(Clone, Data ,Lens, PartialEq, Eq)]
struct Version {
    name: String,
    installed: bool,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn get_installed_versions() -> Vector<Version> {
    let mut vector = Vector::new();
    for entry in fs::read_dir(util::VERSIONS_DIR.as_path()).unwrap() {
        let path = entry.unwrap().path();
        let version_name: String = String::from(
            path.file_name().unwrap()
                .to_str().unwrap()
        );

        let mut jar_path = PathBuf::new();
        jar_path.push(path.clone());
        let mut file = version_name.clone();
        file.push_str(".jar");
        jar_path.push(file);

        let mut json_path = PathBuf::new();
        json_path.push(path.clone());
        let mut file = version_name.clone();
        file.push_str(".json");
        json_path.push(file);

        if jar_path.exists() & json_path.exists() {
            vector.push_front(Version{
                name: version_name,
                installed: true,
            })
        }
    }
    vector.sort();
    vector
}



fn update_version_list_from_mojang(version_list: &mut Vector<Version>) {
    let manifest;
    match mojang::get_version_manifest() {
        Ok(value) => manifest = value,
        Err(_) => return,
    }
    for version in manifest.versions {
        if version.r#type != "release" {
            continue;
        }
        let mut already_installed = false;
        println!("{} {}", version.id, version.r#type);
        for installed_version in version_list.iter() {
            if installed_version.name == version.id {
                already_installed = true;
                break;
            }
        }
        if !already_installed {
            version_list.push_front(Version{
                name: version.id.clone(),
                installed:false,
            })

        }
    }
    version_list.sort();
}

#[derive(Clone, Data ,Lens)]
struct LaunchOptions {
    auth_player_name: String,
    auth_uuid: String,
    auth_password: String,
    online: bool,

    version: String,
    use_latest_version: bool,
    version_list: Vector<Version>,
    show_version_list: bool,

    game_directory: String,

}

impl Default for LaunchOptions {
    fn default() -> Self {
        LaunchOptions {
            auth_player_name: String::from("Toto"),
            auth_uuid: String::from("00000000-0000-0000-0000-000000000000"),
            auth_password: String::from(""),
            online: false,

            version: String::from("- - -"),
            use_latest_version: true,
            version_list: get_installed_versions(),
            show_version_list: true,

            game_directory: String::from(util::MINECRAFT_DIR.to_str().unwrap()),

        }
    }
}


fn build_root_widget() -> impl Widget<LaunchOptions> {
    let mut title_label = Label::new(LocalizedString::new("launcher-title")
                                     .with_placeholder("Minecraft Launcher"));
    title_label.set_text_size(30.0);

    let online = Checkbox::new(LocalizedString::new("online-toggle")
                               .with_placeholder("Online mode (connect to Mojang)"))
        .lens(LaunchOptions::online);

    let player_name = Flex::column()
        .with_child(
            Label::new(LocalizedString::new("player-name-label")
                       .with_placeholder("Player Name"))
        )
        .with_child(
            TextBox::new().lens(LaunchOptions::auth_player_name)
        );

    let player_password = Flex::column()
        .with_child(
            Label::new(LocalizedString::new("player-password-label")
                       .with_placeholder("Password"))
        )
        .with_child(
            TextBox::new().lens(LaunchOptions::auth_password)
        )
        .disabled_if(|data, _| !data.online);

    let player_name_and_password = Flex::row()
        .with_child(player_name)
        .with_default_spacer()
        .with_child(player_password);

    let uuid_options = Flex::row()
        .with_child(Label::new(LocalizedString::new("uuid")
                               .with_placeholder("UUID")))
        .with_child(TextBox::new()
                    .with_text_alignment(TextAlignment::Center)
                    .lens(LaunchOptions::auth_uuid)
                    .fix_width(320.0))
        .with_default_spacer()
        .with_child(Button::new("🗘")
                    .on_click(|_, data: &mut LaunchOptions, _| data.auth_uuid = util::generate_random_uuid()))
        .disabled_if(|data, _| data.online);


    let player_options = Flex::column()
        .with_child(online)
        .with_child(player_name_and_password)
        .with_spacer(10.0)
        .with_child(uuid_options);

    let use_latest = Checkbox::new(LocalizedString::new("use-latest-version")
                                   .with_placeholder("Use latest version (from Mojang)"))
        .lens(LaunchOptions::use_latest_version);

    let mut selected_version = Label::new(|data: &LaunchOptions, _: &_|
                                          if data.use_latest_version {
                                              String::from("<latest>")
                                          } else {format!("{}", data.version)});
    selected_version.set_text_size(30.0);

    let selected_version =Flex::column()
        .with_child(Label::new(LocalizedString::new("selected-version")
                               .with_placeholder("Selected version:")))
        .with_child(selected_version)
        .disabled_if(|data, _| data.use_latest_version);


    let game_options = Flex::column()
        .with_child(use_latest)
        .with_child(selected_version);


    let version_list = ViewSwitcher::new(
        |data: &LaunchOptions, _| data.show_version_list,
        |selector, _, _| match selector {
            true => Box::new(
                Scroll::new(
                    List::new(|| {
                        Label::new(|(data, item): &(LaunchOptions, Version), _env: &_| format!("{}{:7}{}", if item.name == data.version {"→"} else {" "}, item.name, if item.installed {"✓"} else {"⨯"}))
                            .on_click(
                                |_, (data, item): &mut (LaunchOptions, Version), _| {
                                    data.version = item.name.clone();
                                })
                    }))
                    .vertical()
                    .lens(lens::Identity.map(
                        |d: &LaunchOptions| (d.clone(), d.version_list.clone()),
                        |d: &mut LaunchOptions, x: (LaunchOptions, Vector<Version>)| {
                            *d = x.0
                        }
                    ))
                    .fix_width(80.0)
            ),
            false => Box::new(Label::new("Coucou"))//Flex::column().fix_width(0.0)),
        }
    ).disabled_if(|data, _| data.use_latest_version);

    let update_versions = Button::new(
        LocalizedString::new("update-versions")
            .with_placeholder("Update available versions from Mojang")
    ).on_click(|_, data: &mut LaunchOptions, _| update_version_list_from_mojang(&mut data.version_list)
    ).disabled_if(|data, _| data.use_latest_version);

    let standard_options = Flex::row()
        .with_child(Flex::column()
                    .with_flex_child(player_options, 1.0)
                    .with_spacer(10.0)
                    .with_flex_child(game_options, 1.0))
        .with_child(Flex::column()
                    .with_flex_child(version_list, 1.0)
                    .with_spacer(10.0)
                    .with_flex_child(update_versions, 1.0));

    //let advanced_options = Flex::column();

    let json = FileSpec::new("JSON file", &["json"]);
    let save_profile_options = FileDialogOptions::new()
        .allowed_types(vec![json])
        .default_type(json)
        .default_name(String::from("ninjalauncher_profile.json"))
        .name_label("Store profile")
        .title("Choose where to store your profile")
        .button_text("Save");

    let save_profile_button = Button::new("Save").on_click(move |ctx, _: &mut LaunchOptions, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_profile_options.clone()))
    });

    let main_col = Flex::column()
        .with_child(title_label
                    .fix_height(40.0)
                    .center())
        .with_spacer(20.0)
        .with_flex_child(standard_options, 1.2)
        //.with_spacer(10.0)
        //.with_flex_child(advanced_options, 1.0)
        .with_spacer(3.0)
        .with_child(save_profile_button);


    //let switch = LensWrap::new(Checkbox::new("bool value"), LaunchOptions::bool_value);
    //col.add_child(switch);
    main_col
}

struct Delegate;

fn main() -> Result<(), Error> {
    //let info = mojang::get_1_16_info().unwrap();
    //let info = mojang::get_version_manifest().unwrap();
    //print!("{:?}", info);

    //let mcpath = &util::MINECRAFT_DIR;
    //print!("{:?}", mcpath.display());

    create_dir_all(&*util::VERSIONS_DIR)?;

    let window = WindowDesc::new(build_root_widget())
        .title(
            LocalizedString::new("launcher-title")
                .with_placeholder("NinjaLauncher")
        ).with_min_size((800.0, 500.0));

    AppLauncher::with_window(window)
        .delegate(Delegate)
        .log_to_console()
        .launch(LaunchOptions::default())
        .expect("Ninjalauncher startup failed!");

    Ok(())
}


impl AppDelegate<LaunchOptions> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut LaunchOptions,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.auth_uuid) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    let first_line = s.lines().next().unwrap_or("");
                    data.auth_uuid = first_line.to_owned();
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}
