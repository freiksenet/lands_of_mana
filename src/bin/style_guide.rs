use bevy::asset::AssetServerSettings;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use lands_of_mana::{
    gui::{widgets::*, GuiContext, TextureType},
    prelude::*,
};

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

fn style_guide_system(mut egui_context: ResMut<EguiContext>, gui_context: Res<GuiContext>) {
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
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0., 5.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(
                egui::RichText::new("Lands of Mana Style Guide")
                    .text_style(egui::TextStyle::Heading),
            );
        });

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        NinePatchWindow::new("I am a window")
            .fixed_size(egui::Vec2::new(16. * 40. - 16., 16. * 20. - 16.))
            .title_bar_nine_patch(*gui_context
                .get_texture_id(TextureType::Window, "dark")
                .unwrap(), egui::vec2(32., 32.))
            .body_nine_patch(*gui_context
                .get_texture_id(TextureType::Window, "bright")
                .unwrap(), egui::vec2(32., 32.))
            .show(ui.ctx(), |ui| {
                ui.label(egui::RichText::new("Heading").text_style(egui::TextStyle::Heading));
                ui.label(
                    egui::RichText::new("Body text is so nice. D'Welt genuch jo gei, dé keen schéi wéi, wa aus hale Räis schéinen. Rout gewëss gewalteg dee ké, ons Noper zënter mä. Dan jo wait jeitzt, ké ons Hären Kënnt d'Musek, fu all Scholl Minutt Nuechtegall. Vu Hämmel d'Blumme Kolrettchen déi. Sin frou Mecht, der geplot Fletschen ké.").text_style(egui::TextStyle::Body),
                );
                ui.label(
                    egui::RichText::new("Smol text is nice too. An taima palis tehto hap. Úil telco nalanta or, oia oaris cotumo elendë ëa, lá vírë tulca timpinen tul. Ar nur onótima taniquelassë. Yá axo ataquë mirilya tanwëataquë, ep nún asar racinë, varta tasar é mat. Up lívë inqua nal, tyávë amanyar goneheca lis lá ")
                        .text_style(egui::TextStyle::Small),
                );
                ui.horizontal(|ui| {
                    ui.add(icon_button(*gui_context
                        .get_texture_id(TextureType::Button, "deep")
                        .unwrap(), *gui_context
                            .get_texture_id(TextureType::IconOutline, "mana-death")
                            .unwrap(), egui::vec2(32., 32.)));
                });
            });
    });
}
