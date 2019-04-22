use std::ffi::CString;

/// Call a C-style function which has an outparam pointer. This is a little
/// wrapper macro to declare a result with a default value, then call an unsafe
/// function which populates the value.
macro_rules! call_unsafe_outparam_fn {
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

/// Create an OpenGL object using several pieces of information:
/// - create_fn: an unsafe GL function which creates the object
/// - get_fn: an unsafe function which checks whether the object was created
/// - get_info_log_fn: an unsafe function used to retrieve the error message
///                    in the event of failure
/// - status_enum: the enum value used in the invocation of `get_fn`
///
/// # Example
///
/// ```rust
/// # extern crate gl;
/// let program_id = create_gl_object!(
///     gl::CreateProgram,
///     gl::GetProgramiv,
///     gl::GetProgramInfoLog,
///     gl::LINK_STATUS
/// ).unwrap();
/// ```
///
macro_rules! create_gl_object {
    ($create_fn:path, $get_fn:path, $get_info_log_fn:path, $status_enum:expr) => {{
        #[allow(unused_unsafe)]
        // this is needed because we could be passed a safe function or closure
        // for any of the fn arguments
        let id = unsafe { $create_fn() };

        #[allow(clippy::cast_lossless)]
        let create_success =
            call_unsafe_outparam_fn!($get_fn, gl::FALSE as GLint, id, $status_enum) as GLboolean;

        if create_success != gl::TRUE {
            println!("Error creating object {:?}", id);
            let len = call_unsafe_outparam_fn!($get_fn, 0, id, gl::INFO_LOG_LENGTH);

            println!("Error details length {:?}", len);
            let error = $crate::util::space_cstring_from_size(len as usize);

            unsafe {
                $get_info_log_fn(id, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }

            Err(error.to_string_lossy().into_owned())
        } else {
            println!("Created object {:?}", id);
            Ok(id)
        }
    }};
}

pub fn space_cstring_from_size(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
