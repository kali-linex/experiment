use std::ffi::CString;

fn main() {
    reqwest::blocking::get("https://my-server.net/");
    let title = CString::new("Experiment").unwrap();
    let message = CString::new("This was an experiment to see how many people would open a random EXE file sent to them by an untrusted party. It's recommended that you do not do this. Opening the file was logged. There was no harm done to your computer.").unwrap();
    unsafe {
        winapi::um::winuser::MessageBoxA(std::ptr::null_mut(), title.as_ptr(), message.as_ptr(), winapi::um::winuser::MB_OK);
    }
}
