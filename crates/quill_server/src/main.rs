#[actix_web::main]
async fn main()->color_eyre::Result<()>{
	quill_server_lib::run().await
}
