use nalgebra::{Point3, Vector3};

pub fn moller_trumbore_intersection(
    origin: &Point3<f32>,
    direction: &Vector3<f32>,
    v0: Point3<f32>,
    v1: Point3<f32>,
    v2: Point3<f32>,
) -> Option<f32> {
    const EPSILON: f32 = 1e-5;

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = direction.cross(&edge2);
    let a = edge1.dot(&h);

    if a.abs() < EPSILON {
        // ray is parallel to the triangle.
        return None;
    }

    let f = 1.0 / a;
    let s = origin - v0;
    let u = f * s.dot(&h);

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.cross(&edge1);
    let v = f * direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // compute intersection point
    let t = f * edge2.dot(&q);

    if t < EPSILON {
        // line intersection but not ray intersection
        return None;
    }

    Some(t)
}

pub fn sphere_intersection(
    origin: &Point3<f32>,
    direction: &Vector3<f32>,
    center: &Point3<f32>,
    radius: f32,
) -> Option<(f32, f32)> {
    let oc = origin - center;
    let a = direction.magnitude_squared();
    let half_b = oc.dot(&direction);
    let c = oc.magnitude_squared() - radius * radius;
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant > 0.0 {
        let p = half_b / a;
        let q = discriminant.sqrt() / a;
        let t0 = - p - q;
        let t1 = - p + q;
        Some((t0, t1))
    } else {
        None
    }
}
