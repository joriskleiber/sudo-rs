#![cfg(test)]

use core::fmt;

#[macro_use]
mod macros;

mod helpers;
mod su;
mod sudo;
mod sudoedit;
mod visudo;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>;

const OTHER_USERNAME: &str = "ghost";
const USERNAME: &str = "ferris";
const GROUPNAME: &str = "rustaceans";
const PASSWORD: &str = "strong-password";
const HOSTNAME: &str = "container";
// 64 characters
const LONGEST_HOSTNAME: &str = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";

const SUDOERS_ROOT_ALL: &str = "root    ALL=(ALL:ALL) ALL";
const SUDOERS_ALL_ALL_NOPASSWD: &str = "ALL ALL=(ALL:ALL) NOPASSWD: ALL";
const SUDOERS_ROOT_ALL_NOPASSWD: &str = "root ALL=(ALL:ALL) NOPASSWD: ALL";
const SUDOERS_USER_ALL_NOPASSWD: &str = "ferris ALL=(ALL:ALL) NOPASSWD: ALL";
const SUDOERS_USER_ALL_ALL: &str = "ferris ALL=(ALL:ALL) ALL";
const SUDOERS_NO_LECTURE: &str = "Defaults lecture=\"never\"";
const SUDOERS_ONCE_LECTURE: &str = "Defaults lecture=\"once\"";
const SUDOERS_ALWAYS_LECTURE: &str = "Defaults lecture=\"always\"";
const SUDOERS_NEW_LECTURE: &str = "Defaults lecture_file = \"/etc/sudo_lecture\"";
const SUDOERS_NEW_LECTURE_USER: &str = "Defaults:ferris lecture_file = \"/etc/sudo_lecture\"";
const PAMD_SUDO_PAM_PERMIT: &str = "auth sufficient pam_permit.so";

const OG_SUDO_STANDARD_LECTURE: &str= "\nWe trust you have received the usual lecture from the local System\nAdministrator. It usually boils down to these three things:\n\n    #1) Respect the privacy of others.\n    #2) Think before you type.\n    #3) With great power comes great responsibility.";

const SUDO_RS_IS_UNSTABLE: &str =
    "SUDO_RS_IS_UNSTABLE=I accept that my system may break unexpectedly";

const SUDO_ENV_DEFAULT_PATH: &str = "/usr/bin:/bin:/usr/sbin:/sbin";
const SUDO_ENV_DEFAULT_TERM: &str = "unknown";

const SUDOERS_USE_PTY: &str = "Defaults use_pty";
const SUDOERS_NOT_USE_PTY: &str = "Defaults !use_pty";

const ENV_PATH: &str = "/usr/bin/env";

#[cfg(not(target_os = "freebsd"))]
const DEFAULT_EDITOR: &str = "/usr/bin/editor";
#[cfg(target_os = "freebsd")]
const DEFAULT_EDITOR: &str = "/usr/bin/vi";

const PANIC_EXIT_CODE: i32 = 101;

enum EnvList {
    Check,
    Keep,
}

impl fmt::Display for EnvList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnvList::Check => "env_check",
            EnvList::Keep => "env_keep",
        };
        f.write_str(s)
    }
}
