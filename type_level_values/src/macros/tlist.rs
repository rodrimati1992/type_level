/**

Type macro for a type-level-list.

This macro uses takes these 2 forms:

- tlist![U0,U1,U2,U3] : 
Where one creates an ordered sequence of types,
this example is equivalent to TList<U0,TList<U1,TList<U2,TList<U3,Nil>>>>.

- tlist![False;3] : 
Repeats the same type 3 times,
this example is equivalent to TList<False,TList<False,TList<False,Nil>>>.

# Example 

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use std::borrow::Cow;

type FirstPrimes=tlist![ U1,U2,U3,U5,U7,U11,U13,U17,U19,U23 ];

type Strings<'a>=tlist![ String,&'a str,Cow<'a,str> ];

# fn main(){}

```

*/
#[macro_export]
macro_rules! tlist {
    () => { $crate::new_types::TNil };
    (..$rest:ty) => { $rest };
    ($current:ty) => { tlist![$current,] };
    ($element:ty;$repeat:ty) => {
        $crate::collection_ops::Repeat<
            $crate::new_types::type_list::TListType,
            $element,
            $repeat
        >
    };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty,$elem_3:ty,$elem_4:ty, $($rest:tt)*) => {
        $crate::new_types::TList<
            $elem_0,
            $crate::new_types::TList<
                $elem_1,
                $crate::new_types::TList<
                    $elem_2,
                    $crate::new_types::TList<
                        $elem_3,
                        $crate::new_types::TList<
                            $elem_4,
                            tlist![$($rest)*]
                        >
                    >
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty,$elem_3:ty, $($rest:tt)*) => {
        $crate::new_types::TList<
            $elem_0,
            $crate::new_types::TList<
                $elem_1,
                $crate::new_types::TList<
                    $elem_2,
                    $crate::new_types::TList<
                        $elem_3,
                        tlist![$($rest)*]
                    >
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty, $($rest:tt)*) => {
        $crate::new_types::TList<
            $elem_0,
            $crate::new_types::TList<
                $elem_1,
                $crate::new_types::TList<
                    $elem_2,
                    tlist![$($rest)*]
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty, $($rest:tt)*) => {
        $crate::new_types::TList<
            $elem_0,
            $crate::new_types::TList<
                $elem_1,
                tlist![$($rest)*]
            >
        >
    };
    ($current:ty, $($rest:tt)*) => {
        $crate::new_types::TList<$current,tlist![$($rest)*]>
    };
}






/** 

Instantiates a type-level-list,
which is a zero-sized-type which does not contain instances of the types it lists.

This macro uses takes these 2 forms:

- tlist_val![U0,U1,U2,U3] : 
this is equivalent to <tlist![U0,U1,U2,U3]>::MTVAL.

- tlist_val![False;3] : 
this is equivalent to <tlist![False;3]>::MTVAL.


# Example 

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use std::borrow::Cow;

fn main(){

    let first_primes:
        tlist    ![ U1,U2,U3,U5,U7,U11,U13,U17,U19,U23 ]=
        tlist_val![ U1,U2,U3,U5,U7,U11,U13,U17,U19,U23 ];

    let strings:
        tlist    ![ String,&str,Cow<str> ]=
        tlist_val![ String,&str,Cow<str> ];

}

```


*/
#[macro_export]
macro_rules! tlist_val {
    ($($all:tt)*) => {
        < tlist!($($all)*) as $crate::core_extensions::MarkerType >::MTVAL
    };
}


