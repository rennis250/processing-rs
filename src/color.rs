use glium;
use glium::Surface;

use Screen;

use shapes::ShapeVertex;

impl<'a> Screen<'a> {
    #[inline]
    pub fn background(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let framebuffer = &mut self.FBO;
        framebuffer.clear_color_srgb(r, g, b, a);
    }

    #[inline]
    pub fn colorMode(&mut self, mode: &str) {
        self.cMode = mode.to_owned();
    }

    #[inline]
    pub fn fill(&mut self, r: &[f32], g: &[f32], b: &[f32], a: &[f32]) {
        if self.fillStuff == false {
            self.fillStuff = true;
        }
        if self.cMode == "RGB" {
            self.fillCol = vec![];
            for x in 0..r.len() {
                // self.fillCol.push(RGB(r[x], g[x], b[x]);
                self.fillCol.push(r[x]);
                self.fillCol.push(g[x]);
                self.fillCol.push(b[x]);
                self.fillCol.push(a[x]);
            }
        } else {
        }
    }

    #[inline]
    pub fn fill_off(&mut self) {
        self.fillStuff = false;
    }

    #[inline]
    pub fn stroke_off(&mut self) {
        self.strokeStuff = false;
    }

    #[inline]
    pub fn fill_on(&mut self) {
        self.fillStuff = true;
    }

    #[inline]
    pub fn stroke_on(&mut self) {
        self.strokeStuff = true;
    }

    #[inline]
    pub fn stroke(&mut self, r: &[f32], g: &[f32], b: &[f32], a: &[f32]) {
        if self.strokeStuff == false {
            self.strokeStuff = true;
        }
        if self.cMode == "RGB" {
            self.strokeCol = vec![];
            for x in 0..r.len() {
                // self.strokeCol.push(RGB(r[x], g[x], b[x]);
                self.strokeCol.push(r[x]);
                self.strokeCol.push(g[x]);
                self.strokeCol.push(b[x]);
                self.strokeCol.push(a[x]);
            }
        } else {
        }
    }

    // Creating & Reading

    // #[inline]
    // pub fn alpha(c: Color) {
    //     c.a
    // }
    //
    // #[inline]
    // pub fn blue(c: Color) {
    //     c.b
    // }
    //
    // #[inline]
    // pub fn brightness(c: Color) {
    //     hsv = convert(HSV, c);
    //     hsv.v
    // }
    //
    // #[inline]
    // pub fn color(r: f32, g: f32, b: f32) {
    //     RGB(r, g, b)
    // }
    //
    // #[inline]
    // pub fn green(c: Color) {
    //     c.g;
    // }
    //
    // #[inline]
    // pub fn hue(c: Color) {
    //     hsv = convert(HSV, c);
    //     hsv.h
    // }
    //
    // #[inline]
    // pub fn lerpColor(c1: Color, c2: Color, amt: f64) {
    //     weighted_color_mean(amt, c1, c2);
    // }
    //
    // #[inline]
    // pub fn red(c: Color) {
    //     c.r
    // }
    //
    // #[inline]
    // pub fn saturation(c: Color) {
    //     hsv = convert(HSV, c);
    //     hsv.s
    // }
}
