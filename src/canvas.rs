use crate::tuple::Color;

struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Self {
        let capacity = width * height;
        let mut pixels = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            pixels.push(Color::new(0.0, 0.0, 0.0));
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        let idx = self.point_to_index(x, y);
        self.pixels[idx] = color;
    }

    fn pixel_at(&self, x: i32, y: i32) -> &Color {
        let idx = self.point_to_index(x, y);
        &self.pixels[idx]
    }

    fn point_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Color;

    #[test]
    fn writing_and_getting_a_pixel_on_the_canvas() {
        let mut canvas = Canvas::new(5, 10);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(*canvas.pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
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
