pub mod result;
pub mod http;

pub enum PhotoVariant {
	Original,
	Preview,
	Thumbnail
}

impl PhotoVariant {
	pub fn to_str(&self) -> String {
		match self {
			PhotoVariant::Thumbnail => "thumbnail".into(),
			PhotoVariant::Preview => "preview".into(),
			PhotoVariant::Original => "original".into()
		}
	}
}