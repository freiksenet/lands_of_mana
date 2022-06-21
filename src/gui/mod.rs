use bevy::{asset::AssetPath, utils::HashMap};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

use crate::{
    config::UiSyncLabel,
    game::{
        world::{CapacityResourceType, StockpileResourceType},
        InGameState,
    },
    prelude::*,
};

pub mod systems;
pub mod widgets;

use systems::*;
pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(config::EngineState::LoadingGraphics, egui_setup)
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(config::EngineState::InGame)
                    .label_and_after(UiSyncLabel::Sync)
                    .with_system(bind_current_player_resources)
                    .into(),
            )
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(config::EngineState::InGame)
                    .label_and_after(UiSyncLabel::Update)
                    .with_system(title_bar)
                    .with_system(resource_bar)
                    .with_system(time_bar)
                    .into(),
            );
    }
}

pub struct GuiContext {
    textures: HashMap<(TextureType, String), egui::TextureId>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TextureType {
    IconOutline,
    Window,
    Button,
}

pub enum GuiWindowType {
    Bright,
    Dark,
    GreenOutline,
    Paper,
    ScrollHorizontal,
    ScrollHorizontalWrapped,
    ScrollVertical,
    ScrollVerticalWrapped,
}

pub enum ButtonType {
    Deep,
    DeepOutline,
    Shallow,
    ShallowOutline,
}

impl GuiContext {
    pub fn new() -> Self {
        GuiContext {
            textures: HashMap::new(),
        }
    }

    pub fn setup(
        &mut self,
        egui_context: &mut ResMut<EguiContext>,
        asset_server: &Res<AssetServer>,
        ui_assets: &Res<assets::UiAssets>,
        icon_assets: &Res<assets::IconAssets>,
    ) -> &Self {
        self.setup_textures(egui_context, asset_server, ui_assets, icon_assets)
            .setup_font_assets(egui_context)
            .setup_styles(egui_context)
    }

    fn setup_styles(&self, egui_context: &mut ResMut<EguiContext>) -> &Self {
        let ctx = egui_context.ctx_mut();
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::WHITE;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::none();
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::none();
        style.visuals.window_shadow = egui::epaint::Shadow::default();
        style.visuals.window_rounding = egui::Rounding::none();

        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(0., egui::Color32::BLACK);

        style.spacing.item_spacing = egui::Vec2::new(4., 4.);
        style.spacing.window_margin = egui::style::Margin::same(8.);
        style.spacing.interact_size = egui::Vec2::new(24., 32.);

        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20., egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Name("Heading2".into()),
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        ctx.set_style(style);
        self
    }

    fn setup_font_assets(&self, egui_context: &mut ResMut<EguiContext>) -> &Self {
        let ctx = egui_context.ctx_mut();
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "CompassPro".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/export/fonts/CompassPro.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "CompassPro".to_owned());

        ctx.set_fonts(fonts);
        self
    }

    fn setup_textures(
        &mut self,
        egui_context: &mut ResMut<EguiContext>,
        asset_server: &Res<AssetServer>,
        ui_assets: &Res<assets::UiAssets>,
        icon_assets: &Res<assets::IconAssets>,
    ) -> &Self {
        for handle in ui_assets.buttons.iter() {
            self.textures.insert(
                (
                    TextureType::Button,
                    get_name(&asset_server.get_handle_path(handle).unwrap()),
                ),
                egui_context.add_image(handle.clone()),
            );
        }
        for handle in ui_assets.windows.iter() {
            self.textures.insert(
                (
                    TextureType::Window,
                    get_name(&asset_server.get_handle_path(handle).unwrap()),
                ),
                egui_context.add_image(handle.clone()),
            );
        }
        for handle in icon_assets.outline.iter() {
            self.textures.insert(
                (
                    TextureType::IconOutline,
                    get_name(&asset_server.get_handle_path(handle).unwrap()),
                ),
                egui_context.add_image(handle.clone()),
            );
        }
        self
    }

    pub fn get_texture_id(
        &self,
        texture_type: TextureType,
        name: &str,
    ) -> Option<&egui::TextureId> {
        self.textures.get(&(texture_type, name.to_owned()))
    }

    pub fn icon_texture_id_for_stockpile_resource(
        &self,
        resource_type: &StockpileResourceType,
    ) -> egui::TextureId {
        *match resource_type {
            StockpileResourceType::Gold => self
                .textures
                .get(&(TextureType::IconOutline, "res-gold".to_owned())),
            StockpileResourceType::Wood => self
                .textures
                .get(&(TextureType::IconOutline, "res-wood".to_owned())),
        }
        .unwrap()
    }

    pub fn icon_texture_id_for_capacity_resource(
        &self,
        resource_type: &CapacityResourceType,
    ) -> egui::TextureId {
        *match resource_type {
            CapacityResourceType::Sun => self
                .textures
                .get(&(TextureType::IconOutline, "mana-sun".to_owned())),
            CapacityResourceType::Arcana => self
                .textures
                .get(&(TextureType::IconOutline, "mana-arcana".to_owned())),
            CapacityResourceType::Death => self
                .textures
                .get(&(TextureType::IconOutline, "mana-death".to_owned())),
            CapacityResourceType::Chaos => self
                .textures
                .get(&(TextureType::IconOutline, "mana-chaos".to_owned())),
            CapacityResourceType::Nature => self
                .textures
                .get(&(TextureType::IconOutline, "mana-nature".to_owned())),
        }
        .unwrap()
    }
}

impl Default for GuiContext {
    fn default() -> Self {
        Self::new()
    }
}

fn get_name(asset_path: &AssetPath) -> String {
    asset_path
        .path()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn egui_setup(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    ui_assets: Res<assets::UiAssets>,
    icon_assets: Res<assets::IconAssets>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    commands.init_resource::<PlayerResources>();
    egui_settings.scale_factor = 2.; // TODO: always scale to like 800
    let mut gui_context = GuiContext::new();
    gui_context.setup(&mut egui_context, &asset_server, &ui_assets, &icon_assets);
    commands.insert_resource(gui_context);
}
