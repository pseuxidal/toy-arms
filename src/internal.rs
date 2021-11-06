use std::str::Utf8Error;
use winapi::shared::minwindef::HMODULE;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use crate::{get_module_handle, read_null_terminated_string};
use crate::cast;
use std::mem::{size_of, zeroed};
use winapi::um::processthreadsapi::GetCurrentProcess;
use crate::pattern_scan_core::pattern_scan_core;

pub struct Module<'a> {
    pub module_name: &'a str,
    pub module_handle: HMODULE,
    pub module_size: u32,
    pub module_base_address: usize,
}

impl<'a> Module<'a> {
    pub fn from_module_name(module_name: &'a str) -> Self {
        let module_handle: HMODULE = get_module_handle(module_name);

        unsafe {
            let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
            GetModuleInformation(GetCurrentProcess(), module_handle, &mut module_info, size_of::<MODULEINFO>() as u32);
            Module {
                module_name,
                module_handle,
                module_base_address: module_info.lpBaseOfDll as usize,
                module_size: module_info.SizeOfImage,
            }
        }
    }

    pub fn read<T>(&self, address: i32) -> *const T {
        cast!(self.module_handle as usize + address as usize, T)
    }

    pub fn read_mut<T>(&self, address: i32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe{
            Ok(read_null_terminated_string(self.module_handle as usize + address as usize)?)
        }

    }

    pub fn pattern_scan(&self, pattern: &str, offset: isize, extra: usize) -> Option<usize> {
        let p_array = pattern.split(" ").collect::<Vec<&str>>();
        let mut pattern_vec: Vec<u8> = Vec::new();
        for p in p_array {
            if p == "?" {
                pattern_vec.push(b'?');
                continue;
            }
            pattern_vec.push(u8::from_str_radix(p, 16).unwrap());
        }
        let pattern_b = pattern_vec.as_slice();
        let base = self.module_base_address as *mut u8;
        let end = self.module_base_address + self.module_size as usize;
        unsafe {
            let address = pattern_scan_core(base, end, pattern_b)?;
            // calculate relative address
            Some(*(address.offset(offset) as *mut usize) - base as usize + extra)
        }
    }
}
