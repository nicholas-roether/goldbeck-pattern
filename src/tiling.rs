use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
	pub x: f32,
	pub y: f32
}

impl Vertex {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

#[derive(Debug, Clone)]
pub struct Shape {
	pub id: Uuid,
	pub vertices: Vec<Vertex>
}

impl Shape {
	pub fn new(vertices: Vec<Vertex>) -> Self {
		Self {
			id: Uuid::new_v4(),
			vertices
		}
	}

	pub fn svg_path(&self) -> String {
		let mut path = String::new();
		for vertex in &self.vertices {
			if !path.is_empty() {
				path.push(' ');
			}
			path.push_str(&format!("{},{}", vertex.x, vertex.y))
		}
		path
	}
}

#[derive(Debug, Clone)]
pub struct Tiling {
	pub tiles: HashMap<Uuid, Shape>
}

const PATTERN_PERIOD: usize = 5;

impl Tiling {
	pub fn new(divisions: usize) -> Self {
		let mut tiles = HashMap::new();
		if divisions == 0 {
			return Self { tiles };
		}
		let square_size = 1.0 / divisions as f32;
		let pattern_unit = square_size / PATTERN_PERIOD as f32;

		for y in 0..divisions {
			let sy = y as f32 / divisions as f32;
			let pattern_index = (divisions - y - 1) % PATTERN_PERIOD;
			let offset_top = (pattern_index + 1) as f32 * pattern_unit;
			let offset_bottom = pattern_index as f32 * pattern_unit;

			for x in 0..divisions {
				let sx = x as f32 / divisions as f32;
				tiles.insert(
					Uuid::new_v4(),
					Shape::new(vec![
						Vertex::new(sx, sy),
						Vertex::new(sx + offset_top, sy),
						Vertex::new(sx + offset_bottom, sy + square_size),
						Vertex::new(sx, sy + square_size),
					])
				);
				tiles.insert(
					Uuid::new_v4(),
					Shape::new(vec![
						Vertex::new(sx + offset_top, sy),
						Vertex::new(sx + square_size, sy),
						Vertex::new(sx + square_size, sy + square_size),
						Vertex::new(sx + offset_bottom, sy + square_size),
					])
				);
			}
		}

		Self { tiles }
	}
}
