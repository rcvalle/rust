//@ build-pass
//@ needs-sanitizer-cfi
<<<<<<< HEAD
//@ compile-flags: -Ccodegen-units=1 -Clto -Ctarget-feature=-crt-static -Zsanitizer=cfi -C unsafe-allow-abi-mismatch=sanitizer
=======
//@ compile-flags: -Ccodegen-units=1 -Clto -Ctarget-feature=-crt-static -Zunstable-options -Csanitize=cfi
>>>>>>> 832830ac5a0 (sanitizers: Add support for stable sanitizers)
//@ no-prefer-dynamic
//@ only-x86_64-unknown-linux-gnu
//@ ignore-backends: gcc

#![feature(allocator_api)]

fn main() {
    let _ = Box::new_in(&[0, 1], &std::alloc::Global);
}
