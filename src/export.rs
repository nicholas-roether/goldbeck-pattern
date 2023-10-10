use std::{error::Error, fmt, iter};

use enum_iterator::Sequence;
use js_sys::{Array, JsString, Uint8Array};
use leptos::{
	document,
	leptos_dom::logging::{console_error, console_log}
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Blob, HtmlAnchorElement, SvgElement, Url, XmlSerializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
#[repr(u8)]
pub enum OutputFormat {
	Svg,
	Ai,
	Pdf
}

impl OutputFormat {
	pub fn extension(&self) -> &'static str {
		match self {
			Self::Svg => "svg",
			Self::Pdf => "pdf",
			Self::Ai => "ai"
		}
	}
}

impl fmt::Display for OutputFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, ".{}", self.extension())
	}
}

fn create_url(content: Vec<u8>) -> Result<String, JsValue> {
	let js_bytes = unsafe { Uint8Array::view(&content) };
	let js_bytes_array = Array::from_iter(iter::once(js_bytes));
	let blob = Blob::new_with_u8_array_sequence(&js_bytes_array)?;
	Url::create_object_url_with_blob(&blob)
}

const EXPORT_HEIGHT: i32 = 1000;

fn copy_children(source: &SvgElement, target: &SvgElement) -> Result<(), JsValue> {
	let children = source.children();
	let mut i = 0;
	while let Some(child) = children.item(i) {
		if let Ok(svg_child) = child.dyn_into::<SvgElement>() {
			let clone = svg_child.clone_node()?.dyn_into::<SvgElement>().unwrap();
			copy_children(&svg_child, &clone)?;
			target.append_child(&clone)?;
		}
		i += 1;
	}
	Ok(())
}

fn render_svg(elem_in_page: SvgElement) -> Result<String, JsValue> {
	let document = window().unwrap().document().unwrap();
	let svg_doc = document
		.implementation()?
		.create_document(Some("http://www.w3.org/2000/svg"), "svg")?;

	let svg_elem = svg_doc
		.document_element()
		.unwrap()
		.dyn_into::<SvgElement>()
		.unwrap();
	let view_box = elem_in_page
		.get_attribute("viewBox")
		.expect("Export SVG is missing viewBox attribute!");
	let vb_parts = view_box.split(' ').collect::<Vec<&str>>();
	let vb_width: i32 = vb_parts[2]
		.parse()
		.expect("Failed to parse export SVG's viewBox width");
	let vb_height: i32 = vb_parts[3]
		.parse()
		.expect("Failed to parse export SVG's viewBox height");

	let aspect_ratio = vb_width / vb_height;
	let export_width = EXPORT_HEIGHT * aspect_ratio;

	svg_elem.set_attribute("viewBox", &view_box)?;
	svg_elem.set_attribute("width", &export_width.to_string())?;
	svg_elem.set_attribute("height", &EXPORT_HEIGHT.to_string())?;

	copy_children(&elem_in_page, &svg_elem)?;

	let serializer = XmlSerializer::new()?;
	let xml = serializer.serialize_to_string(&svg_doc)?;
	Ok(xml)
}

fn get_svg_elem(selector: &str) -> Option<SvgElement> {
	let Some(elem) = window()
		.unwrap()
		.document()
		.unwrap()
		.query_selector(selector)
		.unwrap()
	else {
		return None;
	};
	Some(elem.dyn_into().unwrap())
}

fn convert_from_svg(svg_content: String, format: OutputFormat) -> Result<Vec<u8>, Box<dyn Error>> {
	match format {
		OutputFormat::Svg => Ok(svg_content.bytes().collect()),
		OutputFormat::Pdf | OutputFormat::Ai => Ok(svg2pdf::convert_str(
			&svg_content,
			svg2pdf::Options::default()
		)?)
	}
}

fn get_download_url(selector: &str, format: OutputFormat) -> Result<String, ()> {
	let Some(svg_elem) = get_svg_elem(selector) else {
		console_error("Cannot export SVG; element not found!");
		return Err(());
	};
	let svg_content = match render_svg(svg_elem) {
		Ok(content) => content,
		Err(err) => {
			console_error(&format!(
				"Failed to render svg: {}",
				err.dyn_into::<JsString>().unwrap()
			));
			return Err(());
		}
	};

	let output_content = match convert_from_svg(svg_content, format) {
		Ok(output_content) => output_content,
		Err(err) => {
			console_error(&format!("Failed to convert to {format}: {err}"));
			return Err(());
		}
	};

	let url = create_url(output_content).expect("Failed to create SVG file blob");
	Ok(url)
}

fn download_file(url: &str, filename: &str) {
	let document = document();
	let a = document
		.create_element("a")
		.expect("Failed to create anchor element!")
		.dyn_into::<HtmlAnchorElement>()
		.unwrap();
	a.set_attribute("href", url).unwrap();
	a.set_attribute("download", filename).unwrap();
	a.set_attribute("style", "display: none").unwrap();
	document
		.body()
		.expect("Document body is not present!")
		.append_child(&a)
		.expect("Failed to append temporary anchor to document");
	a.click();
	a.remove();
}

pub fn export_pattern(selector: &str, filename: &str, format: OutputFormat) {
	let Ok(download_url) = get_download_url(selector, format) else {
		console_error("SVG File creation failed");
		return;
	};

	let complete_filename = format!("{filename}.{}", format.extension());

	console_log(&complete_filename);

	download_file(&download_url, &complete_filename);
}

