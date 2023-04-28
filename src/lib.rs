#![cfg(windows)]

pub mod dll;
pub mod loader;

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::Console::FreeConsole;

use simplelog::*;

pub fn init(module: HMODULE) {
	unsafe {
		AllocConsole();
	};
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Trace,
			Config::default(),
			std::fs::File::create("d3d9.log").unwrap(),
		),
	])
	.unwrap();
	log_panics::init();
	log::info!("Hi! {module:?}");
	let r = unsafe { minhook_sys::MH_Initialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Initialize() (returned {r})");
	}

	std::thread::spawn(crate::loader::dll_loader::load_dlls);
}

pub fn free(module: HMODULE) {
	log::info!("Bye! {module:?}");
	let r = unsafe { minhook_sys::MH_Uninitialize() };
	if r != minhook_sys::MH_OK {
		log::error!("Unable to minhook_sys::MH_Uninitialize() (returned {r})");
	}
	unsafe {
		FreeConsole();
	};
}