# Slab Allocator and Linux SLUB Overview

## What is a slab allocator
A slab allocator caches fixed-size objects to reduce allocation overhead
and fragmentation. Memory is divided into slabs containing multiple objects
of the same size.

## Linux SLUB allocator
SLUB is the default Linux kernel slab allocator. It stores freelist pointers
directly inside freed objects and uses per-CPU slabs to reduce contention.

## Exploitation relevance
Heap vulnerabilities such as use-after-free can corrupt freelist pointers,
leading to arbitrary write primitives. SLUB’s simplicity makes it a common
target in kernel exploitation.

## Relation to this project
This Rust implementation mimics SLUB’s freelist-in-object design but omits
advanced features such as per-CPU slabs and page allocators.

## Limitations
This allocator does not implement:
- Per-CPU slabs
- NUMA awareness
- Slab growth or shrinking
- Memory poisoning or debugging features

These features are present in the Linux SLUB allocator but were omitted
to keep the implementation minimal and educational.
