use graphics::*;

use mint;

type Vec3 = na::Vector3<f32>;

/// A struct containing all the necessary info for drawing a Drawable.
///
/// This struct implements the `Default` trait, so to set only some parameter
/// you can just do:
///
/// ```rust,ignore
/// graphics::draw_ex(ctx, drawable, DrawParam{ dest: my_dest, .. Default::default()} )
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawParam {
    /// a portion of the drawable to clip, as a fraction of the whole image.
    /// Defaults to the whole image (1.0) if omitted.
    pub(crate) src: Rect,
    /// the position to draw the graphic expressed as a `Point2`.
    pub(crate) dest: Point2,
    /// orientation of the graphic in radians.
    pub(crate) rotation: f32,
    /// x/y scale factors expressed as a `Point2`.
    pub(crate) scale: Point2,
    /// specifies an offset from the center for transform operations like scale/rotation,
    /// with `0,0` meaning the origin and `1,1` meaning the opposite corner from the origin.
    /// By default these operations are done from the top-left corner, so to rotate something
    /// from the center specify `Point2::new(0.5, 0.5)` here.
    pub(crate) offset: Point2,
    /// x/y shear factors expressed as a `Point2`.
    pub(crate) shear: Point2,
    /// A color to draw the target with.
    /// Default: white.
    pub(crate) color: Color,

    /// The transform matrix for the DrawParams
    matrix: Matrix4,

    translation_matrix: Matrix4,
    offset_matrix: Matrix4,
    offset_inverse_matrix: Matrix4,
    rotation_matrix: Matrix4,
    shear_matrix: Matrix4,
    scale_matrix: Matrix4,

}

impl Default for DrawParam {
    fn default() -> Self {
        DrawParam {
            src: Rect::one(),
            dest: Point2::origin(),
            rotation: 0.0,
            scale: Point2::new(1.0, 1.0),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: WHITE,

            matrix: na::one(),
            translation_matrix: na::one(),
            offset_matrix: na::one(),
            offset_inverse_matrix: na::one(),
            rotation_matrix: na::one(),
            shear_matrix: na::one(),
            scale_matrix: na::one(),
        }
    }
}

impl DrawParam {
    /// Create a new DrawParam with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Turn the DrawParam into a model matrix, combining
    /// destination, rotation, scale, offset and shear.
    pub fn into_matrix(self) -> Matrix4 {
        // let translate = Matrix4::new_translation(&Vec3::new(self.dest.x, self.dest.y, 0.0));
        // let offset = Matrix4::new_translation(&Vec3::new(self.offset.x, self.offset.y, 0.0));
        // let offset_inverse =
        //     Matrix4::new_translation(&Vec3::new(-self.offset.x, -self.offset.y, 0.0));
        // let axis_angle = Vec3::z() * self.rotation;
        // let rotation = Matrix4::new_rotation(axis_angle);
        // let scale = Matrix4::new_nonuniform_scaling(&Vec3::new(self.scale.x, self.scale.y, 1.0));
        // let shear = Matrix4::new(
        //     1.0,
        //     self.shear.x,
        //     0.0,
        //     0.0,
        //     self.shear.y,
        //     1.0,
        //     0.0,
        //     0.0,
        //     0.0,
        //     0.0,
        //     1.0,
        //     0.0,
        //     0.0,
        //     0.0,
        //     0.0,
        //     1.0,
        // );
        // translate * offset * rotation * shear * scale * offset_inverse;

        self.translation_matrix * self.offset_matrix * self.rotation_matrix * self.shear_matrix * self.scale_matrix * self.offset_inverse_matrix
    }

    /// Set the source rect
    pub fn src(mut self, src: Rect) -> Self {
        self.src = src;
        self
    }


    /// Set the dest point
    pub fn dest<T>(mut self, dest: T) -> Self where T: Into<mint::Point2<f32>> {
        let p: mint::Point2<f32> = dest.into();
        // // BUGGO: Should be able to just do Point2::from(),
        // // see https://github.com/sebcrozet/nalgebra/issues/352
        // self.dest = Point2::new(p.x, p.y);
        self.translation_matrix = Matrix4::new_translation(&Vec3::new(p.x, p.y, 0.0));
        self
    }

    /// TODO
    pub fn color<T>(mut self, color: T) -> Self where T: Into<Color> {
        self.color = color.into();
        self
    }

    /// TODO
    pub fn rotation(mut self, rotation: f32) -> Self {
        let axis_angle = Vec3::z() * rotation;
        self.rotation_matrix = Matrix4::new_rotation(axis_angle);
        // self.rotation = rotation;
        self
    }

    /// TODO
    pub fn scale<T>(mut self, scale: T) -> Self where T: Into<mint::Point2<f32>> {
        let p: mint::Point2<f32> = scale.into();
        // BUGGO
        // self.scale = Point2::new(p.x, p.y);
        self.scale_matrix = Matrix4::new_nonuniform_scaling(&Vec3::new(p.x, p.y, 1.0));
        self
    }

    /// TODO
    pub fn offset<T>(mut self, offset: T) -> Self where T: Into<mint::Point2<f32>> {
        let p: mint::Point2<f32> = offset.into();
        // BUGGO

        self.offset_matrix = Matrix4::new_translation(&Vec3::new(p.x, p.y, 0.0));
        self.offset_inverse_matrix =
            Matrix4::new_translation(&Vec3::new(-p.x, -p.y, 0.0));
        // self.offset = Point2::new(p.x, p.y);
        self
    }

    /// TODO
    pub fn shear<T>(mut self, shear: T) -> Self where T: Into<mint::Point2<f32>> {
        let p: mint::Point2<f32> = shear.into();
        // BUGGO
        // self.shear = Point2::new(p.x, p.y);
        self.shear_matrix = Matrix4::new(
            1.0,
            p.x,
            0.0,
            0.0,
            p.y,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        self
    }

    /// Set the full transform matrix for the `DrawParam`, replacing 
    /// anything already there.
    pub fn matrix<T>(mut self, matrix: T) -> Self where T: Into<mint::ColumnMatrix4<f32>> {
        let m: mint::ColumnMatrix4<f32> = matrix.into();
        self.matrix = Matrix4::from(m);
        self
    }
}
