use anyhow::{Ok, Result};
use std::ffi::c_void;
use std::iter::once;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::copy;
use std::{env, ffi::OsStr};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, BOOLEAN, HANDLE},
        Storage::FileSystem::{
            CreateFileW, FileDispositionInfo, FileRenameInfo, SetFileInformationByHandle, DELETE,
            FILE_ATTRIBUTE_NORMAL, FILE_DISPOSITION_INFO, FILE_RENAME_INFO, FILE_RENAME_INFO_0,
            FILE_SHARE_NONE, OPEN_EXISTING,
        },
    },
};

pub fn disappear() -> Result<()> {
    let placeholder = b":legit___";
    if let Some(filename) = env::current_exe()?.to_str() { //get_filename
        let mut handle = open(&filename)?;
        rename(placeholder, handle);
        close(handle);
        handle = open(&filename)?;
        dispose(handle);
        close(handle);
    };
    Ok(())
}

fn open(path: &str) -> Result<HANDLE> {
    let os_path: Vec<u16> = OsStr::new(path).encode_wide().chain(once(0)).collect(); //TO_CHANGE
    let handle = unsafe {
        CreateFileW(
            PCWSTR::from_raw(os_path.as_ptr()),
            DELETE,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    }?;
    Ok(handle)
}

fn rename(placeholder: &[u8; 9], handle: HANDLE) {
    let filename = placeholder.map(|b| b as u16);
    let length = size_of::<[u16; 9]>();
    let mut file_rename_info: FILE_RENAME_INFO = FILE_RENAME_INFO {
        Anonymous: FILE_RENAME_INFO_0 {
            ReplaceIfExists: BOOLEAN(0),
        },
        RootDirectory: HANDLE::default(),
        FileNameLength: length as u32,
        FileName: [0],
    };
    unsafe {
        copy(
            filename.as_ptr(),
            file_rename_info.FileName.as_mut_ptr(),
            length,
        )
    };

    let buffer_size = size_of::<[u16; 9]>() + size_of::<FILE_RENAME_INFO>();
    let result = unsafe {
        SetFileInformationByHandle(
            handle,
            FileRenameInfo,
            &file_rename_info as *const _ as *const c_void,
            buffer_size as u32,
        )
    };
    let _ = Ok(result);
}

fn close(handle: HANDLE) {
    if Some(unsafe { CloseHandle(handle) }).is_some() {
        let _ = Ok(());
    }
}

fn dispose(handle: HANDLE) {
    let file_delete: FILE_DISPOSITION_INFO = FILE_DISPOSITION_INFO {
        DeleteFile: BOOLEAN(1),
    };
    let result = unsafe {
        SetFileInformationByHandle(
            handle,
            FileDispositionInfo,
            &file_delete as *const _ as *const c_void,
            size_of::<FILE_DISPOSITION_INFO>() as u32,
        )
    };
    let _ = Ok(result);
}
