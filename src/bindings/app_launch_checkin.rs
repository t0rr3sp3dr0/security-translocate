use libc::pid_t;

#[link(name = "Security", kind = "framework")]
extern "C" {
    // https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L139

    fn SecTranslocateAppLaunchCheckin(pid: pid_t); // __OSX_AVAILABLE(10.12)
}

pub fn app_launch_checkin(pid: pid_t) {
    unsafe { SecTranslocateAppLaunchCheckin(pid) }
}
