use bevy::{asset::AssetPath, utils::HashMap};
use bevy_egui::{egui, EguiContext, EguiSettings};
use strum_macros::{EnumIter, EnumString};

use crate::{
    game::world::{CapacityResourceType, StockpileResourceType},
    gui::widgets,
    prelude::*,
};

#[derive(Debug)]
pub struct GuiContext {
    textures: HashMap<(TextureType, String), egui::TextureId>,
    texture_atlases: HashMap<egui::TextureId, EguiTextureAtlas>,
    style: GuiStyle,
    desired_width: u32,

    computed_size: Option<egui::Vec2>,
}

#[derive(Debug)]
pub struct EguiTextureAtlas {
    pub texture_id: egui::TextureId,
    rows: usize,
    columns: usize,
    tile_size: egui::Vec2,
}

impl EguiTextureAtlas {
    pub fn get_uv_for_texture_id(&self, texture_id: usize) -> egui::Rect {
        let full_uv = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(
                self.columns as f32 * self.tile_size.x,
                self.rows as f32 * self.tile_size.y,
            ),
        );
        let row = texture_id / self.columns;
        let column = texture_id % self.columns;

        let tile_uv = egui::Rect::from_min_size(
            egui::pos2(
                column as f32 * self.tile_size.x,
                row as f32 * self.tile_size.y,
            ),
            self.tile_size,
        );
        egui::emath::RectTransform::from_to(
            full_uv,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1., 1.)),
        )
        .transform_rect(tile_uv)
    }
}

impl Default for GuiContext {
    fn default() -> Self {
        GuiContext {
            desired_width: 1280,
            style: GuiStyle::default(),
            textures: HashMap::new(),
            texture_atlases: HashMap::new(),
            computed_size: None,
        }
    }
}

#[derive(Debug)]
pub struct GuiStyle {
    // Minimal unit used as a step everywhere
    pub unit: f32,

    pub spacing: f32,
    pub window_margin: f32,
    pub interact_size: (f32, f32),
}

impl Default for GuiStyle {
    fn default() -> Self {
        GuiStyle {
            unit: 4.,
            spacing: 1.,
            window_margin: 2.,
            interact_size: (6., 8.),
        }
    }
}

impl GuiContext {
    pub fn setup(
        &mut self,
        (egui_context, windows, egui_settings, asset_server, ui_assets, icon_assets, atlases): (
            &mut ResMut<EguiContext>,
            &mut ResMut<Windows>,
            &mut ResMut<EguiSettings>,
            &Res<AssetServer>,
            &Res<assets::UiAssets>,
            &Res<assets::IconAssets>,
            &Res<Assets<TextureAtlas>>,
        ),
    ) -> &Self {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(false);
        let window_width = window.physical_width();
        let window_height = window.physical_height();
        let scale = window_width as f64 / self.desired_width as f64;
        self.computed_size = Some(egui::vec2(
            self.desired_width as f32,
            (window_height as f64 / scale).floor() as f32,
        ));
        egui_settings.scale_factor = scale;
        self.setup_textures(egui_context, asset_server, ui_assets, icon_assets, atlases)
            .setup_font_assets(egui_context)
            .setup_styles(egui_context)
    }

    pub fn egui_window_size(&self) -> Option<egui::Vec2> {
        self.computed_size
    }

    fn setup_styles(&self, egui_context: &mut ResMut<EguiContext>) -> &Self {
        let ctx = egui_context.ctx_mut();
        let GuiStyle {
            unit,
            spacing,
            window_margin,
            interact_size,
            ..
        } = self.style;
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::WHITE;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::none();
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::none();
        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(0., egui::Color32::BLACK);
        style.visuals.window_shadow = egui::epaint::Shadow::default();
        style.visuals.window_rounding = egui::Rounding::none();

        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(0., egui::Color32::BLACK);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(0., egui::Color32::BROWN);

        style.spacing.button_padding = egui::Vec2::new(unit * spacing, unit);
        style.spacing.item_spacing = egui::Vec2::new(unit * spacing, unit * spacing);
        style.spacing.window_margin = egui::style::Margin::same(unit * window_margin);
        style.spacing.interact_size =
            egui::Vec2::new(interact_size.0 * unit, interact_size.1 * unit);
        style.spacing.icon_spacing = 2.;

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
        atlases: &Res<Assets<TextureAtlas>>,
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

        self.add_texture_atlas(
            (TextureType::Other, "cursors".to_string()),
            EguiTextureAtlas {
                texture_id: egui_context.add_image(
                    atlases
                        .get(ui_assets.cursors.clone_weak())
                        .unwrap()
                        .texture
                        .clone(),
                ),
                rows: 8,
                columns: 8,
                tile_size: egui::vec2(16., 16.),
            },
        );

        self.add_texture_atlas(
            (TextureType::Other, "clicks".to_string()),
            EguiTextureAtlas {
                texture_id: egui_context.add_image(
                    atlases
                        .get(ui_assets.clicks.clone_weak())
                        .unwrap()
                        .texture
                        .clone(),
                ),
                rows: 12,
                columns: 4,
                tile_size: egui::vec2(16., 16.),
            },
        );

        self.add_texture_atlas(
            (TextureType::Other, "directions".to_string()),
            EguiTextureAtlas {
                texture_id: egui_context.add_image(
                    atlases
                        .get(ui_assets.directions.clone_weak())
                        .unwrap()
                        .texture
                        .clone(),
                ),
                rows: 16,
                columns: 4,
                tile_size: egui::vec2(16., 16.),
            },
        );

        self.add_texture_atlas(
            (TextureType::Other, "selectors".to_string()),
            EguiTextureAtlas {
                texture_id: egui_context.add_image(
                    atlases
                        .get(ui_assets.selectors.clone_weak())
                        .unwrap()
                        .texture
                        .clone(),
                ),
                rows: 5,
                columns: 4,
                tile_size: egui::vec2(16., 16.),
            },
        );

        self.add_texture_atlas(
            (TextureType::Other, "badges".to_string()),
            EguiTextureAtlas {
                texture_id: egui_context.add_image(
                    atlases
                        .get(ui_assets.badges.clone_weak())
                        .unwrap()
                        .texture
                        .clone(),
                ),
                rows: 2,
                columns: 8,
                tile_size: egui::vec2(16., 24.),
            },
        );

        self
    }

    fn add_texture_atlas(&mut self, key: (TextureType, String), atlas: EguiTextureAtlas) -> &Self {
        self.textures.insert(key, atlas.texture_id);
        self.texture_atlases.insert(atlas.texture_id, atlas);
        self
    }

    pub fn get_texture_id(
        &self,
        texture_type: TextureType,
        name: &str,
    ) -> Option<&egui::TextureId> {
        self.textures.get(&(texture_type, name.to_owned()))
    }

    pub fn get_texture_atlas(&self, texture_type: TextureType, name: &str) -> &EguiTextureAtlas {
        let texture_id = self.get_texture_id_unwrap(texture_type, name);
        self.texture_atlases
            .get(&texture_id)
            .unwrap_or_else(|| panic!("Cannot find texture atlast {:?}", (texture_type, name)))
    }

    pub fn get_texture_id_unwrap(&self, texture_type: TextureType, name: &str) -> egui::TextureId {
        *self
            .textures
            .get(&(texture_type, name.to_owned()))
            .unwrap_or_else(|| panic!("Cannot find texture {:?}", (texture_type, name)))
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

fn get_name(asset_path: &AssetPath) -> String {
    asset_path
        .path()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter, EnumString)]
pub enum TextureType {
    IconOutline,
    Window,
    Button,
    Other,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter, EnumString)]
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter, EnumString)]
pub enum ButtonType {
    Deep,
    DeepOutline,
    Shallow,
    ShallowOutline,
}

impl ButtonType {
    pub fn to_name(&self) -> &str {
        match self {
            ButtonType::Deep => "deep",
            ButtonType::DeepOutline => "deep_outline",
            ButtonType::Shallow => "shallow",
            ButtonType::ShallowOutline => "shallow_outline",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter, EnumString)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    pub fn to_nine_patch_size(&self) -> egui::Vec2 {
        match self {
            ButtonSize::Small => egui::vec2(8., 8.),
            ButtonSize::Medium => egui::vec2(8., 8.),
            ButtonSize::Large => egui::vec2(16., 16.),
        }
    }

    pub fn to_min_icon_button_size(&self, style: &GuiStyle) -> egui::Vec2 {
        match self {
            ButtonSize::Small => egui::vec2(style.unit * 4., style.unit * 4.),
            ButtonSize::Medium => egui::vec2(style.unit * 6., style.unit * 6.),
            ButtonSize::Large => egui::vec2(style.unit * 8., style.unit * 8.),
        }
    }

    pub fn to_icon_size(&self, style: &GuiStyle) -> egui::Vec2 {
        match self {
            ButtonSize::Small => egui::vec2(style.unit * 3., style.unit * 3.),
            ButtonSize::Medium => egui::vec2(style.unit * 4., style.unit * 4.),
            ButtonSize::Large => egui::vec2(style.unit * 5., style.unit * 5.),
        }
    }

    pub fn to_min_button_size(&self, style: &GuiStyle) -> egui::Vec2 {
        match self {
            ButtonSize::Small => egui::vec2(style.unit * 8., style.unit * 4.),
            ButtonSize::Medium => egui::vec2(style.unit * 16., style.unit * 6.),
            ButtonSize::Large => egui::vec2(style.unit * 32., style.unit * 8.),
        }
    }

    pub fn to_text_size(&self) -> f32 {
        match self {
            ButtonSize::Small => 10.,
            ButtonSize::Medium => 12.,
            ButtonSize::Large => 15.,
        }
    }
}

impl GuiContext {
    pub fn icon_button(
        &self,
        button_type: &ButtonType,
        button_size: &ButtonSize,
        icon_name: &str,
    ) -> impl egui::Widget {
        let icon_size = button_size.to_min_icon_button_size(&self.style);
        widgets::icon_button(
            self.get_texture_id_unwrap(TextureType::Button, button_type.to_name()),
            self.get_texture_id_unwrap(TextureType::IconOutline, icon_name),
            icon_size,
        )
    }

    pub fn button(
        &self,
        button_type: &ButtonType,
        button_size: &ButtonSize,
        text: &str,
        // icon_name: Option<&str>,
    ) -> impl egui::Widget {
        widgets::button(
            (
                button_size.to_nine_patch_size(),
                self.get_texture_id_unwrap(TextureType::Button, button_type.to_name()),
            ),
            egui::RichText::new(text)
                .size(button_size.to_text_size())
                .into(),
            button_size.to_min_button_size(&self.style),
            None,
        )
    }

    pub fn button_with_icon(
        &self,
        button_type: &ButtonType,
        button_size: &ButtonSize,
        text: &str,
        icon_name: &str,
    ) -> impl egui::Widget {
        widgets::button(
            (
                button_size.to_nine_patch_size(),
                self.get_texture_id_unwrap(TextureType::Button, button_type.to_name()),
            ),
            egui::RichText::new(text)
                .size(button_size.to_text_size())
                .into(),
            button_size.to_min_button_size(&self.style),
            Some((
                button_size.to_icon_size(&self.style),
                self.get_texture_id_unwrap(TextureType::IconOutline, icon_name),
            )),
        )
    }
}
