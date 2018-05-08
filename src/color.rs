use glium;
use glium::Surface;

use Screen;

use shapes::ShapeVertex;

impl<'a> Screen<'a> {
	/// Change the background color of the window. It takes four f32's. If color mode
	/// is equal to "RGB", then it takes one for red, one for green, one for blue, and
	/// one for alpha. If color mode is equal to "HSB", then the arguments are
	/// reinterpreted as one for hue, one for saturation, one for brightness, and one
	/// for alpha.
    #[inline]
    pub fn background(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let framebuffer = &mut self.FBO;
        framebuffer.clear_color_srgb(r, g, b, a);
    }

	/// Change the color mode to "RGB" or "HSB". This causes the arguments to fill(),
	/// stroke(), background(), and color() to be reinterpreted in the respective 
	/// color system.
    #[inline]
    pub fn colorMode(&mut self, mode: &str) {
        self.cMode = mode.to_owned();
    }

	/// Change the color used to fill in the space bounded by shapes. For example,
	/// setting fill to 1, 1, 1, 1 in "RGB" mode will cause the interior of a
	/// rectangle to be white. The arguments to fill are actually slices of f32's. This
	/// is meant to be a convenience when you know that you want to draw many of the
	/// same kind of shape, but each with a different stroke color. Calling this
	/// function will also undo the effect of fill_off().
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

	/// This disables filling in of shapes, such that only their outline is drawn. It
	/// essentially acts as if the interior of a shape was transparent. Calling fill()
	/// or fill_on() will re-enable filling in of shapes.
    #[inline]
    pub fn fill_off(&mut self) {
        self.fillStuff = false;
    }

	/// This disables the drawing of edges of shapes, such that only their interior is
	/// drawn. It essentially acts as if the shape was one single color, with the edge
	/// sharing that color. It's a little easier to understand with some examples
	/// (see the Processing reference). Calling stroke() or stroke_on() will re-enable
	/// the drawing of edges of shapes.
    #[inline]
    pub fn stroke_off(&mut self) {
        self.strokeStuff = false;
    }

	/// This undoes the effect of fill_off(), so that the interiors of shapes are drawn
	/// again.
    #[inline]
    pub fn fill_on(&mut self) {
        self.fillStuff = true;
    }
	
	/// This undoes the effect of stroke_off(), so that the edges of shapes are drawn
	/// again.
    #[inline]
    pub fn stroke_on(&mut self) {
        self.strokeStuff = true;
    }
	
	/// Change the color used to drawn the edges of shapes. For example,
	/// setting stroke to 1, 1, 1, 1 in "RGB" mode will cause the edge of a
	/// rectangle to be white. The arguments to stroke are actually slices of f32's.
	/// This is meant to be a convenience when you know that you want to draw many of
	/// the same kind of shape, but each with a different edge color. Calling this
	/// function will also undo the effect of stroke_off().
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
