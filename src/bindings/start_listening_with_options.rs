use crate::internal::annotations::{_Nonnull, _Nullable};
use crate::internal::option::from_bool_and_error_to_option;
use core_foundation::base::TCFType;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::error::{CFError, CFErrorRef};
use std::ptr::null_mut;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L57

    fn SecTranslocateStartListeningWithOptions(
        options: _Nonnull<CFDictionaryRef>,
        out_error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)
}

pub fn start_listening_with_options(options: CFDictionary) -> Option<CFError> {
    let options_ref = options.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateStartListeningWithOptions(options_ref, &mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}
