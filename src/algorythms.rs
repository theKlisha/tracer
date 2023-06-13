use crate::math::{Point3, Vector3};
use rand::Rng;

pub fn triangle_intersection(
    origin: &Point3,
    direction: &Vector3,
    v0: &Point3,
    v1: &Point3,
    v2: &Point3,
) -> Option<f64> {
    const EPSILON: f64 = 1e-5;

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
    origin: &Point3,
    direction: &Vector3,
    center: &Point3,
    radius: f64,
) -> Option<(f64, f64)> {
    let oc = origin - center;
    let a = direction.magnitude_squared();
    let half_b = oc.dot(&direction);
    let c = oc.magnitude_squared() - radius * radius;
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant > 0.0 {
        let p = half_b / a;
        let q = discriminant.sqrt() / a;
        let t0 = -p - q;
        let t1 = -p + q;
        Some((t0, t1))
    } else {
        None
    }
}

pub fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3 {
    random_in_unit_sphere().normalize()
}
