#![allow(clippy::needless_doctest_main)]

//! This is a confortable wrapper arround standard ways of
//! declaring lazily initialized types.
//!
//! # Examples
//! ## Using the [DelayInit] struct
//! ```
//! use delay_init::*;
//! use std::collections::HashMap;
//!
//! static NUMBERS : DelayInit<HashMap<i32,i32>> = DelayInit::new(|| {
//!     let mut map = HashMap::new();
//!     map.insert(12,12);
//!     map.insert(13,13);
//!     map
//! });
//!
//! fn main() {
//!     println!("{}", NUMBERS.get(&12).unwrap());
//! }
//! ```
//!
//! ## Using the [delay] macro
//! ```
//! use delay_init::*;
//! use std::collections::HashMap;
//!
//! delay! {
//!     static NUMBERS : HashMap<i32,i32> = {
//!         let mut map = HashMap::new();
//!         map.insert(12,12);
//!         map.insert(13,13);
//!         map
//!     };
//! }
//!
//! fn main() {
//!     println!("{}", NUMBERS.get(&12).unwrap());
//! }
//! ```

#![no_std]

use core::ops::Deref;
use core::marker::Sync;

#[cfg(feature = "no_std")]
mod singleton {
    use spin::Once;
    use core::ops::FnOnce;
    use core::marker::Sync;

    pub struct Singleton<T: Sync>(Once<T>);

    impl<T: Sync> Singleton<T> {
        pub const fn new() -> Self { Self(Once::new()) }
        pub fn get_instance(&self, f: impl FnOnce() -> T) -> &T {
            self.0.call_once(f)
        }
    }
}


#[cfg(not(feature = "no_std"))]
mod singleton {
    extern crate std;
    use std::sync::OnceLock;
    use core::ops::FnOnce;
    use core::marker::Sync;

    pub struct Singleton<T: Sync>(OnceLock<T>);

    impl<T: Sync> Singleton<T> {
        pub const fn new() -> Self { Self(OnceLock::new()) }
        pub fn get_instance(&self, f: impl FnOnce() -> T) -> &T {
            self.0.get_or_init(f)
        }
    }
}

use singleton::Singleton;

pub struct DelayInit<T: Sync, F = fn() -> T> {
    object: Singleton<T>,
    init: F,
}

impl<T: Sync, F> DelayInit<T, F> {
    pub const fn new(builder: F) -> Self {
        Self {
            object: Singleton::new(),
            init: builder,
        }
    }
}

impl<T: Sync> Deref for DelayInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.object.get_instance(self.init)
    }
}

#[macro_export]
macro_rules! delay {
    ( $( $v:vis static $name:ident : $type:ty = $e:expr ; )* ) => {
        $(
            $v static $name : $crate::DelayInit<$type> = $crate::DelayInit::new(|| { $e });
        )*
    };
}
