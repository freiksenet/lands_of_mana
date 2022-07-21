use crate::prelude::{
    egui,
    egui::{emath::RectTransform, layers::ShapeIdx},
    *,
};

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
    uv: Option<egui::Rect>,
    rect: Option<egui::Rect>,
}

pub struct NinePatchPrepared {
    texture_id: egui::TextureId,
    size: egui::Vec2,
    shape_idx: ShapeIdx,
    uv: egui::Rect,
    rect: Option<egui::Rect>,
}

impl NinePatch {
    pub fn new(texture_id: egui::TextureId, size: egui::Vec2) -> Self {
        NinePatch {
            texture_id,
            size,
            uv: None,
            rect: None,
        }
    }

    pub fn uv(mut self, uv: egui::Rect) -> Self {
        self.uv = Some(uv);
        self
    }

    pub fn rect(mut self, rect: egui::Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    pub fn begin(self, ui: &mut egui::Ui) -> NinePatchPrepared {
        let margin = ui.style_mut().spacing.window_margin;
        ui.style_mut().spacing.window_margin = egui::style::Margin::same(0.);
        let shape_idx = ui.painter().add(egui::Shape::Noop);
        ui.style_mut().spacing.window_margin = margin;
        let uv = match self.uv {
            Some(uv) => uv,
            _ => egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(1., 1.)),
        };
        NinePatchPrepared {
            texture_id: self.texture_id,
            size: self.size,
            shape_idx,
            uv,
            rect: self.rect,
        }
    }
}

impl NinePatchPrepared {
    pub fn end(&self, ui: &mut egui::Ui) {
        nine_patch_ui(
            ui,
            self.texture_id,
            self.size,
            self.uv,
            self.shape_idx,
            self.rect,
        );
    }
}

fn nine_patch_ui(
    ui: &mut egui::Ui,
    texture_id: egui::TextureId,
    size: egui::Vec2,
    uv: egui::Rect,
    background_idx: ShapeIdx,
    rect: Option<egui::Rect>,
) {
    let rect = match rect {
        Some(rect) => rect,
        _ => ui.min_rect(),
    };
    let _width = (rect.width() / size.x).ceil() as u32;
    let height = (rect.height() / size.y).ceil() as u32;

    if ui.is_rect_visible(rect) {
        let uv_transform = RectTransform::from_to(
            egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(1., 1.)),
            uv,
        );
        let mut mesh = egui::Mesh::with_texture(texture_id);
        if height == 1 {
            small_long_nine_patch_ui(&mut mesh, rect, size, uv_transform);
        // } else if width == 1 {
        //     small_narrow_nine_patch_ui(&mut mesh, rect, size);
        } else {
            big_nine_patch_ui(&mut mesh, rect, size, uv_transform);
        }
        ui.painter().set(background_idx, mesh)
    }
}

// Less than 2 rows of nine patch
fn small_long_nine_patch_ui(
    mesh: &mut egui::Mesh,
    rect: egui::Rect,
    size: egui::Vec2,
    uv_transform: RectTransform,
) {
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
            NinePatchPart::Corner(Direction::NorthWest) => {
                egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(0.33, top_end))
            }
            NinePatchPart::Corner(Direction::NorthEast) => {
                egui::Rect::from_min_max(egui::pos2(0.66, 0.), egui::pos2(1., top_end))
            }
            NinePatchPart::Corner(Direction::North) => {
                egui::Rect::from_min_max(egui::pos2(0.33, 0.), egui::pos2(0.66, top_end))
            }
            _ => egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66)),
        };

        mesh.add_rect_with_uv(
            top_rect,
            uv_transform.transform_rect(top_uv_rect),
            egui::Color32::WHITE,
        );

        let bottom_rect = egui::Rect::from_min_max(
            egui::pos2(start_x, top + height),
            egui::pos2(start_x + size.x as f32, rect.bottom()),
        );
        let bottom_end = 0.66 + 0.33 * (height / size.y);
        let bottom_uv_rect = match uv_from_index(x, 1, width, 2) {
            NinePatchPart::Corner(Direction::SouthWest) => {
                egui::Rect::from_min_max(egui::pos2(0., bottom_end), egui::pos2(0.33, 1.))
            }
            NinePatchPart::Corner(Direction::SouthEast) => {
                egui::Rect::from_min_max(egui::pos2(0.66, bottom_end), egui::pos2(1., 1.))
            }
            NinePatchPart::Corner(Direction::South) => {
                egui::Rect::from_min_max(egui::pos2(0.33, bottom_end), egui::pos2(0.66, 1.))
            }
            _ => egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66)),
        };

        mesh.add_rect_with_uv(
            bottom_rect,
            uv_transform.transform_rect(bottom_uv_rect),
            egui::Color32::WHITE,
        );
    }
}

// Less than 2 cols of nine patch
// fn small_narrow_nine_patch_ui(mesh: &mut egui::Mesh, rect: egui::Rect, size: egui::Vec2) {
//     panic!("TODO");
// }

// bigger nine patch
fn big_nine_patch_ui(
    mesh: &mut egui::Mesh,
    rect: egui::Rect,
    size: egui::Vec2,
    uv_transform: RectTransform,
) {
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
            mesh.add_rect_with_uv(
                tile_rect,
                uv_transform.transform_rect(uv_rect),
                egui::Color32::WHITE,
            );
        }
    }
}

pub enum NinePatchPart {
    Center,
    Corner(Direction),
}

fn uv_from_index(x: u32, y: u32, width: u32, height: u32) -> NinePatchPart {
    match (x, y) {
        (x, y) if x == 0 && y == 0 => NinePatchPart::Corner(Direction::NorthWest),
        (x, y) if x == width - 1 && y == 0 => NinePatchPart::Corner(Direction::NorthEast),
        (x, y) if x == 0 && y == height - 1 => NinePatchPart::Corner(Direction::SouthWest),
        (x, y) if x == width - 1 && y == height - 1 => NinePatchPart::Corner(Direction::SouthEast),
        (x, _) if x == 0 => NinePatchPart::Corner(Direction::West),
        (x, _) if x == width - 1 => NinePatchPart::Corner(Direction::East),
        (_, y) if y == 0 => NinePatchPart::Corner(Direction::North),
        (_, y) if y == height - 1 => NinePatchPart::Corner(Direction::South),
        _ => NinePatchPart::Center,
    }
}

fn big_nine_patch_uv(corner: NinePatchPart) -> egui::Rect {
    match corner {
        NinePatchPart::Corner(Direction::NorthWest) => {
            egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(0.33, 0.33))
        }
        NinePatchPart::Corner(Direction::NorthEast) => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.), egui::pos2(1., 0.33))
        }
        NinePatchPart::Corner(Direction::SouthWest) => {
            egui::Rect::from_min_max(egui::pos2(0., 0.66), egui::pos2(0.33, 1.))
        }
        NinePatchPart::Corner(Direction::SouthEast) => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.66), egui::pos2(1., 1.))
        }
        NinePatchPart::Corner(Direction::West) => {
            egui::Rect::from_min_max(egui::pos2(0., 0.33), egui::pos2(0.33, 0.66))
        }
        NinePatchPart::Corner(Direction::East) => {
            egui::Rect::from_min_max(egui::pos2(0.66, 0.33), egui::pos2(1., 0.66))
        }
        NinePatchPart::Corner(Direction::North) => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.), egui::pos2(0.66, 0.33))
        }
        NinePatchPart::Corner(Direction::South) => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.66), egui::pos2(0.66, 1.))
        }
        NinePatchPart::Center => {
            egui::Rect::from_min_max(egui::pos2(0.33, 0.33), egui::pos2(0.66, 0.66))
        }
    }
}
