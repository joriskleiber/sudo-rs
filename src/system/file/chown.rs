use std::{fs::File, io, os::unix::fs};

use crate::system::interface::{GroupId, UserId};

pub(crate) trait Chown {
    fn chown(&self, uid: UserId, gid: GroupId) -> io::Result<()>;
}

impl Chown for File {
    fn chown(&self, owner: UserId, group: GroupId) -> io::Result<()> {
        fs::fchown(self, Some(owner.inner()), Some(group.inner()))
    }
}
