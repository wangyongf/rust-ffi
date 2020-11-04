use libc::{c_char, c_uint};
use std::ffi::{CStr, CString};
use std::iter;
use std::slice;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[no_mangle]
pub extern fn hm_chars(s: *const c_char) -> c_uint {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as c_uint
}

#[no_mangle]
pub extern fn batman_song(length: c_uint) -> *mut c_char {
    let mut song = String::from("boom ");
    song.extend(iter::repeat("nana ").take(length as usize));
    song.push_str("Batman! boom");
    let c_str = CString::new(song).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern fn free_song(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn sum_of_even(n: *const c_uint, len: c_uint) -> c_uint {
    let numbers = unsafe {
        assert!(!n.is_null());
        slice::from_raw_parts(n, len as usize)
    };
    let sum = numbers.iter()
        .filter(|&v| v % 2 == 0)
        .fold(0, |acc, v| acc + v);
    sum as c_uint
}

#[repr(C)]
pub struct Turple {
    x: c_uint,
    y: c_uint,
}

impl From<(u32, u32)> for Turple {
    fn from(tup: (u32, u32)) -> Self {
        Turple { x: tup.0, y: tup.1 }
    }
}

impl From<Turple> for (u32, u32) {
    fn from(tup: Turple) -> Self {
        (tup.x, tup.y)
    }
}

fn compute_turple(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b + 1, a - 1)
}

#[no_mangle]
pub extern fn flip_things_around(tup: Turple) -> Turple {
    compute_turple(tup.into()).into()
}

pub struct Database {
    data: HashMap<String, u32>,
}

impl Database {
    fn new() -> Database {
        Database { data: HashMap::new() }
    }

    fn insert(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            self.data.insert(zip, i);
        }
    }

    fn get(&self, zip: &str) -> u32 {
        self.data.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern fn database_new() -> *mut Database {
    Box::into_raw(Box::new(Database::new()))
}

#[no_mangle]
pub extern fn database_insert(ptr: *mut Database) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.insert();
}

#[no_mangle]
pub extern fn database_query(ptr: *const Database, zip: *const c_char) -> c_uint {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.get(zip_str)
}

#[no_mangle]
pub extern fn database_free(ptr: *mut Database) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
