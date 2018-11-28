macro_rules! cfg_debug_stack {
    (if #[debug_stack] { $($with:tt)* }) => {
        #[cfg(any(debug_assertions, test))]
        {
            $($with)*
        }
    };
    (if #[debug_stack] { $($with:tt)* } else { $($without:tt)* }) => {
        #[cfg(any(debug_assertions, test))]
        {
            $($with)*
        }
        #[cfg(all(not(debug_assertions), not(test)))]
        {
            $($without)*
        }
    };
}
