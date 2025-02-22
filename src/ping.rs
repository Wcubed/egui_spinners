use egui::{vec2, Color32, Response, Sense, Spinner, Stroke, Ui, Widget, WidgetInfo, WidgetType};

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct Ping {
    /// Uses the style's `interact_size` if `None`.
    size: Option<f32>,
    color: Option<Color32>,
}

impl Ping {
    /// Create a new spinner that uses the style's `interact_size` unless changed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the spinner's size. The size sets both the height and width, as the spinner is always
    /// square. If the size isn't set explicitly, the active style's `interact_size` is used.
    #[inline]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the spinner's color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = Some(color.into());
        self
    }

    fn paint_at(&self, ui: &mut Ui, rect: egui::Rect) {
        if !ui.is_rect_visible(rect) {
            return;
        }

        let color = self
            .color
            .unwrap_or_else(|| ui.visuals().strong_text_color());
        let max_radius = rect.height() / 2.0;
        let time = ui.input(|i| i.time);
        let progress = (time % 1.0) as f32;
        let radius = max_radius * progress;

        ui.painter()
            .circle_filled(rect.center(), max_radius * 0.1, color);
        ui.painter().circle_stroke(
            rect.center(),
            radius,
            Stroke::new(1.0, color.gamma_multiply(1.0 - progress)),
        );
    }
}

impl Widget for Ping {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self
            .size
            .unwrap_or_else(|| ui.style().spacing.interact_size.y);

        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
        response.widget_info(|| WidgetInfo::new(WidgetType::ProgressIndicator));
        self.paint_at(ui, rect);

        response
    }
}
