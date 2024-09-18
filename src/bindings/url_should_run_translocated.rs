use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::result::from_bool_and_error_to_result;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::{null, null_mut};

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L163

    fn SecTranslocateURLShouldRunTranslocated(
        path: _Nonnull<CFURLRef>,
        should_translocate: _Nonnull<*mut bool>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)
}

pub fn url_should_run_translocated(path: CFURL) -> Result<bool, CFError> {
    let path_ref = path.as_concrete_TypeRef();

    let mut should_translocate = false;
    let mut error_ref = null_mut();
    let ok = unsafe {
        SecTranslocateURLShouldRunTranslocated(path_ref, &mut should_translocate, &mut error_ref)
    };

    let should_translocate_ref = if ok { &should_translocate } else { null() };
    from_bool_and_error_to_result(should_translocate_ref, error_ref)
}
