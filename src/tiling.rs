#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vertex {
	pub x: f32,
	pub y: f32
}

fn svg_precision(val: f32) -> f32 {
	(val * 1000.0).round() / 1000.0
}

impl Vertex {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn svg_point(&self) -> String {
		format!("{},{}", svg_precision(self.x), svg_precision(self.y))
	}
}

#[derive(Debug, Clone)]
pub struct Shape(&'static [Vertex]);

impl Shape {
	pub fn svg_path(&self) -> String {
		let mut path = String::new();
		for vertex in self.0 {
			if !path.is_empty() {
				path.push(' ');
			}
			path.push_str(&vertex.svg_point())
		}
		path
	}
}

const PATTERN_SIZE_SQUARES: usize = 5;
const PATTERN_HEIGHT: usize = PATTERN_SIZE_SQUARES;
const PATTERN_WIDTH: usize = 2 * PATTERN_HEIGHT;

const fn num_tiles(reps_x: usize, reps_y: usize) -> usize {
	reps_x * PATTERN_WIDTH * reps_y * PATTERN_HEIGHT
}

type Tile = [Vertex; 4];

const fn generate_tiles<const NUM_TILES: usize>(reps_x: usize, reps_y: usize) -> [Tile; NUM_TILES] {
	let width = reps_x * PATTERN_SIZE_SQUARES;
	let height = reps_y * PATTERN_SIZE_SQUARES;

	let mut tiles: [Tile; NUM_TILES] = [[Vertex::new(0.0, 0.0); 4]; NUM_TILES];

	let mut index: usize = 0;
	let mut x = 0;
	let mut y = 0;

	while index < NUM_TILES {
		let x_f = x as f32;
		let y_f = y as f32;
		let offset_i = (y % PATTERN_HEIGHT) as f32;
		let offset_top = 1.0 - offset_i / PATTERN_HEIGHT as f32;
		let offset_bottom = 1.0 - (offset_i + 1.0) / PATTERN_HEIGHT as f32;

		tiles[index] = [
			Vertex::new(x_f, y_f),
			Vertex::new(x_f + offset_top, y_f),
			Vertex::new(x_f + offset_bottom, y_f + 1.0),
			Vertex::new(x_f, y_f + 1.0)
		];
		tiles[index + 1] = [
			Vertex::new(x_f + offset_top, y_f),
			Vertex::new(x_f + 1.0, y_f),
			Vertex::new(x_f + 1.0, y_f + 1.0),
			Vertex::new(x_f + offset_bottom, y_f + 1.0)
		];

		index += 2;
		x += 1;
		if x == width {
			x = 0;
			y += 1;
		}
		if y == height {
			break;
		}
	}

	tiles
}

const fn num_lines(reps_x: usize, reps_y: usize) -> usize {
	let vertical = reps_x * PATTERN_SIZE_SQUARES - 1;
	let horizontal = reps_y * PATTERN_SIZE_SQUARES - 1;
	let diagonal = vertical + reps_y;
	vertical + horizontal + diagonal
}

type Line = [Vertex; 2];

const fn generate_lines<const NUM_LINES: usize>(reps_x: usize, reps_y: usize) -> [Line; NUM_LINES] {
	let width = reps_x * PATTERN_SIZE_SQUARES;
	let height = reps_y * PATTERN_SIZE_SQUARES;
	let num_vertical = width - 1;
	let num_horizontal = height - 1;
	let num_diagonal = num_vertical + reps_y;

	let mut lines: [Line; NUM_LINES] = [[Vertex::new(0.0, 0.0); 2]; NUM_LINES];
	let mut i = 0;

	let mut x = 1;
	while x <= num_vertical {
		lines[i] = [
			Vertex::new(x as f32, 0.0),
			Vertex::new(x as f32, height as f32)
		];
		x += 1;
		i += 1;
	}

	let mut y = 1;
	while y <= num_horizontal {
		lines[i] = [
			Vertex::new(0.0, y as f32),
			Vertex::new(width as f32, y as f32)
		];
		y += 1;
		i += 1;
	}

	let mut top_x = 1;
	while top_x <= num_diagonal {
		let bottom_x = top_x as i32 - reps_y as i32;
		lines[i] = [
			Vertex::new(top_x as f32, 0.0),
			Vertex::new(bottom_x as f32, height as f32)
		];
		top_x += 1;
		i += 1;
	}

	lines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TilingFormat {
	F5X5,
	F10X10,
	F10X15,
	F15X15
}

#[derive(Debug)]
pub struct Tiling {
	tiles: &'static [Tile],
	lines: &'static [Line],
	viewport_width: f32,
	viewport_height: f32
}

impl PartialEq for Tiling {
	fn eq(&self, other: &Self) -> bool {
		self.tiles.len() == other.tiles.len()
	}
}

impl Tiling {
	const F5X5_TILES: [Tile; num_tiles(1, 1)] = generate_tiles(1, 1);
	const F5X5_LINES: [Line; num_lines(1, 1)] = generate_lines(1, 1);
	pub const F5X5: Self = Self::new(
		&Self::F5X5_TILES,
		&Self::F5X5_LINES,
		PATTERN_SIZE_SQUARES as f32,
		PATTERN_SIZE_SQUARES as f32
	);

	const F10X10_TILES: [Tile; num_tiles(2, 2)] = generate_tiles(2, 2);
	const F10X10_LINES: [Line; num_lines(2, 2)] = generate_lines(2, 2);
	pub const F10X10: Self = Self::new(
		&Self::F10X10_TILES,
		&Self::F10X10_LINES,
		2.0 * PATTERN_SIZE_SQUARES as f32,
		2.0 * PATTERN_SIZE_SQUARES as f32
	);

	const F10X15_TILES: [Tile; num_tiles(2, 3)] = generate_tiles(2, 3);
	const F10X15_LINES: [Line; num_lines(2, 3)] = generate_lines(2, 3);
	pub const F10X15: Self = Self::new(
		&Self::F10X15_TILES,
		&Self::F10X15_LINES,
		2.0 * PATTERN_SIZE_SQUARES as f32,
		3.0 * PATTERN_SIZE_SQUARES as f32
	);

	const F15X15_TILES: [Tile; num_tiles(3, 3)] = generate_tiles(3, 3);
	const F15X15_LINES: [Line; num_lines(3, 3)] = generate_lines(3, 3);
	pub const F15X15: Self = Self::new(
		&Self::F15X15_TILES,
		&Self::F15X15_LINES,
		3.0 * PATTERN_SIZE_SQUARES as f32,
		3.0 * PATTERN_SIZE_SQUARES as f32
	);

	const fn new(
		tiles: &'static [Tile],
		lines: &'static [Line],
		viewport_width: f32,
		viewport_height: f32
	) -> Self {
		Tiling {
			tiles,
			lines,
			viewport_width,
			viewport_height
		}
	}

	pub fn load(format: TilingFormat) -> Self {
		match format {
			TilingFormat::F5X5 => Self::F5X5,
			TilingFormat::F10X10 => Self::F10X10,
			TilingFormat::F10X15 => Self::F10X15,
			TilingFormat::F15X15 => Self::F15X15
		}
	}

	pub fn viewport_width(&self) -> f32 {
		self.viewport_width
	}

	pub fn viewport_height(&self) -> f32 {
		self.viewport_height
	}

	pub fn iter_tiles(&self) -> impl Iterator<Item = Shape> {
		self.tiles.iter().map(|points| Shape(points))
	}

	pub fn num_tiles(&self) -> usize {
		self.tiles.len()
	}

	pub fn iter_lines(&self) -> impl Iterator<Item = &'static Line> {
		self.lines.iter()
	}
}
