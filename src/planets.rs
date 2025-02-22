use egui::{vec2, Color32, Response, Sense, Stroke, Ui, Vec2, Widget, WidgetInfo, WidgetType};

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct Planets {
    /// Uses the style's `interact_size` if `None`.
    size: Option<f32>,
    color: Option<Color32>,
}

impl Planets {
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
        let time = ui.input(|i| i.time);

        let planet1_radius = rect.width() * 0.05;
        let planet1_orbit = rect.width() * 0.3 - planet1_radius;
        let planet1_pos = rect.center() + Vec2::angled((time * 3.0) as f32) * planet1_orbit;

        let planet2_radius = rect.width() * 0.05;
        let planet2_orbit = rect.width() * 0.5 - planet2_radius;
        let planet2_pos = rect.center() + Vec2::angled((time * 2.0) as f32) * planet2_orbit;

        // Sun
        ui.painter()
            .circle_filled(rect.center(), rect.width() * 0.1, color);

        // Orbits
        let orbit_stroke = Stroke::new(1.0, color.gamma_multiply(0.2));
        ui.painter()
            .circle_stroke(rect.center(), planet1_orbit, orbit_stroke);
        ui.painter()
            .circle_stroke(rect.center(), planet2_orbit, orbit_stroke);

        // Planets
        ui.painter()
            .circle_filled(planet1_pos, planet1_radius, color);
        ui.painter()
            .circle_filled(planet2_pos, planet2_radius, color);
    }
}

impl Widget for Planets {
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
