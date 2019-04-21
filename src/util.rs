use std::ffi::CString;

/// Call a C-style function which has an outparam pointer. This is a little
/// wrapper macro to declare a result with a default value, then call an unsafe
/// function which populates the value.
macro_rules! call_output_fn {
    ($function:path, $default_value:expr, $($args:expr),*) => {
        {
            let mut result = $default_value;
            unsafe {
                $function($($args),*, &mut result);
            }
            result
        }
    };
}

pub fn space_cstring_from_size(len: usize) -> CString {
    let mut buffer = vec![b' '; len + 1];
    // null-terminate the CString
    *buffer.last_mut().unwrap() = 0;
    unsafe { CString::from_vec_unchecked(buffer) }
}
