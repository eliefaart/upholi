pub fn compute_md5_hash(bytes: &[u8]) -> String {
	let mut md5_context = md5::Context::new();
	md5_context.consume(&bytes);
	let digest = md5_context.compute();
	format!("{:?}", digest)
}