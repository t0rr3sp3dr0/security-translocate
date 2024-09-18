use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::result::from_url_and_error_to_result;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::null_mut;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L215

    fn SecTranslocateCreateOriginalPathForURL(
        translocated_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.12)
}

pub fn create_original_path_for_url(translocated_path: CFURL) -> Result<CFURL, CFError> {
    let translocated_path_ref = translocated_path.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let url_ref =
        unsafe { SecTranslocateCreateOriginalPathForURL(translocated_path_ref, &mut error_ref) };

    from_url_and_error_to_result(url_ref, error_ref)
}
