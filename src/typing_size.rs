use egui::{vec2, Color32, Response, Sense, Ui, Widget, WidgetInfo, WidgetType};

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct Typing {
    /// Uses the style's `interact_size` if `None`.
    width: Option<f32>,
    color: Option<Color32>,
}

impl Typing {
    /// Create a new spinner that uses the style's `interact_size` unless changed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the spinner's size. The size sets both the height and width, as the spinner is always
    /// square. If the size isn't set explicitly, the active style's `interact_size` is used.
    #[inline]
    pub fn width(mut self, size: f32) -> Self {
        self.width = Some(size);
        self
    }

    /// Sets the spinner's color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = Some(color.into());
        self
    }

    fn paint_at(&self, ui: &mut Ui, rect: egui::Rect, radius: f32) {
        if !ui.is_rect_visible(rect) {
            return;
        }

        let dot_amount = 3;
        let speed = 3.0;

        let color = self
            .color
            .unwrap_or_else(|| ui.visuals().strong_text_color());
        let time = ui.input(|i| i.time) * speed;
        let time_offset = 2.0 / dot_amount as f64;

        let spacing = (rect.width() - (2.0 * radius * dot_amount as f32)) / (dot_amount - 1) as f32;

        for index in 0..dot_amount {
            let factor = (time - time_offset * index as f64).sin() * 0.3 + 0.7;

            ui.painter().circle_filled(
                rect.left_center() + vec2(radius + (radius * 2.0 + spacing) * index as f32, 0.0),
                radius * factor as f32,
                color,
            );
        }
    }
}

impl Widget for Typing {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = self
            .width
            .unwrap_or_else(|| ui.style().spacing.interact_size.x);

        let radius = width * 0.1;

        let (rect, response) = ui.allocate_exact_size(vec2(width, radius * 2.0), Sense::hover());
        response.widget_info(|| WidgetInfo::new(WidgetType::ProgressIndicator));
        self.paint_at(ui, rect, radius);

        response
    }
}
