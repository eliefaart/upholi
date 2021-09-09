use upholi_lib::result::Result;

pub mod photo;
pub mod album;

pub trait Entity {
	type TEncrypted;
	type TDecrypted;
	type TData;
	type TJavaScript;

	fn from_encrypted(source: Self::TEncrypted, private_key: &[u8]) -> Result<Self>
		where Self: std::marker::Sized;
	fn get_id(&self) -> &str;
	fn get_data(&self) -> &Self::TData;
	fn as_js_value(&self) -> &Self::TJavaScript;
}