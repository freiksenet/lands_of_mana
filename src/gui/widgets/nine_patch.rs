use crate::prelude::{egui, egui::layers::ShapeIdx};

// pub trait NinePatchWindow {
//     fn show_nine_patch<R>(
//         self,
//         ctx: &egui::Context,
//         options: NinePatchOptions,
//         add_contents: impl FnOnce(&mut egui::Ui) -> R,
//     ) -> Option<egui::InnerResponse<Option<R>>>;
// }

// impl<'open> NinePatchWindow for egui::Window<'open> {
//     fn show_nine_patch<R>(
//         self,
//         ctx: &egui::Context,
//         options: NinePatchOptions,
//         add_contents: impl FnOnce(&mut egui::Ui) -> R,
//     ) -> Option<egui::InnerResponse<Option<R>>> {
//         self.frame(egui::Frame::none().inner_margin(0.).outer_margin(0.))
//             .show(ctx, |ui| {
//                 ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
//                 let background_idx = ui.painter().add(egui::Shape::Noop);
//                 let frame_base =
//                     egui::Frame::none().inner_margin(egui::style::Margin::symmetric(4., 4.));
//                 let mut frame = frame_base.begin(ui);
//                 let response = add_contents(&mut frame.content_ui);
//                 frame.end(ui);
//                 response
//             })
//     }
// }

pub struct NinePatch {
    texture_id: egui::TextureId,
    size: egui::Vec2,
    shape_idx: ShapeIdx,
}

impl NinePatch {
    pub fn begin(ui: &mut egui::Ui, texture_id: egui::TextureId, size: egui::Vec2) -> Self {
        let margin = ui.style_mut().spacing.window_margin;
        ui.style_mut().spacing.window_margin = egui::style::Margin::same(0.);
        let shape_idx = ui.painter().add(egui::Shape::Noop);
        ui.style_mut().spacing.window_margin = margin;
        NinePatch {
            texture_id,
            size,
            shape_idx,
        }
    }

    pub fn end(&self, ui: &mut egui::Ui) {
        nine_patch_ui(ui, self.texture_id, self.size, self.shape_idx);
    }
}

fn nine_patch_ui(
    ui: &mut egui::Ui,
    texture_id: egui::TextureId,
    size: egui::Vec2,
    background_idx: ShapeIdx,
) {
    let rect = ui.min_rect();
    let width = (rect.width() / size.x).ceil() as u32;
    let height = (rect.height() / size.y).ceil() as u32;

    if ui.is_rect_visible(rect) {
        let mut mesh = egui::Mesh::with_texture(texture_id);
        if height == 1 {
            small_long_nine_patch_ui(&mut mesh, rect, size);
        } else if width == 1 {
            small_narrow_nine_patch_ui(&mut mesh, rect, size);
        } else {
            big_nine_patch_ui(&mut mesh, rect, size);
        }
        ui.painter().set(background_idx, mesh)
    }
}

// Less than 2 rows of nine patch
fn small_long_nine_patch_ui(mesh: &mut egui::Mesh, rect: egui::Rect, size: egui::Vec2) {
    let left = rect.left();
    let missing_width = match rect.width() % size.x {
        rem if rem == 0. => 0.,
        rem => size.x - rem,
    };
    let top = rect.top();
    let width = (rect.width() / size.x).ceil() as u32;
    let height = rect.height() / 2.;

    for x in 0..width {
        let start_x = x as f32 * size.x + left - if x == width - 1 { missing_width } else { 0. };
        let top_rect = egui::Rect::from_min_max(
            egui::pos2(start_x, top),
            egui::pos2(start_x + size.x as f32, top + height),
        );
        let top_end = 0.33 * (height / size.y);
        let top_uv_rect = match uv_from_index(x, 0, width, 2) {
            NinePatchCorner::NorthWest => {
                egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(0.33, top_end))
            }
            NinePatchCorner::NorthEast => {
                egui::Rect::from_min_max(egui::pos2(0.66, 0.), egui::pos2(1., top_end))
            }
            NinePatchCorner::North => {
                egui::Rect::from_min_max(egui::pos2(0.33, 0.), egui::pos2(0.66, top_end))
            }
            _ => egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66)),
        };

        mesh.add_rect_with_uv(top_rect, top_uv_rect, egui::Color32::WHITE);

        let bottom_rect = egui::Rect::from_min_max(
            egui::pos2(start_x, top + height),
            egui::pos2(start_x + size.x as f32, rect.bottom()),
        );
        let bottom_end = 0.66 + 0.33 * (height / size.y);
        let bottom_uv_rect = match uv_from_index(x, 1, width, 2) {
            NinePatchCorner::SouthWest => {
                egui::Rect::from_min_max(egui::pos2(0., bottom_end), egui::pos2(0.33, 1.))
            }
            NinePatchCorner::SouthEast => {
                egui::Rect::from_min_max(egui::pos2(0.66, bottom_end), egui::pos2(1., 1.))
            }
            NinePatchCorner::South => {
                egui::Rect::from_min_max(egui::pos2(0.33, bottom_end), egui::pos2(0.66, 1.))
            }
            _ => egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66)),
        };

        mesh.add_rect_with_uv(bottom_rect, bottom_uv_rect, egui::Color32::WHITE);
    }
}

// Less than 2 cols of nine patch
fn small_narrow_nine_patch_ui(mesh: &mut egui::Mesh, rect: egui::Rect, size: egui::Vec2) {
    panic!("TODO");
}

// bigger nine patch
fn big_nine_patch_ui(mesh: &mut egui::Mesh, rect: egui::Rect, size: egui::Vec2) {
    let left = rect.left();
    let width = (rect.width() / size.x).ceil() as u32;
    let missing_width = match rect.width() % size.x {
        rem if rem == 0. => 0.,
        rem => size.x - rem,
    };
    let top = rect.top();
    let height = (rect.height() / size.y).ceil() as u32;
    let missing_height = match rect.height() % size.y {
        rem if rem == 0. => 0.,
        rem => size.y - rem,
    };
    for x in 0..width {
        for y in 0..height {
            let start_x =
                x as f32 * size.x + left - if x == width - 1 { missing_width } else { 0. };
            let start_y =
                y as f32 * size.y + top - if y == height - 1 { missing_height } else { 0. };
            let tile_rect = egui::Rect::from_min_max(
                egui::pos2(start_x, start_y),
                egui::pos2(start_x + size.x as f32, start_y + size.y as f32),
            );

            let uv_rect = big_nine_patch_uv(uv_from_index(x, y, width, height));
            mesh.add_rect_with_uv(tile_rect, uv_rect, egui::Color32::WHITE);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum NinePatchCorner {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    Center,
}

fn uv_from_index(x: u32, y: u32, width: u32, height: u32) -> NinePatchCorner {
    match (x, y) {
        (x, y) if x == 0 && y == 0 => NinePatchCorner::NorthWest,
        (x, y) if x == width - 1 && y == 0 => NinePatchCorner::NorthEast,
        (x, y) if x == 0 && y == height - 1 => NinePatchCorner::SouthWest,
        (x, y) if x == width - 1 && y == height - 1 => NinePatchCorner::SouthEast,
        (x, _) if x == 0 => NinePatchCorner::West,
        (x, _) if x == width - 1 => NinePatchCorner::East,
        (_, y) if y == 0 => NinePatchCorner::North,
        (_, y) if y == height - 1 => NinePatchCorner::South,
        _ => NinePatchCorner::Center,
    }
}

fn big_nine_patch_uv(corner: NinePatchCorner) -> egui::Rect {
    match corner {
        NinePatchCorner::NorthWest => {
            egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(0.33, 0.33))
        }
        NinePatchCorner::NorthEast => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.), egui::pos2(1., 0.33))
        }
        NinePatchCorner::SouthWest => {
            egui::Rect::from_min_max(egui::pos2(0., 0.66), egui::pos2(0.33, 1.))
        }
        NinePatchCorner::SouthEast => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.66), egui::pos2(1., 1.))
        }
        NinePatchCorner::West => {
            egui::Rect::from_min_max(egui::pos2(0., 0.33), egui::pos2(0.33, 0.66))
        }
        NinePatchCorner::East => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.33), egui::pos2(1., 0.66))
        }
        NinePatchCorner::North => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.), egui::pos2(0.66, 0.33))
        }
        NinePatchCorner::South => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.66), egui::pos2(0.66, 1.))
        }
        NinePatchCorner::Center => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66))
        }
    }
}
