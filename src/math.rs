use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
	x: f64,
	y: f64,
	z: f64,
}

impl Vector {
	pub fn new(x: f64, y: f64, z: f64) -> Vector {
		Vector {x: x, y: y, z: z}
	}
	
	pub fn sum(self) -> f64 {
		self.x + self.y + self.z
	}
	
	pub fn smul(self, v: Vector) -> f64 {
		(self * v).sum()
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
			x: self.x * v.x,
			y: self.y * v.y,
			z: self.z * v.z,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
	center: Vector,
	radius: f64,
}

impl Sphere {
	pub fn new(center: Vector, radius: f64) -> Sphere {
		Sphere {
			center: center,
			radius: radius,
		}
	}
	
	pub fn trace(self, src: Vector, ray: Vector) -> Option<f64> {
		// Connects src with the center of self.
		let a = self.center - src;
		
		let k1 = ray.len2();
		let k2 = a.smul(ray);
		let k3 = a.len2();
		
		let d = 4.*k2*k2 + 4.*k1*(self.radius*self.radius - k3);
		
		// println!("{}", d);
		
		if d < 0. {
			None
		} else {
			Some(2.*k2 - d.sqrt() / (2.*k1))
		}
	}
	
	pub fn center(self) -> Vector {
		self.center
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
		let t = sphere.trace(src, Vector::new(0.1, 0.1, 0.)).unwrap();
		assert!(t - 10. + 10./2.0f64.sqrt() < f64::EPSILON);
	}
}
