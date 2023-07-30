#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vertex {
	x: f32,
	y: f32
}

impl Vertex {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
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
			path.push_str(&format!("{},{}", vertex.x, vertex.y))
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

	let mut vertices: [Tile; NUM_TILES] = [[Vertex::new(0.0, 0.0); 4]; NUM_TILES];

	let mut index: usize = 0;
	let mut x = 0;
	let mut y = 0;

	while index < NUM_TILES {
		let x_f = x as f32;
		let y_f = y as f32;
		let offset_i = (y % PATTERN_HEIGHT) as f32;
		let offset_top = 1.0 - offset_i / PATTERN_HEIGHT as f32;
		let offset_bottom = 1.0 - (offset_i + 1.0) / PATTERN_HEIGHT as f32;

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

		index += 2;
		x += 1;
		if x > width {
			x = 0;
			y += 1;
		}
		if y > height {
			break;
		}
	}

	vertices
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TilingFormat {
	Small,
	Wide,
	Tall,
	Large
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tiling {
	tiles: &'static [Tile],
	viewport_width: f32,
	viewport_height: f32
}

impl Tiling {
	const SMALL_TILES: [Tile; num_tiles(1, 1)] = generate_tiles(1, 1);
	pub const SMALL: Self = Self::new(&Self::SMALL_TILES, 5.0, 5.0);

	const WIDE_TILES: [Tile; num_tiles(2, 1)] = generate_tiles(2, 1);
	pub const WIDE: Self = Self::new(&Self::WIDE_TILES, 10.0, 5.0);

	const TALL_TILES: [Tile; num_tiles(1, 2)] = generate_tiles(1, 2);
	pub const TALL: Self = Self::new(&Self::TALL_TILES, 5.0, 10.0);

	const LARGE_TILES: [Tile; num_tiles(2, 2)] = generate_tiles(2, 2);
	pub const LARGE: Self = Self::new(&Self::LARGE_TILES, 10.0, 10.0);

	const fn new(tiles: &'static [Tile], viewport_width: f32, viewport_height: f32) -> Self {
		Tiling {
			tiles,
			viewport_width,
			viewport_height
		}
	}

	pub fn load(format: TilingFormat) -> Self {
		match format {
			TilingFormat::Small => Self::SMALL,
			TilingFormat::Wide => Self::WIDE,
			TilingFormat::Tall => Self::TALL,
			TilingFormat::Large => Self::LARGE
		}
	}

	pub fn view_box(&self) -> String {
		format!("0 0 {} {}", self.viewport_width, self.viewport_height)
	}

	pub fn iter_tiles(&self) -> impl Iterator<Item = Shape> {
		self.tiles.iter().map(|points| Shape(points))
	}
}
