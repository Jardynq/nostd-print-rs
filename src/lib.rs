#![no_std]

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub mod internal {
    #[cfg(unix)]
    pub use crate::unix::Writer;
    #[cfg(windows)]
    pub use crate::windows::Writer;
}

#[cfg(unix)]
pub use crate::unix::read_to_buf;
#[cfg(windows)]
pub use crate::windows::read_to_buf;

#[cfg(feature = "alloc")]
pub mod alloc_feature {
    extern crate alloc;
    use alloc::string::String;
    
    const BUF_SIZE: usize = 128;
    pub fn read_string() -> String {
        let mut result = String::new();
        let mut buf = [0u8; BUF_SIZE];
        while let Some(count) = super::read_to_buf(&mut buf) {
            if count == 0 {
                break;
            } else if count < BUF_SIZE {
                result.push_str(&String::from_utf8_lossy(&buf[..count]));
                break;
            } else {
                result.push_str(&String::from_utf8_lossy(&buf));
            }
        }
        result
    }
}
#[cfg(feature = "alloc")]
pub use alloc_feature::*;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::nostd_print::internal::Writer::stdout();
        let _ = write!(&mut writer, "{}", format_args!($($arg)*));
    })
}
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::nostd_print::internal::Writer::stdout();
        let _ = writeln!(&mut writer, "{}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::nostd_print::internal::Writer::stderr();
        let _ = write!(&mut writer, "{}", format_args!($($arg)*));
    })
}
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::nostd_print::internal::Writer::stderr();
        let _ = writeln!(&mut writer, "{}", format_args!($($arg)*));
    }};
}

#[allow(unused_imports)]
use crate as nostd_print;

#[cfg(test)]
mod test {
    use crate::*;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Test<'a> {
        float: f32,
        int: isize,
        string: &'static str,
        nested: Option<&'a Test<'a>>,
    }

    #[test]
    fn test_out() {
        let a = Test {
            float: 42.1234,
            int: -42,
            string: "Hello, world!",
            nested: Some(&Test {
                float: -284.213,
                int: 421,
                string: "Bye, world!",
                nested: None,
            }),
        };

        print!("Testing stdout: ");
        println!("{a:#?}");
        eprint!("Testing stderr: ");
        eprintln!("{a:#?}");
    }

    #[test]
    fn test_in() {
        let mut buf = [0u8; 100];
        if read_to_buf(&mut buf).is_some() {
            let s = unsafe { core::str::from_utf8_unchecked(&buf) };
            println!("Input: {s:?}");
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_alloc() {
        let s = read_string();
        println!("String: {s:?}");
    }
}
