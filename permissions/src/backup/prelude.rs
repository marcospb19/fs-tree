use crate::{Classes, ModeBits};

/// ModeBits::READ
pub const READ: ModeBits = ModeBits::READ;
/// ModeBits::WRITE
pub const WRITE: ModeBits = ModeBits::WRITE;
/// ModeBits::EXECUTE
pub const EXECUTE: ModeBits = ModeBits::EXECUTE;
/// ModeBits::NULL
pub const NULL: ModeBits = ModeBits::NULL;
/// ModeBits::ALL_BITS
pub const ALL_BITS: ModeBits = ModeBits::ALL_BITS;

/// Classes::OWNER
pub const OWNER: Classes = Classes::OWNER;
/// Classes::GROUP
pub const GROUP: Classes = Classes::GROUP;
/// Classes::OTHER
pub const OTHER: Classes = Classes::OTHER;

use OTHER as Other;
