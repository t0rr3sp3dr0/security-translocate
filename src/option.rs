use core_foundation::base::TCFType as _;
use core_foundation::error::{CFError, CFErrorRef};

pub(crate) trait SomeCFErrorRefUnlessOk {
    fn some_cf_error_ref_unless_ok(ok: bool, cf_error_ref_option: Option<CFErrorRef>) -> Self;
}

impl SomeCFErrorRefUnlessOk for Option<CFError> {
    fn some_cf_error_ref_unless_ok(ok: bool, cf_error_ref_option: Option<CFErrorRef>) -> Self {
        if ok {
            None
        } else {
            let cf_error_ref = cf_error_ref_option.unwrap();
            let cf_error = unsafe { CFError::wrap_under_create_rule(cf_error_ref) };

            Some(cf_error)
        }
    }
}

pub(crate) trait ToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl<T> ToOption<*const T> for *const T {
    fn to_option(self) -> Option<*const T> {
        if !self.is_null() {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> ToOption<*mut T> for *mut T {
    fn to_option(self) -> Option<*mut T> {
        if !self.is_null() {
            Some(self)
        } else {
            None
        }
    }
}
