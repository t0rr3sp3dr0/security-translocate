use crate::internal::annotations::_Nullable;
use core_foundation::base::TCFType;
use core_foundation::error::{CFError, CFErrorRef};
use core_foundation::url::{CFURLRef, CFURL};

pub(crate) fn from_bool_and_error_to_result(
    bool_ref: _Nullable<*const bool>,
    error_ref: _Nullable<CFErrorRef>,
) -> Result<bool, CFError> {
    if !bool_ref.is_null() {
        let bool = unsafe { bool_ref.read() };
        Ok(bool)
    } else {
        let error = unsafe { TCFType::wrap_under_create_rule(error_ref) };
        Err(error)
    }
}

pub(crate) fn from_url_and_error_to_result(
    url_ref: _Nullable<CFURLRef>,
    error_ref: _Nullable<CFErrorRef>,
) -> Result<CFURL, CFError> {
    if !url_ref.is_null() {
        let url = unsafe { TCFType::wrap_under_create_rule(url_ref) };
        Ok(url)
    } else {
        let error = unsafe { TCFType::wrap_under_create_rule(error_ref) };
        Err(error)
    }
}
