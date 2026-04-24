mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // См. неопределенные типы (opaque) https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Раскладка согласно ман странице Linux для функции readdir(3), где ino_t и
    // off_t соответствуют определениям в
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Раскладка в соответствии в ман страницей macOS для dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        // См. https://github.com/rust-lang/libc/issues/414 и раздел
        // _DARWIN_FEATURE_64_BIT_INODE в ман страницах macOS для stat(2).
        //
        // "Platforms that existed before these updates were available" это ссылка на
        // macOS (в противоположность iOS / wearOS / и пр.) на Intel и PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Конвертируем Rust-строку &str в C-строку CString
        // CString::new может вернуть ошибку, если в строке есть нулевой байт
        let c_path = CString::new(path)
            .map_err(|_| format!("путь содержит нулевой байт: {}", path))?;

        // Вызываем opendir через FFI — он принимает *const c_char
        let dir = unsafe { ffi::opendir(c_path.as_ptr()) };

        // opendir возвращает нулевой указатель при ошибке
        if dir.is_null() {
            Err(format!("не удалось открыть директорию: {}", path))
        } else {
            Ok(DirectoryIterator {
                path: c_path,
                dir,
            })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        // Вызываем readdir — он возвращает указатель на dirent или NULL
        // NULL означает, что записи в директории закончились
        let entry = unsafe { ffi::readdir(self.dir) };

        if entry.is_null() {
            None
        } else {
            // Цепочка конвертаций:
            // &entry.d_name — это [c_char; 256]
            // .as_ptr() — получаем *const c_char
            // CStr::from_ptr() — получаем &CStr (находим конец по нулевому байту)
            // .to_bytes() — получаем &[u8] (универсальный интерфейс для "неизвестных данных")
            // OsStr::from_bytes() — получаем &OsStr (создаём из байтов)
            // .to_os_string() — получаем OsString (копируем данные)
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) };
            Some(OsStr::from_bytes(name.to_bytes()).to_os_string())
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Вызываем closedir для освобождения ресурсов, выделенных opendir
        // Это критически важно — без этого будет утечка файловых дескрипторов
        unsafe {
            ffi::closedir(self.dir);
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("файлы: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
