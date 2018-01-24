use const_api;

#[derive(Debug)]
pub struct LauncherStructPipe {
	pub file_descriptor_child: i32,
	file_descriptor_pairent: i32,
	file_descriptor_id_child: i32,
	pipe_type: const_api::Direction
}
