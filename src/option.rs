use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};

pub(crate) fn from_bool_and_error_to_option(ok: bool, error_ref: CFErrorRef) -> Option<CFError> {
    if ok {
        None
    } else {
        let error = unsafe { TCFType::wrap_under_create_rule(error_ref) };
        Some(error)
    }
}
