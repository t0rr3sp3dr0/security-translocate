use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::result::from_bool_and_error_to_result;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::{null, null_mut};

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L190

    fn SecTranslocateIsTranslocatedURL(
        path: _Nonnull<CFURLRef>,
        is_translocated: _Nonnull<*mut bool>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)
}

pub fn is_translocated_url(path: CFURL) -> Result<bool, CFError> {
    let path_ref = path.as_concrete_TypeRef();

    let mut is_translocated = false;
    let mut error_ref = null_mut();
    let ok =
        unsafe { SecTranslocateIsTranslocatedURL(path_ref, &mut is_translocated, &mut error_ref) };

    let is_translocated_ref = if ok { &is_translocated } else { null() };
    from_bool_and_error_to_result(is_translocated_ref, error_ref)
}
