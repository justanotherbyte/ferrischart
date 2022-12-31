use image::{RgbImage, Rgb};
use imageproc::drawing;

pub struct ScatterGraph<'a> {
    x_axis_text: &'a str,
    y_axis_text: &'a str

}

impl<'a> Default for ScatterGraph<'a> {
    fn default() -> Self {
        Self {
            x_axis_text: "unset",
            y_axis_text: "unset"
        }
    }
}

impl<'a> ScatterGraph<'a> {
    pub fn build() -> Self {
        Self::default()
    }
    pub fn set_axis_text(mut self, x_axis_text: &'a str, y_axis_text: &'a str) -> Self {
        self.x_axis_text = x_axis_text;
        self.y_axis_text = y_axis_text;

        self
    }
    pub fn draw(self, path: &str) {
        // first step is to create a basic white image
        let mut canvas = RgbImage::new(500, 500);
        canvas.fill(255); // fill it with 255 to make it white

        // now we need to draw the axis lines
        // we'll use a padding of 50
        let line_color = Rgb([0, 0, 0]); // black
        let top_left = (50f32, 50f32);

        // draw y-axis
        let y_end = (50f32, 450f32);
        drawing::draw_line_segment_mut(&mut canvas, top_left, y_end, line_color);

        // draw x-axis
        let x_end = (450f32, 450f32);
        drawing::draw_line_segment_mut(&mut canvas, y_end, x_end, line_color);

        // write axis texts
        let font_data = include_bytes!("../assets/Roboto-Black.ttf");
        
        // save image
        canvas.save(path).unwrap();
    }
}