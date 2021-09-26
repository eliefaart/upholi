use upholi_lib::result::Result;

pub mod photo;
pub mod album;

pub trait Entity {
	type TEncrypted;
	type TDecrypted;
	type TData;
	type TJavaScript;

	fn from_encrypted(source: Self::TEncrypted, key_name: &str, key: &[u8]) -> Result<Self>
		where Self: std::marker::Sized;
	fn get_id(&self) -> &str;
	fn get_decrypted(&self) -> &Self::TDecrypted;
	fn get_data(&self) -> &Self::TData;
	fn get_data_mut(&mut self) -> &mut Self::TData;
	fn as_js_value(&self) -> &Self::TJavaScript;
}