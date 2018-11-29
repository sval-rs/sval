macro_rules! cfg_debug_stack {
    (if #[debug_assertions] { $($with:tt)* }) => {
        #[cfg(debug_assertions)]
        {
            $($with)*
        }
    };
    (if #[debug_assertions] { $($with:tt)* } else { $($without:tt)* }) => {
        #[cfg(debug_assertions)]
        {
            $($with)*
        }
        #[cfg(not(debug_assertions))]
        {
            $($without)*
        }
    };
}
