
pub unsafe fn struct_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}
pub unsafe fn u8_slice_to_ref<T>(bytes: &[u8]) -> &T {
    let bytes_ptr = bytes.as_ptr();
    let test : *const T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value = unsafe {test.as_ref()}.unwrap();
    return value;
}
pub unsafe fn u8_slice_to_ref_mut<T>(bytes: &mut [u8]) -> &mut T {
    let bytes_ptr = bytes.as_ptr();
    let test : *mut T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value : &mut T = unsafe {test.as_mut()}.unwrap();
    return value;
}

pub unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}