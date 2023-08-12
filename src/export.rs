use std::iter;

use js_sys::{Array, JsString};
use leptos::{document, leptos_dom::console_error};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Blob, BlobPropertyBag, HtmlAnchorElement, SvgElement, Url, XmlSerializer};

fn create_svg_file(content: String) -> Result<String, JsValue> {
	let js_str = JsValue::from_str(&content);
	let js_str_array = Array::from_iter(iter::once(js_str));
	let mut blob_properties = BlobPropertyBag::new();
	blob_properties.type_("image/svg+xml");
	let blob = Blob::new_with_str_sequence_and_options(&js_str_array, &blob_properties)?;
	Url::create_object_url_with_blob(&blob)
}

const EXPORT_HEIGHT: i32 = 500;

fn render_svg(elem: SvgElement) -> Result<String, JsValue> {
	let document = window().unwrap().document().unwrap();
	let svg_doc = document
		.implementation()?
		.create_document(Some("http://www.w3.org/2000/svg"), "svg")?;

	let svg_elem = svg_doc.document_element().unwrap();
	if let Some(view_box) = elem.get_attribute("viewBox") {
		svg_elem.set_attribute("viewBox", &view_box)?;
	}
	svg_elem.set_attribute("height", &format!("{EXPORT_HEIGHT}px"))?;

	let children = elem.children();
	let mut i = 0;
	while let Some(child) = children.item(i) {
		svg_elem.append_child(&child.clone_node_with_deep(true)?)?;
		i += 1;
	}

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

fn get_download_url(selector: &str) -> Option<String> {
	let Some(svg_elem) = get_svg_elem(selector) else {
		console_error("Cannot export SVG; element not found!");
		return None;
	};
	let content = match render_svg(svg_elem) {
		Ok(content) => content,
		Err(err) => {
			console_error(&format!(
				"Failed to render svg: {}",
				err.dyn_into::<JsString>().unwrap()
			));
			return None;
		}
	};
	let url = create_svg_file(content).expect("Failed to create SVG file blob");
	Some(url)
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

pub fn export_svg(selector: &str, filename: &str) {
	let Some(download_url) = get_download_url(selector) else {
		console_error("SVG File creation failed");
		return;
	};
	download_file(&download_url, filename);
}
