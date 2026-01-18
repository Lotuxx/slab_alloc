#![no_std]

#[cfg(test)]
extern crate std;



use core::ptr::null_mut;

#[repr(C)]
struct FreeObject {
    next: *mut FreeObject,
}

// data struct
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



// slab allocator over a fixed memory region
impl SlabAllocator {
    
    pub unsafe fn new(
        buffer: *mut u8,
        buffer_size: usize,
        object_size: usize,
    ) -> Self {
        let mut freelist = null_mut();
        let mut offset = 0;
        let mut count = 0;

        while offset + object_size <= buffer_size {
            let obj = buffer.add(offset) as *mut FreeObject;
            (*obj).next = freelist;
            freelist = obj;
            offset += object_size;
            count += 1;
        }

        Self {
            slab: Slab {
                start: buffer,
                freelist,
                object_size,
                capacity: count,
                free_count: count,
            },
        }
    }
}
