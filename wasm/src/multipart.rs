struct MultipartPart {
	name: String,
	bytes: Vec<u8>
}

pub struct Multipart {
	pub body: Vec<u8>,
	pub content_length: usize,
	pub content_type: String
}

pub struct MultipartBuilder {
	parts: Vec<MultipartPart>
}

impl MultipartBuilder {
	pub fn new() -> Self {
		Self {
			parts: vec!{}
		}
	}

	pub fn add_bytes(mut self, name: &str, bytes: &[u8]) -> Self {
		let part = MultipartPart {
			name: name.into(),
			bytes: bytes.into()
		};
		self.parts.push(part);
		self
	}

	pub fn build(self) -> Multipart {
		// Based on:
		// https://www.reddit.com/r/rust/comments/69ywsr/multipartform_request_with_reqwest/dhaqszf?utm_source=share&utm_medium=web2x&context=3

		let mut body = Vec::new();
		let newline = b"\r\n";
		let body_boundary = br"--MULTIPARTBINARY";
		let end_boundary =  br"--MULTIPARTBINARY--";

		for part in self.parts {
			let disp = format!("Content-Disposition: form-data; name=\"{}\"", part.name);
			let content_type = br"Content-Type: application/octet-stream";

			body.extend(body_boundary.as_ref());
			body.extend(newline);
			body.extend(disp.as_bytes());
			body.extend(newline);
			body.extend(content_type.as_ref());
			body.extend(newline);
			//body.extend(enc.as_ref());
			//body.extend(rn);
			body.extend(newline);
			body.extend(part.bytes.as_slice());
			body.extend(newline);
		}

		body.extend(end_boundary.as_ref());

		let content_length = body.len();
		Multipart {
			body,
			content_length,
			content_type: "multipart/form-data; boundary=MULTIPARTBINARY".to_string()
		}
	}
}