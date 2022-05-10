use actix_web::web::{delete, get, post, put};
use actix_web::{App, HttpServer};
use handlers::{albums::*, photos::*, shares::*, users::*};

mod cookies;
mod handlers;
mod http;

/// Start and run the web server
pub async fn run_server() -> std::io::Result<()> {
	let address = &crate::SETTINGS.server.address;
	let wwwroot_path = &crate::SETTINGS.server.wwwroot_path;

	println!("Hello server, address: {}", address);

	HttpServer::new(move || {
		App::new()
			// API routes
			.service(
				actix_web::web::scope("/api")
					.route("/user/register", post().to(route_register_user))
					.route("/user/login", post().to(route_login_user))
					.route("/user/info", get().to(route_user_info))
					.route("/photos", get().to(route_get_photos))
					.route("/photos/minimal", get().to(route_get_photos_minimal))
					.route("/photos/find", post().to(route_find_photos))
					.route("/photos/find/minimal", post().to(route_find_photos_minimal))
					.route("/photo/exists", get().to(route_check_photo_exists))
					.route("/photo", post().to(route_upload_photo))
					.route("/photo/{photo_id}", get().to(route_get_photo))
					.route("/photo/{photo_id}", delete().to(route_delete_photo))
					.route("/photo/{photo_id}/original", get().to(route_download_photo_original))
					.route("/photo/{photo_id}/thumbnail", get().to(route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", get().to(route_download_photo_preview))
					.route("/albums", get().to(route_get_albums))
					.route("/album", post().to(route_create_album))
					.route("/album/{album_id}", get().to(route_get_album))
					.route("/album/{album_id}", put().to(route_update_album))
					.route("/album/{album_id}", delete().to(route_delete_album))
					.route("/shares", get().to(route_get_shares))
					.route("/share", post().to(route_create_share))
					.route("/share/{share_id}", get().to(route_get_share))
					.route("/share/{share_id}", put().to(route_update_share))
					.route("/share/{share_id}", delete().to(route_delete_share)),
			)
			// Static files
			.service(actix_files::Files::new("/", wwwroot_path).index_file("index.html"))
	})
	.bind(address)
	.unwrap_or_else(|_| panic!("Failed to bind to address, perhaps the port is in use?"))
	.run()
	.await
}

// pub async fn route_index(req: HttpRequest) -> Result<NamedFile> {
// 	let path: PathBuf = req.match_info().query("filename").parse().unwrap();
// 	println!("{path:?}");
// 	Ok(NamedFile::open(path)?)
// }
