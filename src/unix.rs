use libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, c_int, c_void, size_t};

pub struct Writer {
    handle: c_int,
}
impl Writer {
    pub fn stdout() -> Self {
        Self { handle: STDOUT_FILENO }
    }
    pub fn stderr() -> Self {
        Self { handle: STDERR_FILENO }
    }
}
impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let status = unsafe { 
            libc::write(
                self.handle, 
                s.as_ptr() as *const c_void,
                s.len() as size_t
            )
        };
        if status >= 0 { Ok(()) } else { Err(core::fmt::Error) }
    }
}

pub fn read_to_buf(buf: &mut [u8]) -> Option<usize> {
    let res = unsafe {
        libc::read(
            STDIN_FILENO,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len(),
        )
    };
    if res < 0 { None } else { Some(res as usize) }
}
