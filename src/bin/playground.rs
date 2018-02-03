extern crate sdl2;
extern crate rustycg;

use std::thread;
use std::time::Duration;
use std::f64;

// use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rustycg::math::*;

fn main() {
	//
	//  Canvas setup.
	//
	
	let sdl = sdl2::init().unwrap();
	let video = sdl.video().unwrap();
	
	let (w, h) = (600, 600);
	
	let win = video.window("rustycg", w, h)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
		
	let mut canvas = win.into_canvas().build().unwrap();
	
	//
	//  Actual drawing.
	//
	
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	
	let origin = Vector::new(0., 0., 0.);
	let base_ray = Vector::new(-0.25, 0.25, 1.);
	
	let dw = -2. * base_ray.x / (w as f64);
	let dh = 2. * base_ray.y / (h as f64);
	
	let tracers: &[&Tracer] = &[
		&Sphere::new(Vector::new(0., -1., 5.), 1.),
		&Sphere::new(Vector::new(0., 1., 5.), 1.),
		&Sphere::new(Vector::new(0., 0., 5.), 1.),
	];
	
	for y in 0..h {
		for x in 0..w {
			let ray = base_ray + 
				Vector::new(dw * (x as f64), -dh * (y as f64), 0.);
			
			tracers.iter()
				.filter_map(|tracer| tracer.trace(origin, ray))
				.min_by(|v, u| v.len2().partial_cmp(&u.len2()).unwrap())
				.map(|v| {
					let intensity = 1.08f64.powf(-v.len2());
					
					canvas.set_draw_color(
						Color::RGB(0, (intensity * 255.) as u8, 0));
						
					let _ = canvas.draw_point((x as i32, y as i32));
				});
		}
	}
	
	canvas.present();
	
	//
	//  Event loop.
	//
	
	let mut events = sdl.event_pump().unwrap();
	
	'mainloop: loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown {keycode: Some(Keycode::Escape), ..} =>
					break 'mainloop,
					
				_ => {}
			}
		}
		
		thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
