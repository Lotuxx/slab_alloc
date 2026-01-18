/*
# Minimal slab allocator for educational purposes.
#
### Safety
# This allocator uses raw pointers and manual memory management.
# Callers must ensure:
# - Memory passed to the allocator is valid and writable
# - Objects are not used after deallocation
# - No double-free occurs
*/



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

    pub fn free_objects(&self) -> usize {
        self.slab.free_count
    }

    pub fn capacity(&self) -> usize {
        self.slab.capacity
    }

    pub unsafe fn alloc(&mut self) -> Option<*mut u8> {
        if self.slab.freelist.is_null() {
            return None;
        }

        let obj = self.slab.freelist;

        // SAFETY: freelist pointer is guaranteed to point to a valid object
        // created during slab initialization.
        self.slab.freelist = (*obj).next;
        self.slab.free_count -= 1;

        Some(obj as *mut u8)
    }

    /* Deallocate an object back into the slab
    
    # Safety
    - `ptr` must have been allocated by this allocator
     - Double free is undefined behavior
    */
    pub unsafe fn dealloc(&mut self, ptr: *mut u8) {
        let obj = ptr as *mut FreeObject;
        (*obj).next = self.slab.freelist;
        self.slab.freelist = obj;
        self.slab.free_count += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_and_free() {
        const BUF_SIZE: usize = 1024;
        const OBJ_SIZE: usize = 32;

        let mut buffer = vec![0u8; BUF_SIZE];

        let mut alloc = unsafe {
            SlabAllocator::new(buffer.as_mut_ptr(), BUF_SIZE, OBJ_SIZE)
        };

        let a = unsafe { alloc.alloc() }.unwrap();
        let b = unsafe { alloc.alloc() }.unwrap();

        assert_ne!(a, b);

        unsafe {
            alloc.dealloc(a);
            alloc.dealloc(b);
        }

        let c = unsafe { alloc.alloc() }.unwrap();
        assert!(c == a || c == b);
    }

    #[test]
    fn free_count_updates() {
        let mut buffer = vec![0u8; 256];
        let mut alloc = unsafe {
            SlabAllocator::new(buffer.as_mut_ptr(), 256, 32)
        };

        assert_eq!(alloc.free_objects(), alloc.capacity());

        let _a = unsafe { alloc.alloc() }.unwrap();
        assert_eq!(alloc.free_objects(), alloc.capacity() - 1);
    }

}

