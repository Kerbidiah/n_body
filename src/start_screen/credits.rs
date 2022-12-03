use egui_macroquad::egui;
use egui::{widgets, Ui};

use iter_tools::Itertools;


/// structure to represent a link
#[derive(Debug, Clone)]
pub struct Link {
	/// display name of the link
	// the `'static` is a "lifetime parameter". I don't completly understand how it works, but it does
	pub name: &'static str,
	/// url of link
	pub link: &'static str,
}

impl Link {
	/// the string that splits the name and the url
	const SPLIT_STRING: &str = ": ";

	/// make a `Link` from a `&str` of the format "name: link"
	pub fn make(s: &'static str) -> Option<Self> {
		if s.contains(Self::SPLIT_STRING) {
			// unsafe block allows us to use unwrap_unchecked
			let (name, link) = unsafe {
				// it will always be Some(_) because we checked that there is a split point
				s.split_once(Self::SPLIT_STRING).unwrap_unchecked()
			};
	
			Some(Link {
				name,
				link,
			})

		} else {
			None
		}
	}

	/// display as a hyperlink in the UI
	fn disp(&self, ui: &mut Ui) {
		ui.hyperlink_to(self.name, self.link);
	}

	/// display a "slice" of `Link`s in the UI
	/// a slice is a more generic type that includes vectors and arrays
	pub fn disp_vec(links: &[Self], ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.heading("Refrences:");
			ui.add_space(120.0);
			widgets::global_dark_light_mode_buttons(ui);
		});
		links.iter().for_each(|l| l.disp(ui));
		ui.separator();
	}

	/// parse the copy of references.txt incuded in the binary at compile time into a vector of `Link`s
	pub fn source_list() -> Vec<Self> {
		// `include_str!` is a macro that basicaly copies the given file and pastes it into the code at compile time
		let sources = include_str!("../../references.txt").split('\n');
		let mut links = sources.map(Self::make).collect_vec();
		links.retain(|l| l.is_some());

		links.iter().map(|l| l.as_ref().unwrap().clone()).collect_vec()
	}
}

/// display some info about me and the version of the binary in the UI
pub fn info(ui: &mut Ui) {
	ui.horizontal(|ui| {
		ui.label("By: Alex Janninck");
		ui.hyperlink_to("(ajanninc@purdue.edu)", "mailto:ajanninc@purdue.edu");
	});

	// env! is a macro that gets an enviornment variable at complile time
	ui.weak(format!("version: {}", env!("CARGO_PKG_VERSION")));
	ui.separator();
}
