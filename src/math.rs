use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vector {
	pub fn new(x: f64, y: f64, z: f64) -> Vector {
		Vector {x: x, y: y, z: z}
	}
	
	pub fn sum(self) -> f64 {
		self.x + self.y + self.z
	}
	
	pub fn emul(self, v: Vector) -> Vector {
		Vector {
			x: self.x * v.x,
			y: self.y * v.y,
			z: self.z * v.z,
		}
	}
	
	pub fn smul(self, v: Vector) -> f64 {
		self.emul(v).sum()
	}
	
	pub fn len2(self) -> f64 {
		self.smul(self)
	}
	
	pub fn len(self) -> f64 {
		self.len2().sqrt()
	}
	
	pub fn cos(self, v: Vector) -> f64 {
		self.smul(v) / (self.len() * v.len())
	}
}

impl Add for Vector {
	type Output = Vector;
	
	fn add(self, v: Vector) -> Vector {
		Vector {
			x: self.x + v.x,
			y: self.y + v.y,
			z: self.z + v.z,
		}
	}
}

impl Sub for Vector {
	type Output = Vector;
	
	fn sub(self, v: Vector) -> Vector {
		Vector {
			x: self.x - v.x,
			y: self.y - v.y,
			z: self.z - v.z,
		}
	}
}

impl Mul for Vector {
	type Output = Vector;
	
	fn mul(self, v: Vector) -> Vector {
		Vector {
			x: self.y * v.z - self.z * v.y,
			y: self.z * v.x - self.x * v.z,
			z: self.x * v.y - self.y * v.x,
		}
	}
}

impl Mul<f64> for Vector {
	type Output = Vector;
	
	fn mul(self, k: f64) -> Vector {
		Vector {
			x: k * self.x,
			y: k * self.y,
			z: k * self.z,
		}
	}
}

pub trait Tracer {
	fn trace(&self, src: Vector, ray: Vector) -> Option<Vector>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
	pub center: Vector,
	pub radius: f64,
}

impl Sphere {
	pub fn new(center: Vector, radius: f64) -> Sphere {
		Sphere {
			center: center,
			radius: radius,
		}
	}
}

impl Tracer for Sphere {
	fn trace(&self, src: Vector, ray: Vector) -> Option<Vector> {
		// Connects src with the center of self.
		let a = self.center - src;
		
		let k1 = ray.len2();
		let k2 = a.smul(ray);
		let k3 = a.len2();
		
		let d = 4.*k2*k2 + 4.*k1*(self.radius*self.radius - k3);
		
		if d < 0. {
			None
		} else {
			let k = (2.*k2 - d.sqrt()) / (2.*k1);
			Some(ray * k)
		}
	}
}

#[cfg(test)]
mod tests {
	use std::f64;
	use super::*;
	
	#[test]
	fn sphere_trace1() {
		let src = Vector::new(0., 0., 0.);
		let sphere = Sphere::new(Vector::new(1., 1., 0.), 1.);

		let v = sphere.trace(src, Vector::new(0.1, 0.1, 0.)).unwrap();

		let xy = 1. - 1. / 2.0f64.sqrt();

		assert!((v.x - xy).abs() < f64::EPSILON);
		assert!((v.y - xy).abs() < f64::EPSILON);
	}
}
