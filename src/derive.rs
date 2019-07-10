#[cfg(feature = "serde_lib")]
#[macro_export]
#[doc(hidden)]
macro_rules! derive_from_serde {
    (if #[cfg(feature = "serde_lib")] { $($with:tt)* } else { $($without:tt)* } ) => {
        $($with)*
    };
}

#[cfg(not(feature = "serde_lib"))]
#[macro_export]
#[doc(hidden)]
macro_rules! derive_from_serde {
    (if #[cfg(feature = "serde_lib")] { $($with:tt)* } else { $($without:tt)* } ) => {
        $($without)*
    };
}
