#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vertex {
	pub x: f32,
	pub y: f32
}

impl Vertex {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

#[derive(Debug, Clone)]
pub struct Shape(Vec<Vertex>);

impl Shape {
	pub fn svg_path(&self) -> String {
		let mut path = String::new();
		for vertex in &self.0 {
			if !path.is_empty() {
				path.push(' ');
			}
			path.push_str(&format!("{},{}", vertex.x, vertex.y))
		}
		path
	}
}

const PATTERN_WIDTH: usize = 2;
const PATTERN_HEIGHT: usize = 5;
const TILE_SIZE: usize = 4;
const NUM_TILES: usize = PATTERN_WIDTH * PATTERN_HEIGHT;

type Tile = [Vertex; TILE_SIZE];

const fn generate_tiles() -> [Tile; NUM_TILES] {
	let mut vertices: [Tile; NUM_TILES] = [[Vertex::new(0.0, 0.0); TILE_SIZE]; NUM_TILES];

	let mut index: usize = 0;
	let mut x = 0;
	let mut y = 0;

	while index < NUM_TILES {
		let x_f = x as f32;
		let y_f = y as f32;
		let offset_top = 1.0 - y_f / PATTERN_HEIGHT as f32;
		let offset_bottom = 1.0 - (y_f + 1.0) / PATTERN_HEIGHT as f32;

		vertices[index] = [
			Vertex::new(x_f, y_f),
			Vertex::new(x_f + offset_top, y_f),
			Vertex::new(x_f + offset_bottom, y_f + 1.0),
			Vertex::new(x_f, y_f + 1.0)
		];
		vertices[index + 1] = [
			Vertex::new(x_f + offset_top, y_f),
			Vertex::new(x_f + 1.0, y_f),
			Vertex::new(x_f + 1.0, y_f + 1.0),
			Vertex::new(x_f + offset_bottom, y_f + 1.0)
		];

		index += PATTERN_WIDTH;
		x += 1;
		if x < PATTERN_WIDTH {
			x = 0;
			y += 1;
		}
	}

	vertices
}

const PATTERN_TILES: [Tile; NUM_TILES] = generate_tiles();

const TILES_PER_SQUARE_X: usize = 2;
const TILES_PER_SQUARE_Y: usize = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct Tiling {
	width: usize,
	height: usize
}

impl Tiling {
	pub fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	pub fn view_box(&self) -> String {
		format!(
			"0 0 {} {}",
			self.width * PATTERN_WIDTH / TILES_PER_SQUARE_X,
			self.height * PATTERN_HEIGHT / TILES_PER_SQUARE_Y
		)
	}

	pub fn tile_width(&self) -> usize {
		PATTERN_WIDTH * self.width
	}

	pub fn tile_height(&self) -> usize {
		PATTERN_HEIGHT * self.height
	}

	pub fn tile(&self, x: usize, y: usize) -> Shape {
		let pattern_x = x % PATTERN_WIDTH;
		let offs_x = x / PATTERN_WIDTH;
		let pattern_y = y % PATTERN_HEIGHT;
		let offs_y = y / PATTERN_HEIGHT;

		let pattern_index = pattern_y * PATTERN_WIDTH + pattern_x;
		let base_vertices = PATTERN_TILES[pattern_index];
		let trans_vertices = base_vertices
			.iter()
			.map(|v| Vertex::new(v.x + offs_x as f32, v.y + offs_y as f32))
			.collect();

		Shape(trans_vertices)
	}

	pub fn iter_tiles(&self) -> TilesIter {
		TilesIter::new(self)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TilesIter<'a> {
	tiling: &'a Tiling,
	x: usize,
	y: usize
}

impl<'a> TilesIter<'a> {
	fn new(tiling: &'a Tiling) -> Self {
		Self { tiling, x: 0, y: 0 }
	}
}

impl<'a> Iterator for TilesIter<'a> {
	type Item = Shape;

	fn next(&mut self) -> Option<Self::Item> {
		if self.y == self.tiling.tile_height() {
			return None;
		}

		let shape = self.tiling.tile(self.x, self.y);

		self.x += 1;
		if self.x == self.tiling.tile_width() {
			self.x = 0;
			self.y += 1;
		}

		Some(shape)
	}
}
