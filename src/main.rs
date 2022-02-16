use std::ffi::c_void;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoA, SPI_SETLOGICALDPIOVERRIDE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

fn main() {
    unsafe {
        SystemParametersInfoA(
            SPI_SETLOGICALDPIOVERRIDE,
            2,
            0 as *mut c_void,
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(1),
        );
    }
}
