use std::os::raw::c_int;

bitflags! {
    /// Bits for combination of `rwx` modes (`Read`, `Write` and `Execute`).
    pub struct ModeBits: c_int {
        const NULL    = 0b000;
        const READ    = libc::R_OK;
        const WRITE   = libc::W_OK;
        const EXECUTE = libc::X_OK;
        const ALL_BITS = libc::R_OK | libc::W_OK | libc::X_OK;
    }
}

impl Default for ModeBits {
    fn default() -> Self {
        ModeBits::empty()
    }
}

impl ModeBits {
    pub fn is_read_set(&self) -> bool {
        self.bits() & Self::READ.bits() != 0
    }

    pub fn is_write_set(&self) -> bool {
        self.bits() & Self::WRITE.bits() != 0
    }

    pub fn is_execute_set(&self) -> bool {
        self.bits() & Self::EXECUTE.bits() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mode_bits_impl() {
        let classes = ModeBits::ALL_BITS;
        assert!(classes.is_read_set());
        assert!(classes.is_write_set());
        assert!(classes.is_execute_set());
    }

    #[test]
    fn testando() {}
}

//     #[test]
//     fn test_explicit_variants_against_custom() {
//         assert_eq!(Read, Custom(0b100));
//         assert_eq!(Write, Custom(0b010));
//         assert_eq!(Execute, Custom(0b001));
//         assert_eq!(AllBits, Custom(0b111));
//     }

//     #[test]
//     // Test a | b and creating ModeBits in all the ways possible
//     fn test_bitor_and_all_constructors() {
//         for i in 0..=7 {
//             for j in 0..=7 {
//                 let merged = Custom(i) | Custom(j);
//                 assert_eq!(merged, Custom(i | j));
//                 assert_eq!(merged, ModeBits::from(i | j));
//                 assert_eq!(merged, ModeBits::from(i) | ModeBits::from(j));
//                 assert_eq!(merged.bits(), i | j);

//                 // Test not equal
//                 if i != j {
//                     assert_ne!(Custom(i), Custom(j));
//                     assert_ne!(ModeBits::from(i), ModeBits::from(j));
//                 }
//             }
//         }
//     }

//     #[test]
//     fn test_useless_bitor() {
//         assert_eq!(Read | Read, Read);
//         assert_eq!(Write | Write, Write);
//         assert_eq!(Execute | Execute, Execute);
//         assert_eq!(AllBits | AllBits, AllBits);
//     }
// }
