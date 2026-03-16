fn danger() {
    unsafe {
        let _ptr = std::ptr::null::<u8>();
    }
}
