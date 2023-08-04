
#[link(wasm_import_module = "wasmedge_ffmpeg")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_avFormatOpenInput"] 
                pub fn avFormatOpenInput(file_ptr: *const u8, file_len: usize) -> u32;}