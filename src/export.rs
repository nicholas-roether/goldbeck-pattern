use std::iter;

use js_sys::{Array, JsString};
use leptos::leptos_dom::console_error;
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

fn get_svg_elem() -> Option<SvgElement> {
	let Some(elem) = window()
		.unwrap()
		.document()
		.unwrap()
		.query_selector("svg#pattern-svg")
		.unwrap()
	else {
		return None;
	};
	Some(elem.dyn_into().unwrap())
}

fn download_svg(url: &str) -> Result<(), JsValue> {
	let document = window().unwrap().document().unwrap();
	let a = document
		.create_element("a")
		.unwrap()
		.dyn_into::<HtmlAnchorElement>()
		.unwrap();
	a.set_attribute("href", url)?;
	a.set_attribute("download", "pattern.svg")?;
	document.body().unwrap().append_child(&a)?;
	a.click();
	// a.remove();
	Ok(())
}

pub fn export_svg() {
	let Some(svg_elem) = get_svg_elem() else {
		console_error("Cannot export pattern SVG; #pattern-svg not found!");
		return;
	};
	let content = match render_svg(svg_elem) {
		Ok(content) => content,
		Err(err) => {
			console_error(&format!(
				"Failed to render svg: {}",
				err.dyn_into::<JsString>().unwrap()
			));
			return;
		}
	};
	let file = create_svg_file(content).expect("Failed to create SVG file blob");
	match download_svg(&file) {
		Ok(_) => (),
		Err(err) => {
			console_error(&format!(
				"Failed to save svg: {}",
				err.dyn_into::<JsString>().unwrap()
			));
		}
	}
}
