use core_foundation::base::{TCFType, TCFTypeRef};
use core_foundation::error::{CFError, CFErrorRef};

pub(crate) trait FromOptionsWithBool {
    fn from_options_with_bool(
        bool_option: Option<bool>,
        cf_error_ref_option: Option<CFErrorRef>,
    ) -> Self;
}

impl FromOptionsWithBool for Result<bool, CFError> {
    fn from_options_with_bool(
        bool_option: Option<bool>,
        cf_error_ref_option: Option<CFErrorRef>,
    ) -> Self {
        match bool_option {
            Some(bool) => Ok(bool),

            None => {
                let cf_error_ref = cf_error_ref_option.unwrap();
                let cf_error = unsafe { CFError::wrap_under_create_rule(cf_error_ref) };

                Err(cf_error)
            }
        }
    }
}

pub(crate) trait FromOptionsWithCFTypeRef<T: TCFTypeRef> {
    fn from_options_with_cf_type_ref(
        cf_type_ref_option: Option<T>,
        cf_error_ref_option: Option<CFErrorRef>,
    ) -> Self;
}

impl<T: TCFType> FromOptionsWithCFTypeRef<T::Ref> for Result<T, CFError> {
    fn from_options_with_cf_type_ref(
        cf_type_ref_option: Option<T::Ref>,
        cf_error_ref_option: Option<CFErrorRef>,
    ) -> Self {
        match cf_type_ref_option {
            Some(cf_type_ref) => {
                let cf_type = unsafe { T::wrap_under_create_rule(cf_type_ref) };

                Ok(cf_type)
            }

            None => {
                let cf_error_ref = cf_error_ref_option.unwrap();
                let cf_error = unsafe { CFError::wrap_under_create_rule(cf_error_ref) };

                Err(cf_error)
            }
        }
    }
}
