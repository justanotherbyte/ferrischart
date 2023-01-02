//! Scatter Graph implementation

use std::collections::HashMap;

use image::{RgbImage, Rgb, imageops};
use imageproc::drawing;
use rusttype::{Font, Scale};

use crate::error::ChartResult;
use super::random_rgb;

/// A struct that aids in building scatter graphs.
pub struct ScatterGraph<'a> {
    title: &'a str,
    x_axis_text: &'a str,
    y_axis_text: &'a str,
    x_labels: Vec<f32>,
    y_labels: Vec<f32>,
    data: Vec<(f32, f32)>
}

impl<'a> Default for ScatterGraph<'a> {
    fn default() -> Self {
        Self {
            title: "unset",
            x_axis_text: "unset",
            y_axis_text: "unset",
            x_labels: vec![],
            y_labels: vec![],
            data: vec![]
        }
    }
}

impl<'a> ScatterGraph<'a> {
    /// Begin the process of building a scatter graph.
    /// Initially returns the default value.
    pub fn build() -> Self {
        Self::default()
    }

    /// Loads data and generates labels based on the data given.
    pub fn load_data<T>(mut self, data: Vec<(T, T)>) -> Self 
    where
        T: Into<f32>,
    {
        if data.is_empty() {
            panic!("data cannot be empty");
        }

        let mut transformed = vec![];
        for (x, y) in data {
            transformed.push((x.into(), y.into()))
        }
        // now that we have this data
        // we want to generate labels for it
        // first we calculate the lowest label and highest label
        // by rounding the lowest value down and highest value up
        let mut x_labels = vec![];
        let mut y_labels = vec![];
        for (x, y) in &transformed {
            x_labels.push(x.to_owned());
            y_labels.push(y.to_owned());
        } // shifting them to their own vectors

        x_labels.sort_by(f32::total_cmp);
        y_labels.sort_by(f32::total_cmp);

        let (lowest_x, highest_x) = (x_labels[0].floor(), x_labels[x_labels.len() - 1].ceil());
        let (lowest_y, highest_y) = (y_labels[0].floor(), y_labels[y_labels.len() - 1].ceil());

        // clear the vectors so that we can now
        // allocate the new labels
        x_labels.clear();
        y_labels.clear();

        // since Step isn't implemented for f32
        // we're doing this old-school
        let mut current = lowest_x;
        while current <= highest_x {
            x_labels.push(current);
            current += 1.0;
        }

        current = lowest_y;
        while current <= highest_y {
            y_labels.push(current);
            current += 1.0;
        }
        
        self.data = transformed;
        self.x_labels = x_labels;
        self.y_labels = y_labels;

        self
    }

    /// Sets a title for the graph.
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.title = title;

        self
    }

    /// Set some text indicating what each respective axis represents.
    pub fn set_axis_text(mut self, x_axis_text: &'a str, y_axis_text: &'a str) -> Self {
        self.x_axis_text = x_axis_text;
        self.y_axis_text = y_axis_text;

        self
    }

    /// Sets the labels for each axis
    /// For example, `.set_labels(vec![1.0, 2.0, 4.0, 5.0])` will draw
    /// 1.0 2.0 4.0 5.0 along the x-axis.
    /// Only use this method if you know what you're doing since the [`load_data`] method already attempts
    /// to do this for you.
    /// 
    /// [`load_data`]: #method.load_data
    pub fn set_labels(mut self, x_labels: Vec<f32>, y_labels: Vec<f32>) -> Self {
        self.x_labels = x_labels;
        self.y_labels = y_labels;

        self
    }

    /// Draws the graph and saves to a file path.
    pub fn draw(self, path: &str) -> ChartResult<()> {
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
        let (axis_x_text_size_x, axis_x_text_size_y) = drawing::text_size(scale, &font, &self.x_axis_text);
        let x_axis_center = (500 - axis_x_text_size_x) / 2;
        let x_axis_text_y = 500 - axis_x_text_size_y;
        drawing::draw_text_mut(&mut canvas, text_color, x_axis_center, x_axis_text_y, scale, &font, &self.x_axis_text);

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
        let mut focused_loc = y_end; // we are starting at the bottom of the y-line

        let tick_size: f32 = 5.0;

        let mut y_tick_locations: HashMap<String, f32> = HashMap::new(); // holds mappings for "y label tick: at n y-component"
        let mut x_tick_locations: HashMap<String, f32> = HashMap::new(); // holds mappings for "x label tick: at n x-component"

        // lets iterate through the y labels and draw them on now
        // focused_loc is the location we are currently looking at on the graph
        for label in self.y_labels {
            let label_string = label.to_string();
            let (focused_loc_x, focused_loc_y) = focused_loc;
            // first we'll draw a line indicating the real position of the number
            // tick size - 5
            /*
            1) We state that the position where the tick_starts is directly in the middle
            of the pixels we've allocated it. So say we allocate it a space that is 10 pixels high,
            it needs to start around the 5 pixel mark, thus we calculate the middle:
            max_y_pixels / 2
            2) The x position will be the same, since it'll be starting on the line
            3) The x position needs to change on the y-axis however when drawing the tick, since the tick
            extends on the x-axis, so we subtract 5.0 from the current position: focused_loc_x - 5.0
            5.0 being our tick size
            */
            let mid = max_y_pixels / 2f32;
            let tick_start = (focused_loc_x, focused_loc_y - mid);
            let tick_end = (focused_loc_x - tick_size, focused_loc_y - mid);
            drawing::draw_line_segment_mut(&mut canvas, tick_start, tick_end, text_color);

            y_tick_locations.insert(label_string.clone(), focused_loc_y - mid);
            
            /*
            Drawing on the text will be slightly different.
            1) First step is to calculate the position where the tick ends, so we can make sure that
            our text begins there (that's already been done above)
            2) Since the co-ordinate system considers (0,0) as the top-left, the text's top-left pixel will be set
            as the point we calculate
            3) We want the middle of our label to be aligned with the tick, so we need to perform the following:
                - calculate the text's height (since we're dealing with the y-axis here)
                - divide this height by 2, to calculate a rough center
                - offset the y-component of the text by this value
            4) Running through step 3 makes sure that the center of the label is actually aligned with the tick,
            however, it'll still be overlapping. To fix this, we'll decrease its x-component by the text-width
            */
            let (text_width, text_height) = drawing::text_size(label_scale, &font, &label_string);
            let rough_center = text_height / 2;
            let (text_location_x, text_location_y) = ((tick_end.0 - text_width as f32), (tick_end.1 - rough_center as f32));

            drawing::draw_text_mut(
                &mut canvas,
                text_color,
                text_location_x as i32,
                text_location_y as i32,
                label_scale,
                &font,
                &label_string
            );

            // Now we just increment focused_loc to the new position we want to focus on
            // we can do that by decrementing its y-component by the max_y_pixels we previously calculated
            // Note: decreasing the y-component actually places the position higher since (0,0) is the top-left
            focused_loc = (focused_loc_x, focused_loc_y - max_y_pixels);
        }

        // now let's iterate through the x labels and do the same
        // again, focused_loc is the location we are currently looking at on the graph
        // we'll reset this back to y_end
        // y_end is the bottom of the y-axis line, and also the beginning of the x-axis line
        focused_loc = y_end;

        // again we need to calculate the maximum amount of pixels we can allocate
        // for each x label
        let max_x_pixels = (400 / &self.x_labels.len()) as f32;

        for label in self.x_labels {
            let label_string = label.to_string();
            let (focused_loc_x, focused_loc_y) = focused_loc;
            // again first, we need to draw on a tick
            /*
            1) Same as before, we state that the beginning of the tick (tick_start) is directly
            in the middle of the pixels we've allocated it
            2) To calculate this, we again divide the max_x_pixels value by 2 to calculate an offset value
            3) This time, our y-component will stay the same, since we're only dealing with the x-axis here,
            and the ticks y-component (at least the position where it starts) will stay the same. Only time we change
            its y-component is when we alter the position so we can draw the tick
            */
            let mid = max_x_pixels / 2f32;
            let tick_start = ((focused_loc_x + mid), focused_loc_y);
            let tick_end = ((focused_loc_x + mid), focused_loc_y + tick_size); // increment y-value
            // by tick_size since that's the height of our tick
            drawing::draw_line_segment_mut(
                &mut canvas,
                tick_start,
                tick_end,
                text_color
            );
            x_tick_locations.insert(label_string.clone(), focused_loc_x + mid);

            /*
            Drawing text on is slightly different from how we drew on our y-axis text
            1) First step (similarly to before) is to place our text's top-left pixel at the position where the
            tick ends
            2) However, this makes it so that the text renders just to the right of the tick
            3) If we want to make the text render directly in the middle of it, we need to have half of the pixels
            to the left of the tick, and half of the pixels to the right of the tick.
            4) An easy way to do this is to calculate the text width, and divide by 2 to get an offset value
            5) Offset its x-component by decreasing it by this offset value
            */
            let (text_width, _) = drawing::text_size(label_scale, &font, label_string.as_str());
            let offset_value = text_width / 2;
            let (text_location_x, text_location_y) = tick_end;
            let text_location_x = text_location_x - offset_value as f32;
            drawing::draw_text_mut(
                &mut canvas,
                text_color,
                text_location_x as i32,
                text_location_y as i32,
                label_scale,
                &font,
                &label_string
            );

            // increment focused_loc on its x-component by incrementing by max_x_pixels
            // Note: increasing the x-component moves it further along the graph
            focused_loc = (focused_loc_x + max_x_pixels, focused_loc_y);
        }

        // now that all of the important sections are complete, we can now
        // focus on drawing a title
        // the title will be at the same y position as where the y-axis line starts
        // we want the bottom of the title to be flush with the top of the y-axis line
        // so we just offset by the text's height

        let (title_width, title_height) = drawing::text_size(scale, &font, self.title);
        let center = (500 - title_width) / 2;
        let (title_pos_x, title_pos_y) = (center, 50 - title_height);
        drawing::draw_text_mut(
            &mut canvas,
            text_color,
            title_pos_x,
            title_pos_y,
            scale,
            &font,
            self.title
        );

        // now for the most important part
        // actually plotting positions
        let mut existing_positions = HashMap::new();
        for (x, y) in self.data {
            let x_string = x.to_string();
            let y_string = y.to_string();

            let x_pos = x_tick_locations.get(&x_string).copied().unwrap_or_else(|| {
                let closest_floor = x.floor(); // first we calculate the floor value of our data
                // example: 9.2 would become 9.0 and 8.6 would become 8.0
                let closest_floor_string = &closest_floor.to_string(); // we convert this into a string
                // so we can use this value as a key in accessing the position of this floor value's tick
                let multiplier = x - closest_floor; // we find a multiplier
                // example: a value of 9.2 would have a floor of 9.0, so the multiplier would become 0.2 (20% essentially)
                let new_x = x_tick_locations[closest_floor_string] + (max_x_pixels * multiplier);
                // we retrieve the closest floor's tick position and create a new position by offseting the tick's position
                // by max_x_pixels * multiplier which is essentially saying (if we reference the example above), since the multiplier
                // was 0.2 (or 20%), we want to get 20% of the maximum allocated pixels and add it on.
                new_x
            }) as i32;
            let y_pos = y_tick_locations.get(&y_string).copied().unwrap_or_else(|| {
                let closest_floor = y.floor();
                let closest_floor_string = &closest_floor.to_string();
                let multiplier = y - closest_floor;
                let new_y = y_tick_locations[closest_floor_string] - (max_y_pixels * multiplier);
                new_y
            }) as i32;
            

            // if the position already exists, that means its a duplicate set of data
            // so we'll increment its count by 1
            let k = (x_pos, y_pos);
            if existing_positions.contains_key(&k) {
                *existing_positions.get_mut(&k).unwrap() += 1;
            } else {
                existing_positions.insert(k, 3);
            }
        }

        for (pos, count) in existing_positions {
            let color = random_rgb();
            drawing::draw_filled_circle_mut(
                &mut canvas,
                pos,
                count,
                color
            );
        }
        
        // save image
        canvas.save(path)?;
        Ok(())
    }
}