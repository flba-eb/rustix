//! User and Group ID types.

#![allow(unsafe_code)]

use crate::backend::c;

/// A group identifier as a raw integer.
#[cfg(not(target_os = "wasi"))]
pub type RawGid = c::gid_t;
/// A user identifier as a raw integer.
#[cfg(not(target_os = "wasi"))]
pub type RawUid = c::uid_t;

/// `uid_t`—A Unix user ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Uid(RawUid);

/// `gid_t`—A Unix group ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Gid(RawGid);

impl Uid {
    /// A `Uid` corresponding to the root user (uid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawUid` into a `Uid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix user ID.
    #[inline]
    pub const unsafe fn from_raw(raw: RawUid) -> Self {
        Self(raw)
    }

    /// Converts a `Uid` into a `RawUid`.
    #[inline]
    pub const fn as_raw(self) -> RawUid {
        self.0
    }

    /// Test whether this uid represents the root user (uid 0).
    #[inline]
    pub const fn is_root(self) -> bool {
        self.0 == Self::ROOT.0
    }
}

impl Gid {
    /// A `Gid` corresponding to the root group (gid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawGid` into a `Gid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix group ID.
    #[inline]
    pub const unsafe fn from_raw(raw: RawGid) -> Self {
        Self(raw)
    }

    /// Converts a `Gid` into a `RawGid`.
    #[inline]
    pub const fn as_raw(self) -> RawGid {
        self.0
    }

    /// Test whether this gid represents the root group (gid 0).
    #[inline]
    pub const fn is_root(self) -> bool {
        self.0 == Self::ROOT.0
    }
}

// Return the raw value of the IDs. In case of `None` it returns `u32::MAX`
// since it has the same bit pattern as `-1` indicating no change to the
// owner/group ID.
// QNX Neutrino (nto) uses i32 values, but only positive values should be used, see e.g.:
// https://www.qnx.com/developers/docs/7.1/#com.qnx.doc.neutrino.lib_ref/topic/s/setuid.html
pub(crate) fn translate_fchown_args(owner: Option<Uid>, group: Option<Gid>) -> (u32, u32) {
    let ow = match owner {
        #[cfg(not(target_os = "nto"))]
        Some(o) => o.as_raw(),
        #[cfg(target_os = "nto")]
        Some(o) => o.as_raw() as u32,
        None => u32::MAX,
    };

    let gr = match group {
        #[cfg(not(target_os = "nto"))]
        Some(g) => g.as_raw(),
        #[cfg(target_os = "nto")]
        Some(g) => g.as_raw() as u32,
        None => u32::MAX,
    };

    (ow, gr)
}
