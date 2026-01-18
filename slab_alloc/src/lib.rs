#![no_std]

#[cfg(test)]
extern crate std;



use core::ptr::null_mut;

#[repr(C)]
struct FreeObject {
    next: *mut FreeObject,
}

pub struct Slab {
    start: *mut u8,
    freelist: *mut FreeObject,
    object_size: usize,
    capacity: usize,
    free_count: usize,
}

pub struct SlabAllocator {
    slab: Slab,
}
