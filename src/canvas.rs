use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, p: Color) {
        self.pixels[y * self.width + x] = p;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y * self.width + x]
    }

    pub fn to_ppm_string(&self) -> String {
        let mut s = String::new();
        // Header
        s.push_str("P3\n");
        s.push_str(self.width.to_string().as_str());
        s.push_str(" ");
        s.push_str(self.height.to_string().as_str());
        s.push_str("\n255\n");
        // Pixel data
        for pixel in self.pixels.iter() {
            s.push_str(pixel.to_ppm_string().as_str());
            s.push_str("\n");
        }
        // Done!
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for p in c.pixels {
            assert_eq!(p, black);
        }
    }

    #[test]
    fn write_pixel() {
        let width = 10;
        let height = 20;
        let black = Color::new(0.0, 0.0, 0.0);
        let red = Color::new(1.0, 0.0, 0.0);
        let green = Color::new(0.0, 1.0, 0.0);
        let mut c = Canvas::new(width, height);
        c.write_pixel(2, 3, red.clone());
        c.write_pixel(3, 2, green.clone());
        for i in 0..(width * height) {
            let x = i / height;
            let y = i % height;
            if x == 2 && y == 3 {
                assert_eq!(*c.pixel_at(x, y), red);
            } else if x == 3 && y == 2 {
                assert_eq!(*c.pixel_at(x, y), green);
            } else {
                assert_eq!(*c.pixel_at(x, y), black);
            }
        }
    }

    #[test]
    fn to_ppm_string() {
        let width = 5;
        let height = 5;
        let red = Color::new(1.0, 0.0, 0.0);
        let green = Color::new(0.0, 1.0, 0.0);
        let mut c = Canvas::new(width, height);
        c.write_pixel(2, 3, red);
        c.write_pixel(3, 2, green);
        assert_eq!(
            c.to_ppm_string(),
            "P3\n5 5\n255\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 255 0\n0 0 0\n0 0 0\n0 0 0\n255 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n",
        );
    }
}