use std::borrow::Cow;
use std::path::PathBuf;

use anyhow::{Result, Error, anyhow};
use iced::theme::Container;
use iced::{Settings, Application, executor, Command, Length, Element, Theme, color, Color, Background, Font};
use iced::alignment::{Alignment, Horizontal};
use iced::widget::{pick_list, column, text, checkbox, row, horizontal_rule, text_input, button, progress_bar, vertical_space, container, horizontal_space};
use iced::window::Icon;
use image::ImageFormat;
use native_dialog::FileDialog;

use crate::installer::{MinecraftVersion, fetch_minecraft_versions, LoaderVersion, fetch_loader_versions, install_client, ClientInstallation};
use crate::{Args, FONT_MEDIUM, ICON, FONT_REGULAR, theme, FONT_SEMIBOLD};

const POPPINS_REGULAR_FONT: Font = Font::External { name: "Poppins Regular", bytes: FONT_REGULAR };
const POPPINS_SEMIBOLD_FONT: Font = Font::External { name: "Poppins Semi-Bold", bytes: FONT_SEMIBOLD };

pub fn run(args: Args) -> iced::Result {
    let mut settings = Settings::default();
    settings.flags = args;
    settings.default_font = Some(FONT_MEDIUM);
    settings.window.size = (400, 500);
    settings.window.icon = Some(Icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap());

    Installer::run(settings)?;

    Ok(())
}

#[derive(Debug)]
struct Installer {
    minecraft_versions: Vec<MinecraftVersion>,
    selected_minecraft_version: Option<MinecraftVersion>,
    show_minecraft_snapshots: bool,
    
    loader_versions: Vec<LoaderVersion>,
    selected_loader_version: Option<LoaderVersion>,
    show_loader_betas: bool,

    directory: PathBuf,
    create_profile: bool,

    is_installing: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SetMinecraftVersions(Vec<MinecraftVersion>),
    SelectMinecraftVersion(MinecraftVersion),
    SetLoaderVersions(Vec<LoaderVersion>),
    SelectLoaderVersion(LoaderVersion),
    DirectoryInputChangeButtonPressed,
    ShowMinecraftSnapshotsCheckmarkChanged(bool),
    ShowLoaderBetasCheckmarkChanged(bool),
    CreateProfileCheckmarkChanged(bool),
    Install,
    InstallationDone,
}

#[cfg(target_os = "windows")]
fn get_default_client_directory() -> PathBuf {
    let mut dir = PathBuf::from(std::env::var("APPDATA").unwrap());
    dir.push(".minecraft");
    dir
}

#[cfg(target_os = "macos")]
fn get_default_client_directory() -> PathBuf {
    let mut dir = PathBuf::from(std::env::var("HOME").unwrap());
    dir.push("Library");
    dir.push("Application Support");
    dir.push("minecraft");
    dir
}

#[cfg(target_os = "linux")]
fn get_default_client_directory() -> PathBuf {
    let mut dir = PathBuf::from(std::env::var("HOME").unwrap());
    dir.push(".minecraft");
    dir
}

impl Application for Installer {
    type Executor = executor::Default;
    type Flags = Args;
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Installer {
                minecraft_versions: vec![],
                selected_minecraft_version: None,
                loader_versions: vec![],
                selected_loader_version: None,
                show_loader_betas: false, 
                show_minecraft_snapshots: false,
                directory: get_default_client_directory(),
                create_profile: true,
                is_installing: false,
            },
            Command::batch([
                // TODO - Do better error handling
                Command::perform(fetch_minecraft_versions(), |result| {
                    // The plan for error handling without shenanigans is simple!
                    // If it succeeds, we execute this message:
                    Message::SetMinecraftVersions(result.unwrap())
                    // But if it doesn't? We'll execute another one that will handle the error smoothly
                }),
                Command::perform(fetch_loader_versions(), |result| {
                    Message::SetLoaderVersions(result.unwrap())
                }),
            ])
        )
    }

    fn title(&self) -> String {
        String::from("Quilt Installer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetMinecraftVersions(versions) => {
                self.minecraft_versions = versions.clone();
                self.selected_minecraft_version = versions.iter().filter(|v| v.stable).cloned().next();
            },
            Message::SelectMinecraftVersion(version) => self.selected_minecraft_version = Some(version),
            Message::SetLoaderVersions(versions) => {
                self.loader_versions = versions.clone();
                self.selected_loader_version = versions.iter().filter(|v| !v.version.contains('-')).cloned().next();
            },
            Message::SelectLoaderVersion(version) => self.selected_loader_version = Some(version),
            Message::DirectoryInputChangeButtonPressed => {
                let mut dialog = FileDialog::new();
                let working_dir = std::env::current_dir();
                
                if self.directory.is_dir() {
                    dialog = dialog.set_location(&self.directory);
                } else if working_dir.is_ok() {
                    dialog = dialog.set_location(working_dir.as_deref().unwrap())
                }
                let result = dialog.show_open_single_dir();

                match result {
                    Ok(Some(path)) => self.directory = path,
                    Ok(None) => (),
                    Err(_error) => (),
                }
            },
            Message::ShowMinecraftSnapshotsCheckmarkChanged(show_minecraft_snapshots) => {
                self.show_minecraft_snapshots = show_minecraft_snapshots;
                let old_selected_minecraft_version = self.selected_minecraft_version.clone();
                if old_selected_minecraft_version.is_some() && !old_selected_minecraft_version.unwrap().stable {
                    self.selected_minecraft_version = (self.minecraft_versions.iter().filter(|v| self.show_minecraft_snapshots || v.stable)).cloned().next();
                }
            },
            Message::ShowLoaderBetasCheckmarkChanged(show_loader_betas) => {
                self.show_loader_betas = show_loader_betas;
                let old_selected_loader_version = self.selected_loader_version.clone();
                if old_selected_loader_version.is_some() && old_selected_loader_version.unwrap().version.contains('-') {
                    self.selected_loader_version = (self.loader_versions.iter().filter(|v| self.show_loader_betas || !v.version.contains('-'))).cloned().next();
                }
            },
            Message::CreateProfileCheckmarkChanged(create_profile) => self.create_profile = create_profile,
            Message::Install => {
                self.is_installing = true;

                if self.selected_minecraft_version.is_none() {
                    return Command::none();
                }
                
                if self.selected_loader_version.is_none() {
                    return Command::none();
                }

                return Command::perform(
                    install_client(ClientInstallation {
                        minecraft_version: self.selected_minecraft_version.clone().unwrap(),
                        loader_version: self.selected_loader_version.clone().unwrap(),
                        install_location: self.directory.clone(),
                        generate_profile: self.create_profile
                    }),
                    |result| {
                        if result.is_ok() {
                            Message::InstallationDone
                        } else {
                            // TODO - Error handle this
                            Message::InstallationDone
                        }
                    }
                );
            },
            Message::InstallationDone => self.is_installing = false,
            // _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let minecraft_version_label = text("Minecraft Version:").font(POPPINS_SEMIBOLD_FONT).width(Length::Units(185));
        let minecraft_version_pick_list = pick_list(
            Cow::from_iter((self.minecraft_versions.iter().filter(|v| self.show_minecraft_snapshots || v.stable)).cloned()),
            self.selected_minecraft_version.clone(),
            Message::SelectMinecraftVersion
        )
        .width(Length::Units(185));

        let loader_version_label = text("Quilt Loader Version:").font(POPPINS_SEMIBOLD_FONT).width(Length::Units(185));
        let loader_version_pick_list = pick_list(
            Cow::from_iter((self.loader_versions.iter().filter(|v| self.show_loader_betas || !v.version.contains('-'))).cloned()),
            self.selected_loader_version.clone(),
            Message::SelectLoaderVersion
        )
        .width(Length::Units(185));

        let versions_row = row![
            column![minecraft_version_label, minecraft_version_pick_list].width(Length::Units(185)).spacing(2),
            column![loader_version_label, loader_version_pick_list].width(Length::Units(185)).spacing(2),
        ].spacing(10);
            
        let directory_label = text("Directory:").font(POPPINS_SEMIBOLD_FONT).height(Length::Units(30));
        let directory_button = button(text("Q").width(Length::Units(30)).horizontal_alignment(Horizontal::Center)).width(Length::Units(30)).on_press(Message::DirectoryInputChangeButtonPressed);
        let directory_label_row = row![directory_label, horizontal_space(Length::Fill), directory_button].width(Length::Units(380));

        let directory_path = text(&self.directory.to_str().unwrap()).width(Length::Units(380)).font(POPPINS_REGULAR_FONT).size(16);
        let directory_column = column![directory_label_row, directory_path].spacing(2);
        
        let options_label = text("Options:").width(Length::Units(380)).font(POPPINS_SEMIBOLD_FONT);
        let show_snapshots_checkbox = checkbox("Show Snapshots", self.show_minecraft_snapshots, Message::ShowMinecraftSnapshotsCheckmarkChanged).width(Length::Units(380));
        let show_loader_betas_checkbox = checkbox("Show Loader Betas", self.show_loader_betas, Message::ShowLoaderBetasCheckmarkChanged).width(Length::Units(380));
        let create_profile_checkbox = checkbox("Create Profile", self.create_profile, Message::CreateProfileCheckmarkChanged).width(Length::Units(380));

        let install = button(text("Install Client")
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Units(250))
                .font(POPPINS_SEMIBOLD_FONT)
            )
            .on_press(Message::Install)
            .padding(10);
        
        // This is a placeholder for a separate "Installing" view!
        let installing_text = text("Installing...");

        let banner = container(
            container(row![]).center_x().center_y()
        )
        .width(Length::Fill)
        .height(Length::Units(100))
        .style(Container::Box);

        let mut content = iced::widget::column![
            versions_row,
            vertical_space(Length::Units(2)),
            directory_column,
            vertical_space(Length::Units(2)),
            options_label,
            show_snapshots_checkbox,
            show_loader_betas_checkbox,
            create_profile_checkbox,
            vertical_space(Length::Fill),
            install,
            vertical_space(Length::Units(5)),
        ]
        .align_items(Alignment::Center)
        .spacing(5)
        .padding(5)
        .width(Length::Fill);
        
        if self.is_installing {
            content = content.push(installing_text);
        }

        let all = column![
            banner,
            content
        ];

        all.into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}