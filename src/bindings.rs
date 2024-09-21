use crate::option::{SomeCFErrorRefUnlessOk, ToOption as _};
use crate::result::{FromOptionsWithBool, FromOptionsWithCFTypeRef};
use core_foundation::base::TCFType as _;
use core_foundation::dictionary::CFDictionary;
use core_foundation::error::CFError;
use core_foundation::url::CFURL;
use libc::pid_t;
use security_translocate_sys::{
    SecTranslocateAppLaunchCheckin, SecTranslocateCreateGeneric,
    SecTranslocateCreateOriginalPathForURL, SecTranslocateCreateSecureDirectoryForURL,
    SecTranslocateDeleteSecureDirectory, SecTranslocateIsTranslocatedURL,
    SecTranslocateStartListening, SecTranslocateStartListeningWithOptions,
    SecTranslocateURLShouldRunTranslocated,
};
use std::ptr::{null, null_mut};

/// Initializes the SecTranslocate Library as the XPC Server, the Disk Arbitration Listener, and the Launch Services Notification Listener.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateStartListening.html>
pub fn start_listening() -> Option<CFError> {
    let mut error = null_mut();
    let ok = unsafe { SecTranslocateStartListening(&mut error) };

    let error = error.to_option();
    Option::some_cf_error_ref_unless_ok(ok, error)
}

/// Initializes the SecTranslocate Library as the XPC Server, the Disk Arbitration Listener, and the Launch Services Notification Listener.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateStartListeningWithOptions.html>
pub fn start_listening_with_options(options: CFDictionary) -> Option<CFError> {
    let options = options.as_concrete_TypeRef();

    let mut error = null_mut();
    let ok = unsafe { SecTranslocateStartListeningWithOptions(options, &mut error) };

    let error = error.to_option();
    Option::some_cf_error_ref_unless_ok(ok, error)
}

/// Translocates the directory specified by `path_to_translocate`.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateSecureDirectoryForURL.html>
pub fn create_secure_directory_for_url(
    path_to_translocate: CFURL,
    destination_path: Option<CFURL>,
) -> Result<CFURL, CFError> {
    let path_to_translocate = path_to_translocate.as_concrete_TypeRef();
    let destination_path = match destination_path {
        Some(destination_path) => destination_path.as_concrete_TypeRef(),
        None => null(),
    };

    let mut error = null_mut();
    let url = unsafe {
        SecTranslocateCreateSecureDirectoryForURL(path_to_translocate, destination_path, &mut error)
    };

    let url = url.to_option();
    let error = error.to_option();
    Result::from_options_with_cf_type_ref(url, error)
}

/// Translocates the directory specified by `path_to_translocate`.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateGeneric.html>
pub fn create_generic(
    path_to_translocate: CFURL,
    destination_path: CFURL,
) -> Result<CFURL, CFError> {
    let path_to_translocate = path_to_translocate.as_concrete_TypeRef();
    let destination_path = destination_path.as_concrete_TypeRef();

    let mut error = null_mut();
    let url =
        unsafe { SecTranslocateCreateGeneric(path_to_translocate, destination_path, &mut error) };

    let url = url.to_option();
    let error = error.to_option();
    Result::from_options_with_cf_type_ref(url, error)
}

/// Registers that a translocated pid is running.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateAppLaunchCheckin.html>
pub fn app_launch_checkin(pid: pid_t) {
    unsafe { SecTranslocateAppLaunchCheckin(pid) }
}

/// Indicates whether the entity defined by path should be run translocated.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateURLShouldRunTranslocated.html>
pub fn url_should_run_translocated(path: CFURL) -> Result<bool, CFError> {
    let path = path.as_concrete_TypeRef();

    let mut should_translocate = false;
    let mut error = null_mut();
    let ok = unsafe {
        SecTranslocateURLShouldRunTranslocated(path, &mut should_translocate, &mut error)
    };

    let should_translocate = if ok { Some(should_translocate) } else { None };
    let error = error.to_option();
    Result::from_options_with_bool(should_translocate, error)
}

/// Indicates if the provided path is a translocated path.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateIsTranslocatedURL.html>
pub fn is_translocated_url(path: CFURL) -> Result<bool, CFError> {
    let path = path.as_concrete_TypeRef();

    let mut is_translocated = false;
    let mut error = null_mut();
    let ok = unsafe { SecTranslocateIsTranslocatedURL(path, &mut is_translocated, &mut error) };

    let is_translocated = if ok { Some(is_translocated) } else { None };
    let error = error.to_option();
    Result::from_options_with_bool(is_translocated, error)
}

/// Finds the original path to a file given a translocated path.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateOriginalPathForURL.html>
pub fn create_original_path_for_url(translocated_path: CFURL) -> Result<CFURL, CFError> {
    let translocated_path = translocated_path.as_concrete_TypeRef();

    let mut error = null_mut();
    let url = unsafe { SecTranslocateCreateOriginalPathForURL(translocated_path, &mut error) };

    let url = url.to_option();
    let error = error.to_option();
    Result::from_options_with_cf_type_ref(url, error)
}

/// Unmount the translocated directory structure and delete the mount point directory.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateDeleteSecureDirectory.html>
pub fn delete_secure_directory(translocated_path: CFURL) -> Option<CFError> {
    let translocated_path = translocated_path.as_concrete_TypeRef();

    let mut error = null_mut();
    let ok = unsafe { SecTranslocateDeleteSecureDirectory(translocated_path, &mut error) };

    let error = error.to_option();
    Option::some_cf_error_ref_unless_ok(ok, error)
}
