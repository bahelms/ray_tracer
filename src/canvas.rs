use crate::tuple::Color;

const MAX_PPM_VALUE: i32 = 255;
const PPM_LINE_SIZE: i32 = 70;

pub struct Canvas {
    width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let capacity = width * height;
        let mut pixels = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            pixels.push(Color::black());
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, pixel: Color) {
        let idx = self.point_to_index(x, y);
        if idx < self.pixels.len() {
            self.pixels[idx] = pixel;
        }
    }

    fn pixel_at(&self, x: i32, y: i32) -> Option<&Color> {
        let idx = self.point_to_index(x, y);
        self.pixels.get(idx)
    }

    fn point_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    // Color values are scaled bewteen 0 and 255: 0:0-1:255
    // This algorithm runs pretty slow.
    // At 500x300 canvas: "cargo run  7.40s user 4.33s system 99% cpu 11.822 total"
    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n{}\n", self.width, self.height, MAX_PPM_VALUE);

        let mut char_count = 0;
        for color in &self.pixels {
            for value in color.iter() {
                let value = scale_value(value, MAX_PPM_VALUE).to_string();
                let value_length = value.len() as i32;
                let next_char_count = char_count + value_length + 1; // for the space
                if next_char_count > PPM_LINE_SIZE {
                    ppm.pop(); // remove previous space
                    ppm = format!("{}\n{} ", ppm, value);
                    char_count = value_length;
                } else {
                    ppm = format!("{}{} ", ppm, value);
                    char_count = next_char_count;
                }
            }
        }
        ppm.pop();
        ppm.push('\n');
        ppm
    }
}

fn scale_value(value: f64, max: i32) -> i32 {
    let total_values = (max + 1) as f64; // include 0 (0..=max is max+1 values)
    let scaled = (value * total_values) as i32;
    scaled.clamp(0, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Color;

    #[test]
    fn scale_value_clamps_values_bewteen_zero_and_max() {
        assert_eq!(scale_value(0.5, 255), 128);
        assert_eq!(scale_value(1.5, 255), 255);
        assert_eq!(scale_value(-0.1, 255), 0);
        assert_eq!(scale_value(0.9, 255), 230);
        assert_eq!(scale_value(0.1, 255), 25);
        assert_eq!(scale_value(0.4, 255), 102);
    }

    #[test]
    fn canvas_to_ppm_lines_do_not_exceed_70_chars() {
        let mut canvas = Canvas::new(10, 2);
        for y in 0..2 {
            for x in 0..10 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = canvas.to_ppm();
        let expected_ppm = "P3\n\
            10 2\n\
            255\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255\n\
            204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153\n\
            ";
        assert_eq!(ppm, expected_ppm);
    }

    #[test]
    fn canvas_to_ppm_with_pixels() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = canvas.to_ppm();
        let expected_ppm = "P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 255\n\
            ";
        assert_eq!(ppm, expected_ppm);
    }

    #[test]
    fn canvas_to_ppm_with_no_pixels_has_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let header = &ppm.lines().collect::<Vec<&str>>()[0..3];
        assert_eq!(header, ["P3", "5 3", "255"]);
    }

    #[test]
    fn writing_a_pixel_out_of_bounds_of_canvas() {
        let mut canvas = Canvas::new(5, 10);
        canvas.write_pixel(5, 9, Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.pixel_at(1, 10), None);
    }

    #[test]
    fn writing_and_getting_a_pixel_on_the_canvas() {
        let mut canvas = Canvas::new(5, 10);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.pixel_at(2, 3).unwrap(), &Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn creating_a_new_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);

        for pixel in canvas.pixels {
            assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
        }
    }
}
