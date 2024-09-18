use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::option::from_bool_and_error_to_option;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};
use std::ptr::null_mut;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L235

    fn SecTranslocateDeleteSecureDirectory(
        translocated_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)
}

pub fn delete_secure_directory(translocated_path: CFURL) -> Option<CFError> {
    let translocated_path_ref = translocated_path.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateDeleteSecureDirectory(translocated_path_ref, &mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}
