use image::{RgbImage, Rgb, imageops};
use imageproc::drawing;
use rusttype::{Font, Scale};

pub struct ScatterGraph<'a> {
    x_axis_text: &'a str,
    y_axis_text: &'a str,
    x_labels: Vec<&'a str>,
    y_labels: Vec<&'a str>
}

impl<'a> Default for ScatterGraph<'a> {
    fn default() -> Self {
        Self {
            x_axis_text: "unset",
            y_axis_text: "unset",
            x_labels: vec![],
            y_labels: vec![]
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

    pub fn set_labels(mut self, x_labels: Vec<&'a str>, y_labels: Vec<&'a str>) -> Self {
        self.x_labels = x_labels;
        self.y_labels = y_labels;

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
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        let font = Font::try_from_bytes(font_data).unwrap();
        let scale = Scale::uniform(25.0);

        // write x-axis text
        let text_color = Rgb([0u8, 0u8, 0u8]);
        // first step is to find the center of the x-axis where the text should be placed
        let (axis_x_text_size_x, _) = drawing::text_size(scale, &font, &self.x_axis_text);
        let x_axis_center = (500 - axis_x_text_size_x) / 2;
        drawing::draw_text_mut(&mut canvas, text_color, x_axis_center, 450, scale, &font, &self.x_axis_text);

        // write y-axis text
        // this is much more complex
        // first we need to make a temporary image and write the text on
        // first, lets calculate the text size so we can make an image that size
        let (axis_y_text_size_x, axis_y_text_size_y) = drawing::text_size(scale, &font, &self.y_axis_text);
        let mut temporary_image = RgbImage::new(axis_y_text_size_x as u32, axis_y_text_size_y as u32);
        temporary_image.fill(255); // fill with 255 to make it white
        drawing::draw_text_mut(&mut temporary_image, text_color, 0, 0, scale, &font, &self.y_axis_text);

        // now that we've created that temporary image
        // we can rotate it and paste it onto the original canvas
        let rotated_temp_image = imageops::rotate270(&temporary_image);
        // paste it onto the original canvas
        let y_axis_center = (500 - axis_y_text_size_x) / 2;
        imageops::overlay(&mut canvas, &rotated_temp_image, 0, y_axis_center as i64);

        // draw y-labels
        // first, let's find the maximum amount of pixels we can allocate for each label
        // we know that the line length is 400, so logically, best way to find maximum amount of pixels
        // we can allocate is to calculate 400 / n where n is the number of y labels we have
        let label_scale = Scale::uniform(12.5);
        let max_y_pixels = (400 / &self.y_labels.len()) as f32;
        let y_end_offset = (y_end.0, y_end.1 - 5.0);
        let mut focused_loc = y_end; // we are starting at the bottom of the y-line

        // lets iterate through the y labels and draw them on now
        // focused_loc is the location we are currently looking at on the graph
        drawing::draw_cross_mut(&mut canvas, text_color, y_end.0 as i32, y_end.1 as i32);
        for label in self.y_labels {
            let (focused_loc_x, focused_loc_y) = focused_loc;
            // first we'll draw a line indicating the real position of the number
            // tick size - 5
            let tick_start = (focused_loc_x, focused_loc_y - 5.0);
            let tick_end = (focused_loc_x - 5.0, focused_loc_y - 5.0);
            drawing::draw_line_segment_mut(&mut canvas, tick_start, tick_end, text_color);
            // next let's calculate how big the text will be
            let (text_size_x, text_size_y) = drawing::text_size(label_scale, &font, label); // we'll use this data
            // to calculate an offset so that the label is displayed clearly
            let (text_loc_x, text_loc_y) = (focused_loc_x - ((text_size_x * 2) as f32), focused_loc_y - text_size_y as f32);
            // we offset the y component by the text's height (text_size_y) to make sure that the bottom of the text is at the bottom
            // of the line so that its flush with the line
            // let's now draw the text on
            drawing::draw_text_mut(&mut canvas, text_color, text_loc_x as i32, text_loc_y as i32, label_scale, &font, label);
            // now that the text has been drawn, we need to change focused_loc to be the new text's location
            // disregarding offset, since that's calculated every iteration since it can change
            focused_loc = (focused_loc_x, focused_loc_y - max_y_pixels); // we calculate the new location by keeping the x-axis
            // the same, since that's not changing, but we subtract from the y-axis by the maximum pixel amount that we previously
            // calculated
        }
        
        // save image
        canvas.save(path).unwrap();
    }
}