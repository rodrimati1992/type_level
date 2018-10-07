/*!
Bundled-in datatypes that use ConstValue parameters.
    


*/

macro_rules! with_docs {
    (
        $($tt:tt)*
    ) => (

        /**
        A ranged unsigned integer type which stores the number compressed and 
        requires `.value()` to recover the uncompressed number.

        # Minimum Rust version

        The minimum required version for ranged unsigned integers is 1.26 .

        The reason is because the tests 
        take a ludicrous ammount of time to compile before 1.26.

        */
        $($tt)*
    )
}

with_docs!{
    #[cfg(rust_1_26)]
    pub mod ranged_uint;
}

with_docs!{
    #[cfg(not(rust_1_26))]
    pub mod ranged_uint{}
}
