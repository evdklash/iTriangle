use i_float::int::point::IntPoint;
use i_overlay::i_float::triangle::Triangle;
use i_overlay::i_float::u128::UInt128;
use crate::delaunay::triangle::DTriangle;
use crate::delaunay::vertex::DVertex;
use crate::index::{NIL_INDEX, Index};
use crate::triangulation::int::Triangulation;

pub struct Delaunay {
    pub triangles: Vec<DTriangle>,
}

impl Delaunay {
    pub fn to_triangulation(&self, shifted: usize) -> Triangulation {
        let mut indices = vec![NIL_INDEX; 3 * self.triangles.len()];
        let i_pnt = indices.as_mut_ptr();

        let mut max_index = 0;
        let mut j = 0;
        for triangle in self.triangles.iter() {
            let a = triangle.vertices[0];
            let b = triangle.vertices[1];
            let c = triangle.vertices[2];

            max_index = max_index.max(c.index).max(a.index.max(b.index));

            unsafe {
                *i_pnt.add(j) = a.index + shifted;
                *i_pnt.add(j + 1) = b.index + shifted;
                *i_pnt.add(j + 2) = c.index + shifted;
            }
            j += 3;
        }

        let mut points = vec![IntPoint::ZERO; max_index + 1];
        let p_pnt = points.as_mut_ptr();

        for triangle in self.triangles.iter() {
            let a = triangle.vertices[0];
            let b = triangle.vertices[1];
            let c = triangle.vertices[2];

            unsafe {
                *p_pnt.add(a.index) = a.point;
                *p_pnt.add(b.index) = b.point;
                *p_pnt.add(c.index) = c.point;
            }
        }

        Triangulation { points, indices }
    }

    pub(super) fn new(triangles: Vec<DTriangle>) -> Self {
        Self { triangles }
    }

    pub(crate) fn build(&mut self) {
        let count = self.triangles.len();
        let mut visit_marks = vec![false; count];
        // let visit_marks_ptr = visit_marks.as_mut_ptr();

        let mut visit_index = 0;

        let mut origin = Vec::with_capacity(64);
        origin.push(0);

        let mut buffer = Vec::with_capacity(64);

        while !origin.is_empty() {
            let mut j = 0;
            while j < origin.len() {
                let i = origin[j];
                j += 1;

                let mut triangle = *unsafe { self.triangles.get_unchecked(i) };
                unsafe {
                    *visit_marks.get_unchecked_mut(i) = true;
                }
                for k in 0..3 {
                    let neighbor_index = triangle.neighbor_by_order(k);
                    if neighbor_index.is_nil() {
                        continue;
                    }
                    let neighbor = *unsafe { self.triangles.get_unchecked(neighbor_index) };
                    if self.swap(triangle, neighbor) {
                        triangle = *unsafe { self.triangles.get_unchecked(triangle.index) };
                        let neighbor = unsafe { self.triangles.get_unchecked(neighbor_index) };

                        let tna = triangle.na();
                        if tna.is_not_nil() && tna != neighbor.index {
                            buffer.push(tna);
                        }

                        let tnb = triangle.nb();
                        if tnb.is_not_nil() && tnb != neighbor.index {
                            buffer.push(tnb);
                        }

                        let tnc = triangle.nc();
                        if tnc.is_not_nil() && tnc != neighbor.index {
                            buffer.push(tnc);
                        }

                        let nna = neighbor.na();
                        if nna.is_not_nil() && nna != triangle.index {
                            buffer.push(nna);
                        }

                        let nnb = neighbor.nb();
                        if nnb.is_not_nil() && nnb != triangle.index {
                            buffer.push(nnb);
                        }

                        let nnc = neighbor.nc();
                        if nnc.is_not_nil() && nnc != triangle.index {
                            buffer.push(nnc);
                        }
                    }
                }
            }

            if buffer.is_empty() && visit_index < count {
                visit_index += 1;
                while visit_index < count {
                    let is_visited = unsafe { *visit_marks.get_unchecked(visit_index) };
                    if !is_visited {
                        buffer.push(visit_index);
                        break;
                    }
                    visit_index += 1;
                }
            }

            origin.clear();

            std::mem::swap(&mut origin, &mut buffer);
        }
    }

    fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
        let pi = pbc.opposite(abc.index);

        let p = pbc.vertex_by_order(pi);

        let bi: usize;
        let ci: usize;
        let a: DVertex;  // opposite a-p
        let b: DVertex;  // edge bc
        let c: DVertex;

        let ai = abc.opposite(pbc.index);
        match ai {
            0 => {
                bi = 1;
                ci = 2;
                a = abc.va();
                b = abc.vb();
                c = abc.vc();
            }
            1 => {
                bi = 2;
                ci = 0;
                a = abc.vb();
                b = abc.vc();
                c = abc.va();
            }
            _ => {
                bi = 0;
                ci = 1;
                a = abc.vc();
                b = abc.va();
                c = abc.vb();
            }
        }

        let is_pass = Self::condition(p.point, c.point, a.point, b.point);

        return if is_pass {
            false
        } else {
            let is_abp_cw = Triangle::is_clockwise_point(a.point, b.point, p.point);

            let bp = pbc.neighbor(c.index);
            let cp = pbc.neighbor(b.index);
            let ab = abc.neighbor_by_order(ci);
            let ac = abc.neighbor_by_order(bi);

            // abc -> abp
            let abp: DTriangle;

            // pbc -> acp
            let acp: DTriangle;

            if is_abp_cw {
                abp = DTriangle::abc_bc_ac_ab(
                    abc.index,
                    a,
                    b,
                    p,
                    bp,                 // a - bp
                    pbc.index,          // p - ap
                    ab,                     // b - ab
                );

                acp = DTriangle::abc_bc_ac_ab(
                    pbc.index,
                    a,
                    p,
                    c,
                    cp,                 // a - cp
                    ac,                     // p - ac
                    abc.index,          // b - ap
                );
            } else {
                abp = DTriangle::abc_bc_ac_ab(
                    abc.index,
                    a,
                    p,
                    b,
                    bp,                 // a - bp
                    ab,                 // p - ab
                    pbc.index,          // b - ap
                );

                acp = DTriangle::abc_bc_ac_ab(
                    pbc.index,
                    a,
                    c,
                    p,
                    cp,                 // a - cp
                    abc.index,          // p - ap
                    ac,                 // b - ac
                )
            }

            // fix neighbor's link
            // ab, cp didn't change neighbor
            // bc -> ap, so no changes

            // ac (abc) is now edge of acp
            let ac_index = abc.neighbor_by_order(bi); // b - angle
            self.update_neighbor_index(ac_index, abc.index, acp.index);

            // bp (pbc) is now edge of abp
            let bp_index = pbc.neighbor(c.index); // c - angle
            self.update_neighbor_index(bp_index, pbc.index, abp.index);

            unsafe {
                *self.triangles.get_unchecked_mut(abc.index) = abp;
                *self.triangles.get_unchecked_mut(pbc.index) = acp;
            }
            true
        };
    }

    fn update_neighbor_index(&mut self, index: usize, old_neighbor: usize, new_neighbor: usize) {
        if index.is_not_nil() {
            let neighbor = unsafe {
                self.triangles.get_unchecked_mut(index)
            };
            neighbor.update_opposite(old_neighbor, new_neighbor);
        }
    }

    // if p0 is inside circumscribe circle of p1, p2, p3 return false
    // if p0 is inside circumscribe A + B > 180
    // return true if triangle satisfied condition and do not need flip triangles
    fn condition(p0: IntPoint, p1: IntPoint, p2: IntPoint, p3: IntPoint) -> bool {
        // x, y of all coordinates must be in range of i32
        // p1, p2, p3 points of current triangle
        // p0 is a test point
        // p1 and p3 common points of triangle p1, p2, p3 and p1, p0, p2
        // alpha (A) is an angle of p1, p0, p3
        // beta (B) is an angle of p1, p2, p3

        let v10 = p1.subtract(p0);
        let v30 = p3.subtract(p0);

        let v12 = p1.subtract(p2);
        let v32 = p3.subtract(p2);

        let cos_a = v10.dot_product(v30);
        let cos_b = v12.dot_product(v32);

        if cos_a < 0 && cos_b < 0 {
            // A > 90 and B > 90
            // A + B > 180
            return false;
        }

        if cos_a >= 0 && cos_b >= 0 {
            // A <= 90 and B <= 90
            // A + B <= 180
            return true;
        }

        let sn_a = v10.cross_product(v30).unsigned_abs(); // A <= 180
        let sn_b = v12.cross_product(v32).unsigned_abs(); // B <= 180

        if cos_a < 0 {
            // cosA < 0
            // cosB >= 0
            let sin_a_cos_b = UInt128::multiply(sn_a, cos_b as u64);            // positive
            let cos_a_sin_b = UInt128::multiply(cos_a.unsigned_abs(), sn_b);    // negative

            sin_a_cos_b >= cos_a_sin_b
        } else {
            // cosA >= 0
            // cosB < 0
            let sin_a_cos_b = UInt128::multiply(sn_a, cos_b.unsigned_abs());    // negative
            let cos_a_sin_b = UInt128::multiply(cos_a as u64, sn_b);            // positive

            cos_a_sin_b >= sin_a_cos_b
        }
    }
}