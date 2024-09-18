use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::result::from_url_and_error_to_result;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::{null, null_mut};

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L98

    fn SecTranslocateCreateSecureDirectoryForURL(
        path_to_translocate: _Nonnull<CFURLRef>,
        destination_path: _Nullable<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.12)
}

pub fn create_secure_directory_for_url(
    path_to_translocate: CFURL,
    destination_path: Option<CFURL>,
) -> Result<CFURL, CFError> {
    let path_to_translocate_ref = path_to_translocate.as_concrete_TypeRef();
    let destination_path_ref = match destination_path {
        Some(url) => url.as_concrete_TypeRef(),

        None => null(),
    };

    let mut error_ref = null_mut();
    let url_ref = unsafe {
        SecTranslocateCreateSecureDirectoryForURL(
            path_to_translocate_ref,
            destination_path_ref,
            &mut error_ref,
        )
    };

    from_url_and_error_to_result(url_ref, error_ref)
}
