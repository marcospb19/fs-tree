use crate::{Classes, ModeBits};

use std::{
    fs,
    os::{raw::c_int, unix::fs::PermissionsExt},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct PermissionBits {
    pub owner_bits: ModeBits,
    pub group_bits: ModeBits,
    pub other_bits: ModeBits,
}

impl PermissionBits {
    /// Function to build new PermissionBits with each field.
    pub fn new(owner_bits: ModeBits, group_bits: ModeBits, other_bits: ModeBits) -> Self {
        PermissionBits {
            owner_bits,
            group_bits,
            other_bits,
        }
    }

    /// Constructs from bits with checks.
    ///
    /// Returns None if there are extra bits set, other than the last 9 bits
    /// expected (3 for the mode of each class)
    pub fn from_bits_checked(bits: c_int) -> Option<Self> {
        // Check if there's more than 0o777
        if bits & !0o777 != 0 {
            None
        } else {
            Some(PermissionBits::from_bits(bits))
        }
    }

    /// Constructs from bits without checks.
    pub fn from_bits(bits: c_int) -> Self {
        let owner: c_int = bits & 0o700;
        let group: c_int = bits & 0o070;
        let other: c_int = bits & 0o007;

        // Shift everything to the last 3 bits
        let owner = ModeBits::from_bits(owner >> 3 >> 3).unwrap();
        let group = ModeBits::from_bits(group >> 3).unwrap();
        let other = ModeBits::from_bits(other).unwrap();

        PermissionBits::new(owner, group, other)
    }

    pub const fn empty() -> Self {
        PermissionBits {
            owner_bits: ModeBits::empty(),
            group_bits: ModeBits::empty(),
            other_bits: ModeBits::empty(),
        }
    }

    pub const fn all() -> Self {
        PermissionBits {
            owner_bits: ModeBits::all(),
            group_bits: ModeBits::all(),
            other_bits: ModeBits::all(),
        }
    }

    /// improve this help lalalalalalala
    /// Prepare a 0o777 style from 3 0b111 style lalala
    pub fn bits(&self) -> c_int {
        let mut result = 0;
        result |= self.owner_bits.bits() << 3 << 3;
        result |= self.group_bits.bits() << 3;
        result |= self.other_bits.bits();
        result
    }

    pub const fn is_empty(&self) -> bool {
        self.owner_bits.is_empty() && self.group_bits.is_empty() && self.other_bits.is_empty()
    }

    pub const fn is_all(&self) -> bool {
        self.owner_bits.is_all() && self.group_bits.is_all() && self.other_bits.is_all()
    }

    pub fn intersects(&self, other: PermissionBits) -> bool {
        self.owner_bits.intersects(other.owner_bits)
            && self.group_bits.intersects(other.group_bits)
            && self.other_bits.intersects(other.other_bits)
    }

    pub fn contains(&self, other: PermissionBits) -> bool {
        self.owner_bits.contains(other.owner_bits)
            && self.group_bits.contains(other.group_bits)
            && self.other_bits.contains(other.other_bits)
    }

    pub fn insert(&mut self, classes: Classes, bits: ModeBits) -> Self {
        if classes.is_owner_set() {
            self.owner_bits.insert(bits);
        }
        if classes.is_group_set() {
            self.group_bits.insert(bits);
        }
        if classes.is_other_set() {
            self.other_bits.insert(bits);
        }
        *self
    }

    pub fn remove(&mut self, classes: Classes, bits: ModeBits) -> Self {
        if classes.is_owner_set() {
            self.owner_bits.remove(bits);
        }
        if classes.is_group_set() {
            self.group_bits.remove(bits);
        }
        if classes.is_other_set() {
            self.other_bits.remove(bits);
        }
        *self
    }

    pub fn set(&mut self, classes: Classes, bits: ModeBits, value: bool) -> Self {
        if classes.is_owner_set() {
            self.owner_bits.set(bits, value);
        }
        if classes.is_group_set() {
            self.group_bits.set(bits, value);
        }
        if classes.is_other_set() {
            self.other_bits.set(bits, value);
        }
        *self
    }
}

impl From<fs::Permissions> for PermissionBits {
    fn from(fs_permissions: fs::Permissions) -> Self {
        // Is this cast safe?
        PermissionBits::from_bits(fs_permissions.mode() as c_int)
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, PermissionBits};
    #[test]
    fn test_permissions_bits_set_method() {
        let permissions_bits = PermissionBits::from_bits(0o000)
            .set(OWNER, READ, true)
            .set(OWNER | GROUP, WRITE, true)
            .set(OTHER, ALL_BITS, true)
            .set(OTHER, READ | WRITE, false);

        assert_eq!(permissions_bits.bits(), 0o621);
    }
}

// #[allow(unused_imports)]
// use super::{
//     Classes::{AllClasses, Group, Other, Owner},
//     ModeBits::{AllBits, Execute, Null, Read, Write},
//     PermissionBits,
// };

// use std::fs::File;

// let file: std::fs::File = File::open("foo.txt")?;
// // Troca isso por um wrapper
// // let mut perms = file.metadata()?.permissions();
// let permissions = PermissionBits::try_from(file)?;

// if permissions.get(Owner, Execute) {
//     println!("Owner of file can execute");
// }

// if permissions.get(Other, Read | Write) {
//     println!("Other users can read and write to this file!");
// }

// // Let's change permissions with set and unset
// let permissions = permissions
//     .unset(Owner, Execute)
//     .set(Group, Read | Write)
//     .reset(Other, Read);

// perms.set_readonly(true);
// file.set_permissions(perms)?;
