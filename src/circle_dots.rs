use std::f32;
use std::f64;

use egui::{vec2, Color32, Response, Sense, Ui, Vec2, Widget, WidgetInfo, WidgetType};

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct CircleDots {
    /// Uses the style's `interact_size` if `None`.
    size: Option<f32>,
    color: Option<Color32>,
}

impl CircleDots {
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
        let speed = 3.0;
        let time = -ui.input(|i| i.time) * speed;

        let dot_radius = rect.width() * 0.1;
        let main_radius = (rect.width() / 2.0) - dot_radius;

        let dot_amount = 8;
        let angle_offset = f32::consts::TAU / dot_amount as f32;

        for index in 0..dot_amount {
            let position = rect.center() + Vec2::angled(angle_offset * index as f32) * main_radius;
            let factor = (time + angle_offset as f64 * index as f64).sin() * 0.4 + 0.6;

            ui.painter()
                .circle_filled(position, dot_radius, color.gamma_multiply(factor as f32));
        }
    }
}

impl Widget for CircleDots {
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
