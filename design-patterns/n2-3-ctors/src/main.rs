/// Time in seconds.

/// Option 1 for Default values
/// # Example
///
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
// pub struct Second {
//     value: u64,
// }
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
// impl Default for Second {
//     fn default() -> Self {
//         Self { value: 0 }
//     }
// }

/// Option 2 for Default values
#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    // Returns the value in seconds
    pub fn value(&self) -> u64 {
        self.value
    }
}

fn main() {
    println!("{}", "hello");
    let s_new = Second::new(42);
    assert_eq!(42, s_new.value());
    let s_default = Second::default();
    assert_eq!(0, s_default.value());
}
