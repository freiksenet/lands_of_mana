use crate::prelude::{
    egui::{WidgetTextGalley, *},
    gui::widgets::nine_patch::NinePatch,
    *,
};

/// Nine patch window with title that can either be fixed size or auto-sized
///
/// No scrolling yet
///
/// Can be moved and anchored similarly as normal window, just remade so it works fine
/// with ninepatch

pub struct NinePatchWindow {
    title: WidgetText,
    area: Area,
    frame: Option<Frame>,
    resize: Resize,

    with_title_bar: bool,

    title_bar_nine_patch: Option<(TextureId, egui::Vec2)>,
    body_nine_patch: Option<(TextureId, egui::Vec2)>,
}

impl NinePatchWindow {
    pub fn new(title: impl Into<WidgetText>) -> Self {
        let title = title.into().fallback_text_style(egui::TextStyle::Heading);
        let area = Area::new(title.text());
        Self {
            title,
            area,
            frame: None,
            resize: Resize::default()
                .with_stroke(false)
                .min_size([96.0, 32.0])
                .default_size([340.0, 420.0])
                .resizable(false),
            with_title_bar: true,
            title_bar_nine_patch: None,
            body_nine_patch: None,
        }
    }

    pub fn title_bar_nine_patch(mut self, texture_id: TextureId, size: egui::Vec2) -> Self {
        self.title_bar_nine_patch = Some((texture_id, size));
        self
    }

    pub fn body_nine_patch(mut self, texture_id: TextureId, size: egui::Vec2) -> Self {
        self.body_nine_patch = Some((texture_id, size));
        self
    }

    /// Assign a unique id to the Window. Required if the title changes, or is shared with another window.
    pub fn id(mut self, id: Id) -> Self {
        self.area = self.area.id(id);
        self
    }

    /// If `false` the window will be grayed out and non-interactive.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.area = self.area.enabled(enabled);
        self
    }
    /// Usage: `Window::new(…).mutate(|w| w.resize = w.resize.auto_expand_width(true))`
    /// Not sure this is a good interface for this.
    pub fn mutate(mut self, mutate: impl Fn(&mut Self)) -> Self {
        mutate(&mut self);
        self
    }

    /// Usage: `Window::new(…).resize(|r| r.auto_expand_width(true))`
    /// Not sure this is a good interface for this.
    pub fn resize(mut self, mutate: impl Fn(Resize) -> Resize) -> Self {
        self.resize = mutate(self.resize);
        self
    }

    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Set minimum width of the window.
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.resize = self.resize.min_width(min_width);
        self
    }
    /// Set minimum height of the window.
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.resize = self.resize.min_height(min_height);
        self
    }

    /// Set current position of the window.
    /// If the window is movable it is up to you to keep track of where it moved to!
    pub fn current_pos(mut self, current_pos: impl Into<egui::Pos2>) -> Self {
        self.area = self.area.current_pos(current_pos);
        self
    }

    /// Set initial position of the window.
    pub fn default_pos(mut self, default_pos: impl Into<egui::Pos2>) -> Self {
        self.area = self.area.default_pos(default_pos);
        self
    }

    /// Set anchor and distance.
    ///
    /// An anchor of `Align2::RIGHT_TOP` means "put the right-top corner of the window
    /// in the right-top corner of the screen".
    ///
    /// The offset is added to the position, so e.g. an offset of `[-5.0, 5.0]`
    /// would move the window left and down from the given anchor.
    ///
    /// Anchoring also makes the window immovable.
    ///
    /// It is an error to set both an anchor and a position.
    pub fn anchor(mut self, align: Align2, offset: impl Into<egui::Vec2>) -> Self {
        self.area = self.area.anchor(align, offset);
        self
    }

    /// Set initial size of the window.
    pub fn default_size(mut self, default_size: impl Into<egui::Vec2>) -> Self {
        self.resize = self.resize.default_size(default_size);
        self
    }

    /// Set initial width of the window.
    pub fn default_width(mut self, default_width: f32) -> Self {
        self.resize = self.resize.default_width(default_width);
        self
    }
    /// Set initial height of the window.
    pub fn default_height(mut self, default_height: f32) -> Self {
        self.resize = self.resize.default_height(default_height);
        self
    }

    /// Set initial position and size of the window.
    pub fn default_rect(self, rect: egui::Rect) -> Self {
        self.default_pos(rect.min).default_size(rect.size())
    }

    /// Sets the window position and prevents it from being dragged around.
    pub fn fixed_pos(mut self, pos: impl Into<egui::Pos2>) -> Self {
        self.area = self.area.fixed_pos(pos);
        self
    }

    /// Sets the window size and prevents it from being resized by dragging its edges.
    pub fn fixed_size(mut self, size: impl Into<egui::Vec2>) -> Self {
        self.resize = self.resize.fixed_size(size).with_stroke(true);
        self
    }

    pub fn max_size(mut self, size: impl Into<egui::Vec2>) -> Self {
        self.resize = self.resize.max_size(size);
        self
    }

    /// Sets the window pos and size and prevents it from being moved and resized by dragging its edges.
    pub fn fixed_rect(self, rect: egui::Rect) -> Self {
        self.fixed_pos(rect.min).fixed_size(rect.size())
    }

    /// Show title bar on top of the window?
    /// If `false`, the window will not be collapsible nor have a close-button.
    pub fn title_bar(mut self, title_bar: bool) -> Self {
        self.with_title_bar = title_bar;
        self
    }

    /// Not resizable, just takes the size of its contents.
    /// Also disabled scrolling.
    /// Text will not wrap, but will instead make your window width expand.
    pub fn auto_sized(mut self) -> Self {
        self.resize = self.resize.auto_sized().with_stroke(false);
        self
    }
}

impl NinePatchWindow {
    pub fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let NinePatchWindow {
            title,
            area,
            frame,
            resize,
            with_title_bar,
            title_bar_nine_patch,
            body_nine_patch,
        } = self;

        let outer_frame = Frame::window(&ctx.style())
            .inner_margin(0.)
            .fill(Color32::TRANSPARENT);

        let content_frame = Frame::window(&ctx.style())
            .outer_margin(0.)
            .inner_margin(0.)
            .fill(Color32::TRANSPARENT);
        let frame = frame
            .unwrap_or_else(|| Frame::window(&ctx.style()))
            .outer_margin(0.)
            .fill(Color32::TRANSPARENT);
        let area_id = area.id;
        let resize_id = area_id.with("resize");
        let mut resize = resize.id(resize_id);

        let area = area.begin(ctx);

        let mut area_content_ui = area.content_ui(ctx);

        let mut outer_frame = outer_frame.begin(&mut area_content_ui);
        let spacing = outer_frame.content_ui.style().spacing.item_spacing;
        outer_frame.content_ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;

        let title_bar = if with_title_bar {
            let title_bar = show_title_bar(&mut outer_frame.content_ui, title);
            resize.min_size.x = resize.min_size.x.at_least(title_bar.rect.width()); // Prevent making window smaller than title bar width
            Some(title_bar)
        } else {
            None
        };

        let mut content_frame = content_frame.begin(&mut outer_frame.content_ui);
        content_frame.content_ui.style_mut().spacing.item_spacing = spacing;

        let nine_patch =
            if let Some((body_nine_patch_texture, body_nine_patch_size)) = body_nine_patch {
                let nine_patch = NinePatch::new(body_nine_patch_texture, body_nine_patch_size);
                Some(nine_patch.begin(&mut content_frame.content_ui))
            } else {
                None
            };

        let mut frame = frame.begin(&mut content_frame.content_ui);

        let inner_response = frame
            .content_ui
            .scope(|frame_content_ui| resize.show(frame_content_ui, |ui| add_contents(ui)));

        frame.end(&mut content_frame.content_ui);

        if let Some(nine_patch) = nine_patch {
            nine_patch.end(&mut content_frame.content_ui);
        }

        content_frame.end(&mut outer_frame.content_ui);

        let frame_response = outer_frame.end(&mut area_content_ui);

        if let Some(title_bar) = title_bar {
            title_bar.ui(
                &mut area_content_ui,
                // frame_response.rect,
                frame_response,
                title_bar_nine_patch,
            );
        }

        let full_response = area.end(ctx, area_content_ui);

        InnerResponse {
            inner: inner_response.inner,
            response: full_response,
        }
    }
}

struct TitleBar {
    // /// A title Id used for dragging windows
    // id: Id,
    /// Prepared text in the title
    title_galley: WidgetTextGalley,
    /// Size of the title bar in a collapsed state (if window is collapsible),
    /// which includes all necessary space for showing the expand button, the
    /// title and the close button.
    min_rect: egui::Rect,
    /// Size of the title bar in an expanded state. This size become known only
    /// after expanding window and painting its content
    rect: egui::Rect,
}

fn show_title_bar(ui: &mut Ui, title: WidgetText) -> TitleBar {
    let inner_response = ui.horizontal(|ui| {
        let height = match &title {
            WidgetText::RichText(text) => text
                .font_height(&ui.fonts(), ui.style())
                .max(ui.spacing().interact_size.y),
            _ => ui.spacing().interact_size.y,
        };
        ui.set_min_height(height);

        let item_spacing = ui.spacing().item_spacing;
        let button_size = egui::Vec2::splat(ui.spacing().icon_width);

        let pad = (height - button_size.y) / 2.0; // calculated so that the icon is on the diagonal (if window padding is symmetrical)

        let title_galley =
            title.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Heading);

        let minimum_width = 2.0 * (pad + button_size.x + item_spacing.x) + title_galley.size().x;
        let min_rect = egui::Rect::from_min_size(ui.min_rect().min, vec2(minimum_width, height));
        let _response = ui.allocate_rect(min_rect, Sense::click());

        TitleBar {
            // id: response.id,
            title_galley,
            min_rect,
            rect: egui::Rect::NAN, // Will be filled in later
        }
    });

    let title_bar = inner_response.inner;
    let rect = inner_response.response.rect;

    TitleBar { rect, ..title_bar }
}

impl TitleBar {
    /// Finishes painting of the title bar when the window content size already known.
    ///
    /// # Parameters
    ///
    /// - `ui`:
    /// - `outer_rect`:
    /// - `content_response`: if `None`, window is collapsed at this frame, otherwise contains
    ///   a result of rendering the window content
    /// - `open`: if `None`, no "Close" button will be rendered, otherwise renders and processes
    ///   the "Close" button and writes a `false` if window was closed
    /// - `collapsing`: holds the current expanding state. Can be changed by double click on the
    ///   title if `collapsible` is `true`
    /// - `collapsible`: if `true`, double click on the title bar will be handled for a change
    ///   of `collapsing` state
    fn ui(
        mut self,
        ui: &mut Ui,
        content_response: Response,
        nine_patch_options_option: Option<(TextureId, egui::Vec2)>,
    ) {
        // Now we know how large we got to be:
        self.rect.max.x = self.rect.max.x.max(content_response.rect.max.x);

        let full_top_rect =
            egui::Rect::from_x_y_ranges(self.rect.x_range(), self.min_rect.y_range());

        ui.allocate_ui_at_rect(full_top_rect, |ui| {
            ui.allocate_rect(full_top_rect, Sense::focusable_noninteractive());
            let nine_patch =
                if let Some((nine_patch_texture_id, nine_patch_size)) = nine_patch_options_option {
                    let nine_patch = NinePatch::new(nine_patch_texture_id, nine_patch_size);
                    Some(nine_patch.begin(ui))
                } else {
                    None
                };

            let text_pos =
                emath::align::center_size_in_rect(self.title_galley.size(), full_top_rect)
                    .left_top();
            let text_pos = text_pos - self.title_galley.galley().rect.min.to_vec2();
            let text_pos = text_pos - 1.5 * egui::Vec2::Y; // HACK: center on x-height of text (looks better)
            self.title_galley.paint_with_fallback_color(
                ui.painter(),
                text_pos,
                ui.visuals().text_color(),
            );

            if let Some(nine_patch) = nine_patch {
                nine_patch.end(ui);
            }
        });

        // // paint separator between title and content:
        // let y = content_response.rect.top() + ui.spacing().item_spacing.y * 0.5;
        // // let y = lerp(self.rect.bottom()..=content_response.rect.top(), 0.5);
        // let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        // ui.painter().hline(outer_rect.x_range(), y, stroke);
    }
}
