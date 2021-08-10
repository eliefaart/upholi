pub mod result;
pub mod http;

pub enum PhotoVariant {
	Original,
	Preview,
	Thumbnail
}

impl PhotoVariant {
	pub fn to_string(&self) -> String {
		match self {
			PhotoVariant::Thumbnail => "thumbnail".into(),
			PhotoVariant::Preview => "preview".into(),
			PhotoVariant::Original => "original".into()
		}
	}
}

impl Into<String> for PhotoVariant {
	fn into(self) -> String {
		self.to_string()
	}
}

impl Into<String> for &PhotoVariant {
	fn into(self) -> String {
		self.to_string()
	}
}