use std::env;
use serde_derive::Deserialize;
use std::collections::{HashMap};

// SFML stuff
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, IntRect, Texture};
use sfml::window::{Event, Key, Style};
use sfml::system::SfBox;
use sfml::graphics::Transformable;

#[derive(Debug, Deserialize)]
struct Point {
	x: f32,
	y: f32
}

#[derive(Debug, Deserialize)]
struct TextureInfo {
	path: String,
	t: i32,
	b: i32,
	l: i32,
	r: i32
}

#[derive(Debug, Deserialize)]
struct LevelInfo {
	name: String,
	textures: HashMap<String, TextureInfo>,
	render_targets: HashMap<String, Vec<Point>>
}

fn main() {
	let args : Vec<String> = env::args().collect();
	if args.len() == 1 {
		println!("{}", "No level file informed");
		return;
	}

	let file_content = match std::fs::read_to_string(&args[1]) {
		Ok(c) => c,
		Err(e) => {
			println!("Failed to open file {}. {}", &args[1], e);
			return;
		}
	};

	let level: LevelInfo = match toml::from_str(&file_content) {
		Ok(l) => l,
		Err(e) => {
			println!("Failed to parse {}. {}", &args[1], e);
			return;
		}
	};

	let mut texture_vec : HashMap<String, SfBox<Texture>> = HashMap::new();
	let mut sprites_vec : HashMap<String, Sprite> = HashMap::new();
	
	for value in &level.render_targets {
		if !&level.textures.contains_key(value.0) {
			println!("Render target references an unknow texture: {}", value.0);
			return;
		}

		let texture_path = &level.textures.get(value.0).unwrap().path;
		if !texture_vec.contains_key(value.0) {
			// Already loaded
		
			let loaded_texture = Texture::from_file(texture_path);
			if loaded_texture.is_none() {
				//println!("Failed to open texture {}", &value.tid); SFML print for us
				return;
			}
	
			&texture_vec.insert(value.0.to_string(), loaded_texture.unwrap());
		}
	}

	for texture in &texture_vec {
		let mut sprite = Sprite::new();
		sprite.set_texture(&texture.1, true);

		let texture_info = &level.textures.get(texture.0).unwrap();
		sprite.set_texture_rect(&IntRect::new(texture_info.l, texture_info.t, texture_info.r, texture_info.b));
		sprites_vec.insert((&texture.0).to_string(), sprite);
	}

	let mut window = RenderWindow::new(
		(800, 600),
		"RPG Game",
		Style::CLOSE | Style::RESIZE,
		&Default::default()
	);

	loop {
		while let Some(event) = window.poll_event() {
			match event {
				Event::Closed => return,
				_ => {}
			}
		}

		window.clear(Color::BLACK);

		for render_target in &level.render_targets {
			let sprite = sprites_vec.get_mut(render_target.0).unwrap();

			for point in render_target.1 {
				sprite.set_position((point.x, point.y));
				window.draw(sprite);
			}
		}
		
		window.display();
	}
}

