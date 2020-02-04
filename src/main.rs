use std::env;
use serde_derive::Deserialize;
use std::collections::{HashMap};

// SFML stuff
use sfml::graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Sprite, IntRect, Texture};
use sfml::window::{Event, Style};
use sfml::system::SfBox;
use sfml::graphics::Transformable;
use sfml::graphics::Shape;
use sfml::system::Vector2;

const BOX_FRAME_SIZE : i32 = 2;

#[derive(Debug, Deserialize)]
struct Point {
	x: f32,
	y: f32
}

#[derive(Debug, Deserialize)]
struct TextureInfo<'a> {
	path: String,
	t: i32,
	l: i32,
	w: i32,
	h: i32,
	
	rigid: Option<bool>,

	// Loaded sprite
	#[serde(skip_deserializing)]
	sprite: Option<Sprite<'a>>,

	// Loaded texture
	#[serde(skip_deserializing)]
	texture: Option<SfBox<Texture>>
}

#[derive(Debug, Deserialize)]
struct LevelInfo<'a> {
	name: String,
	textures: HashMap<String, TextureInfo<'a>>,
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

	let mut texture_vec : HashMap<String, SfBox<Texture>> = HashMap::new();
	let mut level: LevelInfo = match toml::from_str(&file_content) {
		Ok(l) => l,
		Err(e) => {
			println!("Failed to parse {}. {}", &args[1], e);
			return;
		}
	};
	
	for value in &level.render_targets {
		if !&level.textures.contains_key(value.0) {
			println!("Render target references an unknow texture: {}", value.0);
			return;
		}

		let texture_info = &mut level.textures.get(value.0).unwrap();
		if !texture_vec.contains_key(value.0) {
			let loaded_texture = Texture::from_file(&texture_info.path);
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

		let texture_info = &mut level.textures.get_mut(texture.0).unwrap();
		sprite.set_texture_rect(&IntRect::new(texture_info.l, texture_info.t, texture_info.w, texture_info.h));

		texture_info.texture = Some(texture.1.clone());
		texture_info.sprite = Some(sprite);
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
			let texture_info = level.textures.get_mut(render_target.0).unwrap();
			let sprite = texture_info.sprite.as_mut().unwrap();

			for point in render_target.1 {
				let tx = point.x * texture_info.w as f32;
				let ty = point.y * texture_info.h as f32;
				sprite.set_position((tx, ty));
				window.draw(sprite);

				if texture_info.rigid.is_some() && texture_info.rigid.unwrap() {
					let mut shape = RectangleShape::with_size(Vector2::new((texture_info.w - BOX_FRAME_SIZE * 2) as f32, (texture_info.h - BOX_FRAME_SIZE * 2) as f32));
					shape.set_position(Vector2::new(tx + BOX_FRAME_SIZE as f32 / 2.0, ty + BOX_FRAME_SIZE as f32 / 2.0));
					shape.set_outline_color(Color::WHITE);
					shape.set_outline_thickness(BOX_FRAME_SIZE as f32);
					shape.set_fill_color(Color::TRANSPARENT);
					window.draw(&shape);
				}
			}
		}
		
		window.display();
	}
}
