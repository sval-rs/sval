#[cfg(feature = "serde")]
#[macro_export]
#[doc(hidden)]
macro_rules! derive_from_serde {
    (if #[cfg(feature = "serde")] { $($with:tt)* } else { $($without:tt)* } ) => {
        $($with)*
    };
}

#[cfg(not(feature = "serde"))]
#[macro_export]
#[doc(hidden)]
macro_rules! derive_from_serde {
    (if #[cfg(feature = "serde")] { $($with:tt)* } else { $($without:tt)* } ) => {
        $($without)*
    };
}
