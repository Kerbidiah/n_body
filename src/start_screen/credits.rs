use iter_tools::Itertools;
use egui_macroquad::egui::Ui;

#[derive(Debug, Clone)]
pub struct Link {
	/// display name of the link
	// the 'static is a lifetime parameter I don't completly understand how it works
	pub name: &'static str,
	/// url of link
	pub link: &'static str,
}

impl Link {
	const SPLIT_STRING: &str = ": ";
	/// make a `Link` from a `&str` of the format "name: link"
	pub fn make(s: &'static str) -> Option<Self> {
		if s.contains(Self::SPLIT_STRING) {
			// unsafe block allows us to use unwrap_unchecked
			let (name, link) = unsafe {
				// this will always be safe because we checked that it has the split point
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

	fn disp(&self, ui: &mut Ui) {
		ui.hyperlink_to(self.name, self.link);
	}

	pub fn disp_vec(links: &[Self], ui: &mut Ui) {
		ui.heading("Refrences:");
		links.iter().for_each(|l| l.disp(ui));
		ui.separator();
	}

	pub fn source_list() -> Vec<Self> {
		// include_str! is a macro that basically copies the given file and pastes it in the code at compile time
		let sources = include_str!("../../references.txt").split('\n');
		let mut links = sources.map(Self::make).collect_vec();
		links.retain(|l| l.is_some());

		links.iter().map(|l| l.as_ref().unwrap().clone()).collect_vec()
	}
}

pub fn info(ui: &mut Ui) {
	ui.horizontal(|ui| {
		ui.label("By: Alex Janninck");
		ui.hyperlink_to("(ajanninc@purdue.edu)", "mailto:ajanninc@purdue.edu");
	});

	// env! is a macro that gets the version from cargo.toml
	ui.weak(format!("version: {}", env!("CARGO_PKG_VERSION")));
	ui.separator();
}