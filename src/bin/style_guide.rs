

use bevy::asset::AssetServerSettings;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use lands_of_mana::{
    gui::{widgets::*, *},
    prelude::*,
};
use strum::IntoEnumIterator;

fn main() {
    let window = WindowDescriptor {
        mode: bevy::window::WindowMode::BorderlessFullscreen,
        title: String::from("mom4x"),
        // width: 1600.,
        // height: 1000.,
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window)
        .insert_resource(AssetServerSettings {
            asset_folder: "assets/export".to_string(),
            watch_for_changes: true,
        })
        .init_resource::<StyleGuideResource>()
        .insert_resource(Msaa { samples: 1 })
        .add_loopless_state(config::EngineState::LoadingAssets);

    app.add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(bevy::window::exit_on_window_close_system)
        .add_plugin(assets::AssetLoadingPlugin {})
        .add_plugin(EguiPlugin)
        .add_exit_system(config::EngineState::LoadingAssets, gui::egui_setup)
        .add_system(style_guide_system.run_in_state(config::EngineState::LoadingAssets.next()))
        .run();
}

pub struct StyleGuideResource {
    window_title: String,
    window_body: String,
}

const TITLE_STYLES: [&str; 8] = [
    "bright",
    "dark",
    "green_outline",
    "paper",
    "scroll_horizontal_wrapped",
    "scroll_horizontal",
    "scroll_vertical_wrapped",
    "scroll_vertical",
];

impl Default for StyleGuideResource {
    fn default() -> Self {
        StyleGuideResource {
            window_title: "dark".to_string(),
            window_body: "bright".to_string(),
        }
    }
}

impl StyleGuideResource {
    fn next_window_style(style: &str) -> &'static str {
        let mut current_pos = TITLE_STYLES.into_iter().position(|s| s == style).unwrap() + 1;
        if current_pos == TITLE_STYLES.len() {
            current_pos = 0;
        }
        TITLE_STYLES[current_pos]
    }
}

fn style_guide_system(
    mut egui_context: ResMut<EguiContext>,
    gui_context: Res<GuiContext>,
    mut style_guide_resource: ResMut<StyleGuideResource>,
) {
    NinePatchWindow::new("Title Bar")
        .title_bar(false)
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "scroll_horizontal_wrapped")
                .unwrap(),
            egui::vec2(32., 16.),
        )
        .frame(
            egui::Frame::window(&egui_context.ctx_mut().style())
                .inner_margin(egui::style::Margin::symmetric(32., 8.)),
        )
        .auto_sized()
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0., 4.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(
                egui::RichText::new("Lands of Mana Style Guide")
                    .text_style(egui::TextStyle::Heading),
            );
        });

    NinePatchWindow::new("I am a window")
        .default_pos(egui::pos2((1280. - 1024.) / 2., 64.))
        .auto_sized()
        .min_width(1000.)
        .max_size(egui::vec2(1024., f32::INFINITY))
        .title_bar_nine_patch(
            *gui_context
                .get_texture_id(
                    TextureType::Window,
                    style_guide_resource.window_title.as_str(),
                )
                .unwrap(),
            egui::vec2(32., 32.),
        )
        .body_nine_patch(
            *gui_context
                .get_texture_id(
                    TextureType::Window,
                    style_guide_resource.window_body.as_str(),
                )
                .unwrap(),
            egui::vec2(32., 32.),
        )
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(egui::RichText::new("Heading").text_style(egui::TextStyle::Heading));
            ui.label(egui::RichText::new(ORC_LOREM_IPSUM).text_style(egui::TextStyle::Body));
            ui.label(egui::RichText::new(ELF_LOREM_IPSUM).text_style(egui::TextStyle::Small));
            let mut icons = vec!["mana-death", "mana-chaos", "mana-sun", "res-gold"]
                .into_iter()
                .cycle();
            for button_size in ButtonSize::iter() {
                ui.horizontal_wrapped(|ui| {
                    for button_type in ButtonType::iter() {
                        ui.add(gui_context.icon_button(
                            &button_type,
                            &button_size,
                            icons.next().unwrap(),
                        ));
                        ui.add(gui_context.button(
                            &button_type,
                            &button_size,
                            format!("{:?} {:?} Button", button_size, button_type).as_str(),
                        ));
                        ui.add(gui_context.button_with_icon(
                            &button_type,
                            &button_size,
                            format!("{:?} {:?} Button", button_size, button_type).as_str(),
                            icons.next().unwrap(),
                        ));
                    }
                });
            }
        });

    NinePatchWindow::new("Control Bar")
        .title_bar(false)
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "scroll_vertical")
                .unwrap(),
            egui::vec2(32., 16.),
        )
        .auto_sized()
        .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0., -8.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                let next_title = ui.add(gui_context.button(
                    &ButtonType::Shallow,
                    &ButtonSize::Medium,
                    "Next Title Style",
                ));
                if next_title.clicked() {
                    style_guide_resource.window_title = StyleGuideResource::next_window_style(
                        style_guide_resource.window_title.as_str(),
                    )
                    .to_string();
                }
                let next_body = ui.add(gui_context.button(
                    &ButtonType::Shallow,
                    &ButtonSize::Medium,
                    "Next Body Style",
                ));
                if next_body.clicked() {
                    style_guide_resource.window_body = StyleGuideResource::next_window_style(
                        style_guide_resource.window_body.as_str(),
                    )
                    .to_string();
                }
            });
        });
}

const ORC_LOREM_IPSUM: &str = "\
Boq bor cha'dich chegh choq chuy ghangwi' ghay'cha' gheb ghogh habli' jij jornub\
lolmoh mughato' nguq per qaryoq qeng ron senwi' rilwi' je tagha' tor valqis\
'edsehcha 'e'mam 'ur. Cha'par chegh choq je' me'nal nuh bey' ngav ngun qanwi'\
qewwi' qeylis mindu' quv bey' qutlh tel valqis yintagh yuqjijdivi' 'orghen rojmab\
'orwi'. Bo dav ghaytanha' ghet mah matlh neb parbing pup qin vagh qum segh siq\
taq tepqengwi' til ting tus vay' vin. Bertlham bis'ub cha'qu' chob denibya'ngan hoch\
holqed hongghor logh mah mevyap natlis naw' ne' qaywi' qa'meh qa'ri' qa'vaq qirq\
qay'wi' segh wud 'e'mamnal. Baghneq be'joy' chadvay' chob denib do'ha' je lo' law'\
pivlob qab qan qawhaq qughdo qaq sa'hut toq torgh yo'seh yahniv yuqjijdivi' 'iw 'ip\
ghomey.";

const ELF_LOREM_IPSUM: &str = "\
Wán uë marda lorna ambarmetta, hep entarda maquetta vá. Mear hravan ve mól, ar\
nimba cotumo quí. Mavor rondë aldëon pio uë, é eru fion sindar. Cú hísië mantil\
caimassë ára, toa maren lavralda nó, entarda hlonítë yén pé. Rië nirya tuilë indómë ëa,\
mer be palmë pendë sindar. Pica hlonítë tië uë, eques roina taniquelassë wen né, ta mar\
tehto artuilë terpellië. Assa tárë rauko sir uë, yarë varnë quesset cu rer";
