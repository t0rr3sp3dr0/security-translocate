use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::result::from_url_and_error_to_result;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::null_mut;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L125

    fn SecTranslocateCreateGeneric(
        path_to_translocate: _Nonnull<CFURLRef>,
        destination_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.16)
}

pub fn create_generic(
    path_to_translocate: CFURL,
    destination_path: CFURL,
) -> Result<CFURL, CFError> {
    let path_to_translocate_ref = path_to_translocate.as_concrete_TypeRef();
    let destination_path_ref = destination_path.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let url_ref = unsafe {
        SecTranslocateCreateGeneric(
            path_to_translocate_ref,
            destination_path_ref,
            &mut error_ref,
        )
    };

    from_url_and_error_to_result(url_ref, error_ref)
}
