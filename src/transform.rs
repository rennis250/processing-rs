use Screen;

use {Matrix4, Vector3, Unit};

impl<'a> Screen<'a> {
    pub fn applyMatrix(
        &mut self,
        n00: f32,
        n01: f32,
        n02: f32,
        n03: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n13: f32,
        n20: f32,
        n21: f32,
        n22: f32,
        n23: f32,
        n30: f32,
        n31: f32,
        n32: f32,
        n33: f32,
    ) {
        let m = Matrix4::new(
            n00,
            n01,
            n02,
            n03,
            n10,
            n11,
            n12,
            n13,
            n20,
            n21,
            n22,
            n23,
            n30,
            n31,
            n32,
            n33,
        );

        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn popMatrix(&mut self) {
        match self.matrices.matrixStack.pop() {
            Some(m) => self.matrices.currMatrix = m,
            None => {
                self.matrices.currMatrix = Matrix4::identity();
            }
        };
    }

    pub fn pushMatrix(&mut self) {
        self.matrices.matrixStack.push(self.matrices.currMatrix);
    }

    pub fn resetMatrix(&mut self) {
        self.matrices.currMatrix = Matrix4::identity();
    }

    pub fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        // let m = Matrix4::new(
        //     angle.cos(),
        //     -angle.sin(),
        //     0.,
        //     0.,
        //     angle.sin(),
        //     angle.cos(),
        //     0.,
        //     0.,
        //     0.,
        //     0.,
        //     1.,
        //     0.,
        //     0.,
        //     0.,
        //     0.,
        //     1.,
        // );
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(x, y, z)), angle);

        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn rotateX(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(1., 0., 0.)), angle);
        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn rotateY(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(0., 1., 0.)), angle);
        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn rotateZ(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(0., 0., 1.)), angle);
        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        // let m = Matrix4::new(x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.);

        self.matrices.currMatrix.append_nonuniform_scaling(
            &Vector3::new(x, y, z),
        ); //* self.matrices.currMatrix;
    }

    pub fn shearX(&mut self, angle: f32) {
        let m = Matrix4::new(
            1.,
            angle.tan(),
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
        );

        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn shearY(&mut self, angle: f32) {
        let m = Matrix4::new(
            1.,
            0.,
            0.,
            0.,
            angle.tan(),
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
        );

        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let m = Matrix4::new(1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.);

        self.matrices.currMatrix = m * self.matrices.currMatrix;
    }

    pub fn printMatrix(&self) {
        println!("{:?}", self.matrices.currMatrix);
    }
}
