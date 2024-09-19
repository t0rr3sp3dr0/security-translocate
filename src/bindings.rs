use crate::option::from_bool_and_error_to_option;
use crate::result::{from_bool_and_error_to_result, from_url_and_error_to_result};
use core_foundation::base::TCFType;
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
    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateStartListening(&mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}

/// Initializes the SecTranslocate Library as the XPC Server, the Disk Arbitration Listener, and the Launch Services Notification Listener.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateStartListeningWithOptions.html>
pub fn start_listening_with_options(options: CFDictionary) -> Option<CFError> {
    let options_ref = options.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateStartListeningWithOptions(options_ref, &mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}

/// Translocates the directory specified by `path_to_translocate`.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateSecureDirectoryForURL.html>
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

/// Translocates the directory specified by `path_to_translocate`.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateGeneric.html>
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
    let path_ref = path.as_concrete_TypeRef();

    let mut should_translocate = false;
    let mut error_ref = null_mut();
    let ok = unsafe {
        SecTranslocateURLShouldRunTranslocated(path_ref, &mut should_translocate, &mut error_ref)
    };

    let should_translocate_ref = if ok { &should_translocate } else { null() };
    from_bool_and_error_to_result(should_translocate_ref, error_ref)
}

/// Indicates if the provided path is a translocated path.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateIsTranslocatedURL.html>
pub fn is_translocated_url(path: CFURL) -> Result<bool, CFError> {
    let path_ref = path.as_concrete_TypeRef();

    let mut is_translocated = false;
    let mut error_ref = null_mut();
    let ok =
        unsafe { SecTranslocateIsTranslocatedURL(path_ref, &mut is_translocated, &mut error_ref) };

    let is_translocated_ref = if ok { &is_translocated } else { null() };
    from_bool_and_error_to_result(is_translocated_ref, error_ref)
}

/// Finds the original path to a file given a translocated path.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateCreateOriginalPathForURL.html>
pub fn create_original_path_for_url(translocated_path: CFURL) -> Result<CFURL, CFError> {
    let translocated_path_ref = translocated_path.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let url_ref =
        unsafe { SecTranslocateCreateOriginalPathForURL(translocated_path_ref, &mut error_ref) };

    from_url_and_error_to_result(url_ref, error_ref)
}

/// Unmount the translocated directory structure and delete the mount point directory.
///
/// # Reference
/// <https://docs.rs/security-translocate-sys/0.1.1/security_translocate_sys/fn.SecTranslocateDeleteSecureDirectory.html>
pub fn delete_secure_directory(translocated_path: CFURL) -> Option<CFError> {
    let translocated_path_ref = translocated_path.as_concrete_TypeRef();

    let mut error_ref = null_mut();
    let ok = unsafe { SecTranslocateDeleteSecureDirectory(translocated_path_ref, &mut error_ref) };

    from_bool_and_error_to_option(ok, error_ref)
}
