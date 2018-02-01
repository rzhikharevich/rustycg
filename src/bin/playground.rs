extern crate sdl2;
extern crate rustycg;

use std::thread;
use std::time::Duration;

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
	
	canvas.set_draw_color(Color::RGB(0, 255, 0));
	
	let camera_origin = Vector::new(0., 0., 0.);
	let sphere = Sphere::new(Vector::new(0., 0., 2.), 1.);
	
	let dist = sphere.center().len();
	
	let frame_w = 2.;
	let frame_h = 2.;
	
	let base_ray = Vector::new(-frame_w / 2., frame_h / 2., 1.);
	
	let dw = frame_w / (w as f64);
	let dh = -frame_h / (h as f64);
	
	for y in 0..h {
		for x in 0..w {
			let ray = base_ray +
				Vector::new(dw * (x as f64), dh * (y as f64), 0.);
			
			if let Some(k) = sphere.trace(camera_origin, ray) {
				canvas.set_draw_color(
					Color::RGB(0, (((dist - k) / dist) * 255.) as u8, 0));
				
				let _ = canvas.draw_point((x as i32, y as i32));
			}
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
