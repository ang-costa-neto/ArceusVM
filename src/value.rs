use std::fmt;

/// NaN-Boxing Strategy:
/// ArceusVM represents every variable (number, boolean, nil, pointer) 
/// using a single 64-bit 'u64' cell. This maximizes cache efficiency
/// and minimizes memory usage.
///
/// We hijack the unused 'Quiet NaN' space in the IEEE 754 float
/// definition (51 bits of payload) to store other types.
///
/// Pattern (Sign, Exponent, Payload):
/// Standard Float: Any pattern *not* matching a Quiet NaN.
/// Tagged Nil:    [0111...1111 1100...] + [1] = 0x7ffc000000000001
/// Tagged False:  [0111...1111 1100...] + [2] = 0x7ffc000000000002
/// Tagged True:   [0111...1111 1100...] + [3] = 0x7ffc000000000003
/// Tagged Object: [1111...1111 1100...] + [48-bit pointer] 

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)] // Guarantees that Value is just a u64 in memory
pub struct Value(u64);

// The base bitmask for a Quiet NaN (all 11 exponent bits are set, and the QNaN bit is 1).
const QNAN_BASE: u64 = 0x7ffc000000000000;
// Bitmask for the sign bit (MSB).
const SIGN_BIT: u64 = 0x8000000000000000;

// Internal type tags stored in the lowest bits of the NaN payload.
const TAG_NIL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;
// Pointers use the sign bit to allow 48-bit addresses (standard on modern CPUs).

impl Value {
    // --- Floating Point Numbers (f64) ---

    /// Creates a new Value from a 64-bit float.
    /// This is a zero-cost abstraction using 'transmute'.
    #[inline(always)]
    pub fn number(num: f64) -> Self {
        // Safe: Transmuting between 64-bit types is valid in Rust.
        Value(unsafe { std::mem::transmute::<f64, u64>(num) })
    }

    /// Checks if the Value is representing a standard float.
    #[inline(always)]
    pub fn is_number(&self) -> bool {
        // If the Quiet NaN bits are not all present, it's a number.
        (self.0 & QNAN_BASE) != QNAN_BASE
    }

    /// Unboxes the Value into an f64.
    /// WARNING: Assumes is_number() is true. Performance-critical, uses transmute.
    #[inline(always)]
    pub fn as_number(&self) -> f64 {
        debug_assert!(self.is_number(), "Attempted to unbox non-number Value as f64");
        // Safe: Direct memory cast, assumes is_number() already passed.
        unsafe { std::mem::transmute::<u64, f64>(self.0) }
    }

    // --- Singleton Types (Nil, Booleans) ---

    /// Creates a new 'Nil' Value.
    #[inline(always)]
    pub fn nil() -> Self {
        Value(QNAN_BASE | TAG_NIL)
    }

    /// Checks if the Value is Nil.
    #[inline(always)]
    pub fn is_nil(&self) -> bool {
        self.0 == (QNAN_BASE | TAG_NIL)
    }

    /// Creates a new 'Boolean' Value.
    #[inline(always)]
    pub fn boolean(b: bool) -> Self {
        if b {
            Value(QNAN_BASE | TAG_TRUE)
        } else {
            Value(QNAN_BASE | TAG_FALSE)
        }
    }

    /// Checks if the Value is a Boolean.
    #[inline(always)]
    pub fn is_boolean(&self) -> bool {
        (self.0 | 1) == (QNAN_BASE | TAG_TRUE)
    }

    /// Unboxes the Value into a bool.
    /// WARNING: Assumes is_boolean() is true.
    #[inline(always)]
    pub fn as_boolean(&self) -> bool {
        debug_assert!(self.is_boolean(), "Attempted to unbox non-boolean Value as bool");
        self.0 == (QNAN_BASE | TAG_TRUE)
    }

    // --- Advanced: Pointers (Planned) ---

    /// Creates a new 'Object' Value from a raw pointer.
    /// Uses the sign bit and the payload to store a 48-bit address.
    #[inline(always)]
    pub fn obj(ptr: *mut ()) -> Self {
        // SIGN_BIT | QNAN_BASE is the tag for pointers.
        Value(SIGN_BIT | QNAN_BASE | (ptr as u64))
    }

    /// Checks if the Value is a pointer to an heap object.
    #[inline(always)]
    pub fn is_obj(&self) -> bool {
        // Both sign bit and QNAN bits must be present.
        (self.0 & (SIGN_BIT | QNAN_BASE)) == (SIGN_BIT | QNAN_BASE)
    }

    /// Returns the raw pointer address.
    #[inline(always)]
    pub fn as_obj(&self) -> *mut () {
        debug_assert!(self.is_obj(), "Attempted to unbox non-object Value as pointer");
        // Extract only the address part by masking out our tags.
        (self.0 & !(SIGN_BIT | QNAN_BASE)) as *mut ()
    }

    // --- Utilities ---

    /// Internal raw value access, for core VM use only.
    #[inline(always)]
    pub fn raw(&self) -> u64 {
        self.0
    }
}

// Custom Debug implementation to show human-readable format,
// which is essential for VM debugging.
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_number() {
            write!(f, "f64({})", self.as_number())
        } else if self.is_nil() {
            write!(f, "nil")
        } else if self.is_boolean() {
            write!(f, "bool({})", self.as_boolean())
        } else if self.is_obj() {
            write!(f, "obj_ptr({:p})", self.as_obj())
        } else {
            // Should never happen if implementation is correct.
            write!(f, "invalid_value({:#018x})", self.0)
        }
    }
}

// Display is used for the user-facing print() function in the VM.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            v if v.is_number() => write!(f, "{}", v.as_number()),
            v if v.is_nil() => write!(f, "nil"),
            v if v.is_boolean() => write!(f, "{}", v.as_boolean()),
            v if v.is_obj() => write!(f, "object"), // Will be expanded later.
            _ => unreachable!(),
        }
    }
}