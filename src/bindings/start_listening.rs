use crate::internal::annotations::_Nullable;
use crate::internal::option::from_bool_and_error_to_option;
use core_foundation::error::{CFError, CFErrorRef};
use std::ptr::null_mut;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L44

    fn SecTranslocateStartListening(error: _Nullable<*mut CFErrorRef>) -> bool; // __OSX_AVAILABLE(10.12)
}

pub fn start_listening() -> Option<CFError> {
    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateStartListening(&mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}
