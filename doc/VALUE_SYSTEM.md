# 💎 ArceusVM: Value System (NaN-Boxing)

In high-performance Virtual Machines (like Google's V8 or LuaJIT), the representation of data in the stack is a critical bottleneck. ArceusVM utilizes a technique called **NaN-Boxing** to ensure that every value—be it a number, boolean, or pointer—occupies exactly **8 bytes (64 bits)**.

---

## 1. The Core Concept: IEEE 754 Exploitation

A 64-bit double-precision floating-point number (IEEE 754) is structured as:
* **1 bit** for the sign.
* **11 bits** for the exponent.
* **52 bits** for the fraction (mantissa).

When all bits in the exponent are set to `1`, the value is considered **NaN (Not-a-Number)**. In standard hardware, if the highest bit of the mantissa is also `1` (the "quiet bit"), the remaining **51 bits** are ignored by the CPU. 

**ArceusVM "hijacks" these 51 bits to store non-float data.**

## 2. Bitmask Strategy

We define a specific bit pattern to distinguish between a real `f64` and our "boxed" types.

| Type | Bit Pattern (Hex) | Description |
| :--- | :--- | :--- |
| **Double** | `<Any non-NaN pattern>` | Standard 64-bit float. |
| **Quiet NaN Prefix**| `0x7ffc000000000000` | Our base mask for all boxed values. |
| **Nil** | `...0001` | Represented as a specific NaN payload. |
| **False** | `...0002` | Boolean false. |
| **True** | `...0003` | Boolean true. |
| **Object (Pointer)** | `0xfffc0000 + [32/48-bit addr]` | Sign bit + QNaN + Pointer address. |

## 3. Implementation Approach

We use a **Newtype** pattern in Rust to wrap a `u64`. This provides a type-safe interface for bit-level manipulation while avoiding the overhead of Rust's `enum` tagging (which would double our memory footprint to 16 bytes).

```rust
pub struct Value(u64);

const QNAN: u64 = 0x7ffc000000000000;
const SIGN_BIT: u64 = 0x8000000000000000;

// Internal Tags
const TAG_NIL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;

impl Value {
    #[inline(always)]
    pub fn is_number(&self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    #[inline(always)]
    pub fn as_number(&self) -> f64 {
        unsafe { std::mem::transmute(self.0) }
    }

    #[inline(always)]
    pub fn is_obj(&self) -> bool {
        (self.0 & (SIGN_BIT | QNAN)) == (SIGN_BIT | QNAN)
    }
}
```

## 4. Engineering Trade-offs
### Pros (Why we did this)
1. Cache Efficiency: By keeping every `Value` at 8 bytes, we maximize the number of values that fit into the L1 Data Cache.
2. No Tag Overhead: Traditional tagged unions require an extra byte (or 8 due to padding) to store the type. NaN-boxing uses the "free" space within the float itself.
3. Fast Dispatch: Checking if a value is a number requires a single bitwise `AND` and a comparison, which is extremely branch-predictor friendly.

### Cons (The risks)
1. Complexity: Manual bit manipulation is error-prone and requires rigorous unit testing.

2. Pointer Limits: Since we use part of the 64 bits for the NaN prefix, we are limited to *48-bit addressing* for pointers (which is the actual limit on modern x86_64 and ARM64 architectures anyway).

## 5. Performance Metrics (Theoretical)
By switching from a 16-byte `enum` to an 8-byte `NaN-Boxed` value, we expect:
- *50% reduction* in stack memory usage.
- *~15-20% increase* in execution speed for arithmetic-heavy code due to reduced memory pressure and better cache utilization.