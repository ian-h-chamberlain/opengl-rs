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
        use std::ffi::CString;

        // we could be passed a safe function or closure which creates the object
        #[allow(unused_unsafe)]
        let id = unsafe { $create_fn() };

        let create_success = call_unsafe_outparam_fn!($get_fn, -1, id, $status_enum);
        assert_ne!(-1, create_success);

        let create_success = create_success as GLboolean;
        if create_success == gl::TRUE {
            eprintln!("`{}` returned success; id {:?}", stringify!($get_fn), id);
            Ok(id)
        } else {
            eprintln!("Error creating object with id {:?}", id);
            let len = call_unsafe_outparam_fn!($get_fn, -1, id, gl::INFO_LOG_LENGTH);
            assert!(len >= 0);

            let error = unsafe { CString::from_vec_unchecked(vec![b' '; len as usize]) };

            #[allow(unused_unsafe)]
            unsafe {
                $get_info_log_fn(id, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }

            Err(error.to_string_lossy().into_owned())
        }
    }};
}
