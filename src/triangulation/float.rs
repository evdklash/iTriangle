use i_overlay::core::fill_rule::FillRule;
use i_overlay::i_float::adapter::PointAdapter;
use i_overlay::i_float::f64_point::F64Point;
use i_overlay::i_float::f64_rect::F64Rect;
use i_overlay::i_shape::f64::adapter::{ShapeToFloat, ShapeToInt};
use i_overlay::i_shape::f64::rect::RectInit;
use i_overlay::i_shape::f64::shape::{F64Path, F64Shape};
use crate::triangulation::int::IntTriangulate;

#[derive(Debug)]
pub struct Triangulation {
    pub points: Vec<F64Point>,
    pub indices: Vec<usize>,
}

pub trait FloatTriangulate {
    fn to_triangulation(&self, validate_rule: Option<FillRule>, min_area: f64) -> Triangulation;

    fn to_convex_polygons(&self, validate_rule: Option<FillRule>, min_area: f64) -> Vec<F64Path>;
}

impl FloatTriangulate for F64Shape {
    fn to_triangulation(&self, validate_rule: Option<FillRule>, min_area: f64) -> Triangulation {
        let rect = if let Some(rect) = F64Rect::with_shape(self){
            rect
        } else {
            return Triangulation { points: vec![], indices: vec![] };
        };

        let adapter = PointAdapter::new(rect);
        let shape = self.to_int(&adapter);
        let sqr_scale = adapter.dir_scale * adapter.dir_scale;
        let int_min_area = (sqr_scale * min_area) as i64;

        let triangulation = shape.to_triangulation(validate_rule, int_min_area);

        let points = triangulation.points.iter().map(|p| adapter.convert_to_float(p)).collect();

        Triangulation { points, indices: triangulation.indices }
    }

    fn to_convex_polygons(&self, validate_rule: Option<FillRule>, min_area: f64) -> Vec<F64Path> {
        let rect = if let Some(rect) = F64Rect::with_shape(self){
            rect
        } else {
            return vec![];
        };

        let adapter = PointAdapter::new(rect);
        let shape = self.to_int(&adapter);
        let sqr_scale = adapter.dir_scale * adapter.dir_scale;
        let int_min_area = (sqr_scale * min_area) as i64;

        let polygons = shape.to_convex_polygons(validate_rule, int_min_area);

        polygons.to_float(&adapter)
    }
}