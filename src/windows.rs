use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    GetStdHandle, ReadConsoleA, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    WriteConsoleA,
};

pub struct Writer {
    pub handle: HANDLE,
}
impl Writer {
    pub fn stdout() -> Self {
        Self {
            handle: unsafe { GetStdHandle(STD_OUTPUT_HANDLE) },
        }
    }
    pub fn stderr() -> Self {
        Self {
            handle: unsafe { GetStdHandle(STD_ERROR_HANDLE) },
        }
    }
}
impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let res = unsafe {
            WriteConsoleA(
                self.handle,
                s.as_ptr() as *const _,
                s.len() as u32,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        if res != 0 { Ok(()) } else { Err(core::fmt::Error) }
    }
}

pub fn read_to_buf(buf: &mut [u8]) -> Option<usize> {
    let mut count: usize = 0;
    let res = unsafe {
        ReadConsoleA(
            GetStdHandle(STD_INPUT_HANDLE),
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
            &mut count as *mut _ as *mut _,
            core::ptr::null_mut(),
        )
    };
    if res != 0 { Some(count) } else { None }
}
