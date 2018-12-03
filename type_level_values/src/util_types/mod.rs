/*!
Types that have ConstValue parameters.
    


*/

macro_rules! with_docs {
    (
        $($tt:tt)*
    ) => (

        /**
        A ranged integer type.

        # Minimum Rust version

        The minimum required version for ranged integers is 1.26 .

        The reason is because the tests
        take a ludicrous ammount of time to compile before 1.26.

        */
        $($tt)*
    )
}

with_docs!{
    #[cfg(rust_1_26)]
    pub mod ranged_int;
}

with_docs!{
    #[cfg(not(rust_1_26))]
    pub mod ranged_int{}
}
