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
		let rn = b"\r\n";
		let body_boundary = br"--MULTIPARTBINARY";
		let end_boundary =  br"--MULTIPARTBINARY--";
		let enc = br"Content-Transfer-Encoding: binary";

		let field_name = match self.parts.len() {
			1 => "file",
			_ => "files",
		};

		body.extend(rn);
		body.extend(rn);

		for part in self.parts {
			let disp = format!("Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"", field_name, part.name);
			let content_type = br"Content-Type: application/octet-stream";

			body.extend(body_boundary.as_ref());
			body.extend(rn);
			body.extend(disp.as_bytes());
			body.extend(rn);
			body.extend(content_type.as_ref());
			body.extend(rn);
			body.extend(enc.as_ref());
			body.extend(rn);
			body.extend(rn);
			body.extend(part.bytes.as_slice());
			body.extend(rn);
		}
		body.extend(end_boundary.as_ref());
		body.extend(rn);
		body.extend(rn);

		let content_length = body.len();
		Multipart {
			body,
			content_length,
			content_type: format!("multipart/form-data; boundary=MULTIPARTBINARY")
		}
	}
}