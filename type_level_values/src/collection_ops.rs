/*!
Operations for collection types,including TypeList,tuples,Option,Result.
*/


use std_::ops::Sub;

use prelude::*;

use crate_::field_traits::{
    GetField, GetFieldOp, MapField, MapFieldOp, MapIntoField, MapIntoFieldOp, SetField,
};
use crate_::fn_adaptors::*;
use crate_::std_ops::*;
use crate_::ops::{
    ConstFrom_,
    ConstInto_,ConstIntoOp,ConstIntoMt,
    IntoInnerOp,IntoInner_,UnwrapOrMt,UnwrapOp,
    If,
    AssertPipedRet,
    ConstLtOp,ConstLtMt,ConstEqMt,
    Add1Op,SatSub1Op
};
use crate_::new_types::type_list;



/// 
/// `static=true` means that we don't pass the $this parameter to the $method when calling it.
macro_rules! declare_collection_op {
    ( 
        $(static=$is_static:ident,)*
        
        $( #[$all_meta:meta] )*
        fn $method:ident ( $this:ident $(,$param:ident)* )
        
        $( #[$type_meta:meta] )*
        type=$type_ident:ident,

        $( #[$func_meta:meta] )*
        function   = $func:ident,

        $( #[$mt_meta:meta] )*
        methodlike = $methodlike:ident,
    ) => {
        $( #[$all_meta] )*
        $( #[$type_meta] )*
        pub type $type_ident<$this $(,$param)*>=
            TypeFn<
                <
                    <
                        $this as $crate::collection_ops::Collection
                    >::Items as CollectionItemsTrait
                >::$method , 
                declare_collection_op!(inner_method_params; 
                    [$(static=$is_static)*] 
                    $this 
                    $(,$param)*
                )
            >;

        type_fn!{
            $( #[$all_meta] )*
            $( #[$func_meta] )*
            pub fn $func[$this $(,$param)*]($this $(,$param)*)
            where[
                $this:$crate::collection_ops::Collection<Items= Methods >,
                Methods:CollectionItemsTrait,
                Methods::$method:TypeFn_<
                    declare_collection_op!(inner_method_params; 
                        [$(static=$is_static)*] 
                        $this 
                        $(,$param)*
                    ),
                    Output=Out
                >
            ]{
                let Methods;let Out;
                Out
            }

        }

        type_fn!{
            captures($($param),*)
            $( #[$all_meta] )*
            $( #[$mt_meta] )*
            pub fn $methodlike[$this]($this)
            where[
                $this:$crate::collection_ops::Collection<Items= Methods >,
                Methods:CollectionItemsTrait,
                Methods::$method:TypeFn_<
                    declare_collection_op!(inner_method_params; 
                        [$(static=$is_static)*] 
                        $this 
                        $(,$param)*
                    ),
                    Output=Out
                >
            ]{
                let Methods;let Out;
                Out
            }
        }
    };
    (inner_method_params; [static=true] $this:ident $(,$param:ident)* )=>{
        ( $($param),* )
    };
    (inner_method_params; [$($anything:tt)*] $this:ident $(,$param:ident)* )=>{
        ( $this $(,$param)* )
    };
}

/**
Trait defined for all collections.

This delegates most items to Self::Items,
defined in the 
[CollectionItemsTrait](./type_level___CollectionItems/trait.CollectionItemsTrait.html)
impl of 
[CollectionItems](./type_level___CollectionItems/struct.CollectionItems.html).

# Implementations

Implement this trait on a ConstType,and this trait will be implemented 
for all values of that ConstType.

This is how TListType implements this trait,
notice that we use SetFields to override items in 
[DefaultCollectionItems](type.DefaultCollectionItems.html),
and that repeat is overriden with a function of the same name with a `_Override` suffix:

```ignore
impl Collection for TListType{
    type CollectEmpty=TNil;
    type Items=SetFields<DefaultCollectionItems<Self>,tlist!(
        (collfns_f::repeat,Repeat_Override),
    )>;
}
```

*/
pub trait Collection{
    /// The collection this collects into by default in every operation that 
    /// creates a new collection.
    ///
    /// This is the same for most ConstTypes with the notable exception of Range*Type.
    type CollectEmpty;

    /// The associated items of a collection,
    ///
    /// Some of the associated functions in `Self::Items`
    /// may require some of the traits in this module to be implemented in 
    /// their default implementation 
    /// [defined in DefaultCollectionItems](./type.DefaultCollectionItems.html).
    type Items:CollectionItemsTrait;
}

impl<This,Type> Collection for This
where
    This:ConstValue<Type=Type>,
    Type:ConstType+Collection,
{
    type CollectEmpty=Type::CollectEmpty;
    type Items=Type::Items;
}

type_fn!{define_trait
    /// Processes the collection incrementally from the start,
    /// starting with Defaultval and the first element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldL_ [DefaultVal,Func]
    type=FoldL
    fn_type=FoldLOp
    method_like=FoldLMt
}

type_fn!{define_trait
    /// Processes the collection incrementally from the end,
    /// starting with Defaultval and the last element.
    ///
    /// If the collection is empty it must return DefaultVal.
    trait=FoldR_ [DefaultVal,Func]
    type=FoldR
    fn_type=FoldROp
    method_like=FoldRMt
}


declare_collection_op!{
    /// Processes the collection incrementally from the start,using the `Func` function,
    /// returning the first element if the collection only contains 1 element.
    fn reduce_l(This,Func)

    type=ReduceL,
    function=ReduceLOp,
    methodlike=ReduceLMt,
}


declare_collection_op!{
    /// Creates a collection of  pairs of 
    /// each element of This and Other,truncating to the smaller collection.
    fn zip(This,Func)

    type=Zip,
    function=ZipOp,
    methodlike=ZipMt,
}


declare_collection_op!{
    /// Processes the collection incrementally from the end,using the `Func` function,
    /// returning the last element if the collection only contains 1 element.
    fn reduce_r(This,Func)

    type=ReduceR,
    function=ReduceROp,
    methodlike=ReduceRMt,
}


declare_collection_op!{
    /// Turns a nested collection of some ConstType into an unnested collection of 
    /// the same ConstType.
    fn flatten(This)

    type=Flatten,
    function=FlattenOp,
    // This is here for consistency's sake
    methodlike=FlattenMt,
}


type_fn!{define_trait
    /// Transforms the elements of the collection with the `Func` function.
    trait=Map_ [Func]
    type=Map
    fn_type=MapOp
    method_like=MapMt
}


type_fn!{define_trait
    /// Returns the collection in which all the elements that 
    /// do not satisfy the `Predicate` are removed.
    trait=Filter_ [Predicate]
    type=Filter
    fn_type=FilterOp
    method_like=FilterMt
}

type_fn!{define_trait
    /// Removes the element at the `Ìndex` position from the collection.
    trait=Remove_ [Index]
    type=Remove
    fn_type=RemoveOp
    method_like=RemoveMt
}

type_fn!{define_trait
    /// Inserts `Value` at the `Ìndex` position into the collection.
    trait=Insert_ [Index,Value]
    type=Insert
    fn_type=InsertOp
    method_like=InsertMt
}

declare_collection_op!{
    /// Returns the collection with the value added at one end.
    ///
    /// Push followed by Pop must return the pushed value and
    /// the collection as it was before pushing 
    /// (this property does not have apply recursively for any collection,eg:a ring buffer).
    fn push(This,Value)

    type=Push,
    function=PushOp,
    methodlike=PushMt,
}


declare_collection_op!{
    /// Puts all the elements of the Other collection at the end of This.
    fn append(This,Other)

    type=Append,
    function=AppendOp,
    methodlike=AppendMt,
}


declare_collection_op!{
    /// Transforms the elements of the collection using a function that returns 
    /// `impl ConstInto_<TryFold>`,
    /// filtering out the TFBreak<_> values and unwrapping the TFVal<_> values.
    ///
    /// 
    ///
    fn filter_map(This,Value)

    type=FilterMap,
    function=FilterMapOp,
    methodlike=FilterMapMt,
}


declare_collection_op!{
    /// Returns the collection with the last/first element removed alongside that element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    fn pop(This)

    type=Pop,
    function=PopOp,
    methodlike=PopMt,
}

declare_collection_op!{
    /// Returns the collection only containing the first N elements.
    fn take(This,N)

    type=Take,
    function=TakeOp,
    methodlike=TakeMt,
}


declare_collection_op!{
    /// Returns the collection only containing the first elements 
    /// which match the Pred predicate.
    fn take_while(This,Pred)

    type=TakeWhile,
    function=TakeWhileOp,
    methodlike=TakeWhileMt,
}


declare_collection_op!{
    /// Returns the collection skipping the first N elements.
    fn skip(This,N)

    type=Skip,
    function=SkipOp,
    methodlike=SkipMt,
}


declare_collection_op!{
    /// Returns the collection skipping the first elements matching the Pred predicate.
    fn skip_while(This,Pred)

    type=SkipWhile,
    function=SkipWhileOp,
    methodlike=SkipWhileMt,
}


type_fn!{define_trait
    /// Returns the collection with the value added after the last element.
    ///
    /// PushBack followed by PopBack must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushBack_ [Value]
    type=PushBack
    fn_type=PushBackOp
    method_like=PushBackMt
}

type_fn!{define_trait
    /// Returns the collection with the last element removed,alongside the last element.
    ///
    /// Returns Some_<(Element,CollectionWithoutValue)> if the collection is not empty,
    /// otherwise returns None_.
    trait=PopBack_ []
    type=PopBack
    fn_type=PopBackOp
}

type_fn!{define_trait
    /// Returns the collection with the value added before the first element.
    ///
    /// PushFront followed by PopFront must return the pushed value and
    /// the collection as it was before pushing.
    trait=PushFront_ [Value]
    type=PushFront
    fn_type=PushFrontOp
    method_like=PushFrontMt
}

type_fn!{define_trait
    /// Returns the collection with the first element removed,alongside the first element.
    ///
    /// Returns None if the collection is empty ,
    /// otherwise retuns the first value and remaining collection in
    /// Some_<(Value,CollectionWithoutValue)>.
    trait=PopFront_ []
    type=PopFront
    fn_type=PopFrontOp
}

type_fn!{define_trait
    /// The ammount of elements in the collection that can be iterated over in FoldL_ .
    ///
    trait=Len_ []
    type=Len
    fn_type=LenOp
}


declare_collection_op!{
    static=true,
    /// Creates a value of the ConstType associated with the function 
    /// by repeating  `Value` `Repeated` times
    ///
    fn repeat(Type,Value,Repeated)

    type=Repeat,
    function=RepeatOp,
    methodlike=RepeatMt,
}

declare_collection_op!{
    /// Reverses `This`.
    ///
    fn reverse(This)

    type=Reverse,
    function=ReverseOp,
    methodlike=ReverseMt,
}


declare_collection_op!{
    /// Separates the elements of `This` into a pair of collections of the same ConstType
    /// based on 
    /// the return value of Pred,if it return False the element goes 
    /// to the first,True goes to the second,
    ///
    fn partition(This,Pred)

    type=Partition,
    function=PartitionOp,
    methodlike=PartitionMt,
}


declare_collection_op!{
    /// Separates the elements of `This` into a pair of collections of the `Type` 
    /// ConstType based on 
    /// the return value of Pred,if it return False the element goes 
    /// to the first,True goes to the second,
    ///
    fn partition_as(This,Type,Pred)

    type=PartitionAs,
    function=PartitionAsOp,
    methodlike=PartitionAsMt,
}


declare_collection_op!{
    /**
    Searches for an element in the collection that satisfies a predicate.

    FindOp takes a collection `This`,and the predicate `Pred`.

    If the predicate returns true for any element 
    then this function return Some_<TheElement>,otherwise it returns Nnoe_
    */
    fn find(This,Pred)

    type=Find,
    function=FindOp,
    methodlike=FindMt,
}


declare_collection_op!{
    /**
    Returns the first position from the start at which the Pred predicate returns True.

    Returns Some_<pos> if the Predicate returns True for some element,
    None_ if it returns False for all elements.
    */
    fn position(This,Pred)

    type=Position,
    function=PositionOp,
    methodlike=PositionMt,
}

declare_collection_op!{
    /**
    Returns the first position from the end at which the Pred predicate returns True.

    Returns Some_<pos> if the Predicate returns True for some element,
    None_ if it returns False for all elements.
    */
    fn r_position(This,Pred)

    type=RPosition,
    function=RPositionOp,
    methodlike=RPositionMt,
}



declare_collection_op!{
    /**
    Searches for an element in the collection,found when Finder returns Some_<Element>

    If Finder Some_<Elem> this returns early with that value,otherwise it keeps looking,
    returning None_ if nothing is found.
    */
    fn find_map(This,Finder)

    type=FindMap,
    function=FindMapOp,
    methodlike=FindMapMt,
}



declare_collection_op!{
    /**
    Tests whether a predicate is true for all elements of a collection.

    This function takes a collection `This`,and the predicate `Pred`.

    If the predicate returns True for all element 
    then this function return True,otherwise it returns False

    */
    fn all(This,Pred)

    type=All,
    function=AllOp,
    methodlike=AllMt,
}



declare_collection_op!{
    /**
    Tests whether a predicate is true for any elements of a collection.
    
    This function takes a collection `This`,and the predicate `Pred`.
    
    If the predicate returns True for any element 
    then this function return True,otherwise it returns False
    */
    fn any(This,Pred)

    type=Any,
    function=AnyOp,
    methodlike=AnyMt,
}


/////////////////////////////////////////////////////////////////////////////////////////


type_fn!{define_trait
    /** 
    Processes the collection incrementally from the start,
    starting with Defaultval and the first element,
    returning early when Func returns a value that converts to TFBreak like Err\_<\_>/None\_,
    
    If the collection is empty it must return TFVal<DefaultVal>.

    # Example

    ```

    # #[macro_use]
    # extern crate type_level_values;

    # use type_level_values::prelude::*;
    use type_level_values::ops::*;
    use type_level_values::collection_ops::*;

    fn main(){
        struct NotAnInteger;
        
        let _:AssertEq<
            TryFoldL<tlist![ U1 ],U6,SafeSubOp>,
            TFVal<U5>
        >;
        let _:AssertEq<
            TryFoldL<tlist![ U1,U2 ],U6,SafeSubOp>,
            TFVal<U3>
        >;
        let _:AssertEq<
            TryFoldL<tlist![ U1,U2,U3 ],U6,SafeSubOp>,
            TFVal<U0>
        >;
        let _:AssertEq<
            TryFoldL<tlist![ U1,U2,U3,U1, ],U6,SafeSubOp>,
            TFBreak<None_>
        >;
        let _:AssertEq<
            TryFoldL<tlist![ U1,U2,U3,U1,NotAnInteger ],U6,SafeSubOp>,
            TFBreak<None_>
        >;
        

    }

    ```


    */
    trait=TryFoldL_ [DefaultVal,Func]
    type=TryFoldL
    fn_type=TryFoldLOp
    method_like=TryFoldLMt
}

type_fn!{define_trait
    /** 
    Processes the collection incrementally from the end,
    starting with Defaultval and the last element,
    returning early when Func returns a value that converts to TFBreak like Err\_<\_>/None\_,
    
    If the collection is empty it must return TFVal<DefaultVal>.

    # Example

    ```

    # #[macro_use]
    # extern crate type_level_values;

    # use type_level_values::prelude::*;
    use type_level_values::ops::*;
    use type_level_values::collection_ops::*;

    fn main(){
            
        struct NotAnInteger;

        let _:AssertEq<
            TryFoldR<tlist![ U1 ],U6,SafeSubOp>,
            TFVal<U5>
        >;
        let _:AssertEq<
            TryFoldR<tlist![ U1,U2 ],U6,SafeSubOp>,
            TFVal<U3>
        >;
        let _:AssertEq<
            TryFoldR<tlist![ U1,U2,U3 ],U6,SafeSubOp>,
            TFVal<U0>
        >;
        let _:AssertEq<
            TryFoldR<tlist![ U1,U1,U2,U3 ],U6,SafeSubOp>,
            TFBreak<None_>
        >;
        let _:AssertEq<
            TryFoldR<tlist![ NotAnInteger,U1,U1,U2,U3 ],U6,SafeSubOp>,
            TFBreak<None_>
        >;

        

    }

    ```

    */
    trait=TryFoldR_ [DefaultVal,Func]
    type=TryFoldR
    fn_type=TryFoldROp
    method_like=TryFoldRMt
}


#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
#[typelevel(items(runtime_conv(NoImpls)))]
pub enum TryFold<T,B>{
    #[typelevel(doc="\
Represents a value.

This is mainly used in TryFold{L,R},and anything that uses TryFold.

This can be converted to/from OptionType/ResultType
    ")]
    TFVal(T),
    #[typelevel(doc="\
Represents the intent to break out of the iteration operation.

This is mainly used in TryFold{L,R},and anything that uses TryFold.

This can be converted to/from OptionType/ResultType
    ")]
    TFBreak(B),
}

type_fn!{
    /// Constructs a TFVal<V>
    pub fn NewTFVal[v](v){ TFVal<v> }
}
type_fn!{
    /// Constructs a TFBreak<V>
    pub fn NewTFBreak[v](v){ TFBreak<v> }
}

impl<T> IntoInner_ for TFVal<T> {
    type Output=T;
}
impl<T> IntoInner_ for TFBreak<T> {
    type Output=T;
}


///////////////////////////

impl<T,Func> Map_<Func> for TFVal<T>
where Func:TypeFn_<T>
{
    type Output=TFVal<Func::Output>;
}

impl<T,Func> Map_<Func> for TFBreak<T>{
    type Output=TFBreak<T>;
}

///////////////////////////


/** 
Alias for converting a value to a TryFoldType.
*/
pub type IntoTryFold=ConstIntoMt<TryFoldType>;


macro_rules! define_tryfold_conv {
    ( generics[$($generic:tt)*] $from:ty : $from_consttype:ty => $try_flow:ty ) => (
        impl<$($generic)*> ConstFrom_<$from> for TryFoldType{
            type Output=$try_flow;
        }
        impl<$($generic)*> ConstInto_<$from_consttype> for $try_flow{
            type Output=$from;
        }
    )
}

define_tryfold_conv!{ generics[T] Ok_<T>:ResultType => TFVal<T> }
define_tryfold_conv!{ generics[T] Err_<T>:ResultType => TFBreak<Err_<T>> }

define_tryfold_conv!{ generics[T] Some_<T>:OptionType => TFVal<T> }
define_tryfold_conv!{ generics[]  None_   :OptionType => TFBreak<None_> }


/////////////////////////////////////////////////////////////////////////////////////////


macro_rules! declare_collection_items {
    (
        Self_ident=$Self_ident:ident,
        // assoc_types=[
        //     $($static_fn:ident = $default_static_fn:ty ),*
        //     $(,)*
        // ]
        static_fns=[
            $($static_fn:ident = $default_static_fn:ty ),*
            $(,)*
        ]
        methods=[
            $($method_fn:ident = $default_method_fn:ty ),*
            $(,)*
        ]
    ) => (
        #[doc(hidden)]
        #[derive(TypeLevel)]
        #[typelevel(
            rename="CollectionItems",
            rename_constvalue="CollectionItems",
            doc="\
        The methods of a collection.

            ",
            items(runtime_conv(NoImpls)) //This is only a type-level struct
        )]
        pub struct __CollectionItems{
            // $(pub $assoc:(),)*
            $(pub $static_fn:(),)*
            $(pub $method_fn:(),)*
        }

        use self::type_level___CollectionItems::{
            CollectionItems_Uninit,
        };
        pub use self::type_level___CollectionItems::{
            CollectionItems,
            CollectionItemsTrait,
            fields as collfns_f,
        };


        // /// The accessors for the associated types/ConstValues in CollectionItems.
        // pub type CollectionAssocTypes=tlist!(
        //     $( collfns_f::$assoc, )*
        // );

        /// The accessors for the associated functions (not taking a 
        /// Self parameter) in CollectionItems.
        pub type CollectionAssocFns=tlist!(
            $( collfns_f::$static_fn, )*
        );

        /// The accessors for the methods in CollectionItems.
        pub type CollectionMethods=tlist!(
            $( collfns_f::$method_fn, )*
        );

        /// Constructs the default CollectionItems.
        ///
        /// The `SelfType` parameter must be a ConstType,eg:TupleType,TListType,OptionType,etc.
        pub type DefaultCollectionItems<$Self_ident>=Construct<
            CollectionItems_Uninit,
            tlist![
                $((collfns_f::$static_fn , $default_static_fn) ,)*
                $((collfns_f::$method_fn , $default_method_fn) ,)*
            ]
        >;
    )
}

declare_collection_items!{
    Self_ident=SelfType,

    static_fns=[
        repeat= Repeat_DefaultImpl<SelfType> ,
    ]
    methods=[
        append= Append_DefaultImpl ,
        filter_map=FilterMap_DefaultImpl  ,
        find_map= FindMap_DefaultImpl ,
        flatten= Flatten_DefaultImpl ,
        first= First_DefaultImpl ,
        last= Last_DefaultImpl ,
        partition= Partition_DefaultImpl ,
        partition_as= PartitionAs_DefaultImpl,
        position=  Position_DefaultImpl,
        r_position=  RPosition_DefaultImpl,
        skip= Skip_DefaultImpl ,
        skip_while= SkipWhile_DefaultImpl ,
        take= Take_DefaultImpl ,
        take_while= TakeWhile_DefaultImpl ,
        zip= Zip_DefaultImpl ,
        all= All_DefaultImpl ,
        any= Any_DefaultImpl ,
        find= Find_DefaultImpl ,
        pop= Pop_DefaultImpl ,
        push= Push_DefaultImpl ,
        reduce_l= ReduceL_DefaultImpl ,
        reduce_r= ReduceR_DefaultImpl ,
        reverse= Reverse_DefaultImpl ,
    ]
}



type_fn!{
    pub fn Take_DefaultImpl[This,N](This,N)
    where[
        This:Collection<CollectEmpty=Empty>,
        RepeatOp:TypeFn_<(type_list::TListType,(),N),Output=Taken>,
        Taken:FoldL_<(This,Empty),Take_Helper0,Output=rev>,
        (GetRhs,ReverseOp):TypeFn_<rev,Output=Out>
    ]{
        let Empty;let Taken;let rev;let Out;
        Out
    }
}


type_fn!{
    fn Take_Helper0[This,OutList]((This,OutList),())
    where[ ( PopFrontOp, Take_Helper1<This,OutList>):TypeFn_<This,Output=Out> ]
    { let Out; Out }
}

type_fn!{
    captures(This,OutList)
    fn 
        Take_Helper1[Elem,Rem](Some_<(Elem,Rem)>)
        where[ OutList:PushFront_<Elem,Output=OutList1> ]
        {
            let OutList1;
            (Rem,OutList1)
        }

        Take_Helper1(None_){
            (This,OutList)
        }
}



type_fn!{
    pub fn Skip_DefaultImpl[This,N](This,N)
    where[
        This:Collection<CollectEmpty=Empty>,
        RepeatOp:TypeFn_<(type_list::TListType,(),N),Output=Removed>,
        Removed:FoldL_<
            This,
            (GetLhs,PopFrontOp,MapMt<GetRhs>,UnwrapOrMt<Empty>),
            Output=Out
        >,
    ]{
        let Empty;let Removed;let Out;
        Out
    }
}




type_fn!{
    pub fn SkipWhile_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt<
                This,
                If<(GetRhs,Pred),
                    (GetLhs,PopFrontOp,UnwrapOp,GetRhs,NewTFVal),
                    (GetLhs,NewTFBreak),
                > 
            >,
            IntoInnerOp,
        ):TypeFn_<This,Output=Out>,
    ]{
        let Out;
        Out
    }
}



type_fn!{
    pub fn TakeWhile_DefaultImpl[This,Pred](This,Pred)
    where[
        This:Collection<CollectEmpty=Empty>,
        (
            TryFoldLMt<
                Empty,
                If<(GetRhs,Pred),
                    (PushFrontOp,NewTFVal),
                    (GetLhs,NewTFBreak),
                > 
            >,
            IntoInnerOp,
            ReverseOp,
        ):TypeFn_<This,Output=Out>,
    ]{
        let Empty;
        let Out;
        Out
    }
}



type_fn!{
    pub fn Position_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt< U0, If<(GetRhs,Pred),(GetLhs,NewTFBreak),(GetLhs,Add1Op,NewTFVal)> >,
            BreakToSome,
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    pub fn RPosition_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldRMt< U0, If<(GetRhs,Pred),(GetLhs,NewTFBreak),(GetLhs,Add1Op,NewTFVal)> >,
            BreakToSome,
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    fn 
        BreakToSome[v](TFBreak<v>){ Some_<v> }
        BreakToSome[v](TFVal<v>){ None_ }
}


type_fn!{
    pub fn Partition_DefaultImpl[This,Pred](This,Pred)
    where[ PartitionAsOp:TypeFn_<(This,This,Pred),Output=Out>, ]
    { let Out; Out }
}


type_fn!{
    pub fn PartitionAs_DefaultImpl[This,Type,Pred](This,Type,Pred)
    where[
        Type:Collection<CollectEmpty=Empty>,
        This:FoldR_<(Empty,Empty),Partition_helper<Pred>,Output=Out>
    ]{
        let Empty;
        let Out;
        Out
    }
}
type_fn!{
    captures(Pred)
    fn Partition_helper[Pair,Elem](Pair,Elem)
    where[
        If<Pred,Const<U1>,Const<U0>>:TypeFn_<Elem,Output=Which>,
        MapFieldOp:TypeFn_<(Pair,Which,PushFrontMt<Elem>),Output=Out>,
    ]{
        let Which;
        let Out;
        Out
    }
}



type_fn!{
    pub fn Zip_DefaultImpl[This,Other](This,Other)
    where[
        This:Collection<CollectEmpty=Empty>,
        (
            TryFoldLMt<(This,Empty),Zip_Helper0>,
            IntoInnerOp,
            GetRhs,
            ReverseOp,
        ):TypeFn_<Other,Output=Out>
    ]{ let Empty;let Out; Out }
}


type_fn!{
    fn Zip_Helper0[Reved,OutList,Elem1]((Reved,OutList),Elem1)
    where[
        (
            PopFrontOp,
            Zip_Helper1<Reved,OutList,Elem1>,
        ):TypeFn_<Reved,Output=Out>
    ]{
        let Out;
        Out
    }
}

type_fn!{
    captures(Reved,OutList,Elem1)
    fn 
        Zip_Helper1[Elem0,Rem](Some_<(Elem0,Rem)>)
        where[ OutList:PushFront_<(Elem0,Elem1),Output=Out> ]
        {
            let Out;
            TFVal<(Rem,Out)>
        }

        Zip_Helper1(None_){
            TFBreak<(Reved,OutList)>
        }

}



type_fn!{
    pub fn First_DefaultImpl[This](This)
    where[
        (
            PopFrontOp,
            MapMt<GetLhs>,
        ):TypeFn_<This,Output=Out>
    ]{
        let Out;
        Out
    }
}


type_fn!{
    pub fn Last_DefaultImpl[This](This)
    where[
        (
            PopBackOp,
            MapMt<GetLhs>,
        ):TypeFn_<This,Output=Out>
    ]{
        let Out;
        Out
    }
}


type_fn!{
    pub fn Append_DefaultImpl[This,Other](This,Other)
    where[
        // Implemented like this because using PushBack on lists is O(n^2),
        // and doing PushFront on Other is not guaranteed to return the same ConstType as This.
        This:Collection<CollectEmpty=Empty>,
        Other:FoldR_<Empty,PushFrontOp,Output=tmp0>,
        This :FoldR_<tmp0 ,PushFrontOp,Output=Out>,
    ]{ 
        let Empty;
        let tmp0;
        let Out;
        Out 
    }
}


type_fn!{
    pub fn All_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt<True,(GetRhs,Pred,If<IdentityFn,NewTFVal,NewTFBreak>)>,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}


type_fn!{
    pub fn Any_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt<False,(GetRhs,Pred,If<IdentityFn,NewTFBreak,NewTFVal>)>,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}


type_fn!{
    pub fn Find_DefaultImpl[This,Pred](This,Pred)
    where[
        (
            TryFoldLMt< None_, (GetRhs,If<Pred,(NewSome,NewTFBreak),(NewNone,NewTFVal)>) >,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}


type_fn!{
    pub fn FindMap_DefaultImpl[This,Finder](This,Finder)
    where[
        (
            TryFoldLMt< None_, (GetRhs,Finder,BreakIfSome) >,
            IntoInnerOp
        ):TypeFn_< This, Output=Out >
    ]{ let Out; Out }
}

type_fn!{
    fn 
        BreakIfSome[v](Some_<v>){ TFBreak<Some_<v>> }
        BreakIfSome(None_){ TFVal<None_> }
}


type_fn!{
    pub fn FilterMap_DefaultImpl[This,Func](This,Func)
    where[
        This:Collection<CollectEmpty=Empty>,
        This:FoldR_< 
            Empty, 
            MapRhs<PushFrontIfTFVal,(Func,IntoTryFold)> ,
            Output=Out
        >,
    ]{ 
        let Empty;
        let Out; 
        Out 
    }
}
type_fn!{
    fn 
        PushFrontIfTFVal[Coll,Val](Coll,TFVal<Val>)
        where[ Coll:PushFront_<Val,Output=Out> ]
        { let Out;Out }

        PushFrontIfTFVal[Coll,E](Coll,TFBreak<E>)
        { Coll }
}



type_fn!{
    pub fn Flatten_DefaultImpl[This](This)
    where[
        This:Collection<CollectEmpty=Empty>,
        This:FoldR_<Empty,Flatten_Helper,Output=Out>,
    ]{ 
        let Empty;
        let Out; 
        Out 
    }
}


type_fn!{
    fn Flatten_Helper[Accum,Collection](Accum,Collection)
    where[ Collection:FoldR_<Accum,PushFrontOp,Output=Out>, ]
    {
        let Out; 
        Out 
    }
}



type_fn!{
    pub fn Pop_DefaultImpl[This](This)
    where[ This:PopFront_<Output=Out> ]
    { let Out;Out }
}

type_fn!{
    pub fn Use_PopBackOp[This](This)
    where[ This:PopBack_<Output=Out> ]
    { let Out;Out }
}


type_fn!{
    pub fn Push_DefaultImpl[This,Val](This,Val)
    where[ This:PushFront_<Val,Output=Out> ]
    { let Out;Out }
}

type_fn!{
    pub fn Use_PushBackOp[This,Val](This,Val)
    where[ This:PushBack_<Val,Output=Out> ]
    { let Out;Out }
}


type_fn!{
    pub fn Reverse_DefaultImpl[This](This)
    where[
        This: Collection<CollectEmpty=CE>,
        This: FoldL_<CE, PushFrontOp, Output = Out>,
    ]{
        let CE;let Out;
        Out
    }
}

type_fn!{
    pub fn ReduceL_DefaultImpl[This,Op](This,Op)
    where[
        This: PopFront_<Output = Some_<(Val, Rem)>>,
        Rem: FoldL_<Val, Op, Output = Out>,
    ]{
        let Val;let Rem;let Out;
        Out
    }
}


type_fn!{
    pub fn ReduceR_DefaultImpl[This,Op](This,Op)
    where[
        This: PopBack_<Output = Some_<(Val, Rem)>>,
        Rem: FoldR_<Val, Op, Output = Out>,
    ]{
        let Val;let Rem;let Out;
        Out
    }
}


type_fn!{
    captures(Type)
    pub fn Repeat_DefaultImpl[Value,Ammount](Value,Ammount)
    where[
        Type: Collection,
        type_list::Repeat_Override:TypeFn_<(Value,Ammount),Output=RepList>,
        RepList: FoldL_<Type::CollectEmpty, PushFrontOp, Output = Out>,
    ]{
        let RepList;
        let Out;
        Out
    }
}


