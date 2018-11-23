Steps to create a collection function:

Do these in any order:

- Create the default implementation of the function (named \*\_DefaultImpl ).

- Declare it in the only `declare_collection_items` macro invocation,
    and assign the default implementation of the function to the field.

- Create a test for type-lists (in the new_types::type_list::tests module),
    and optionally other ConstTypes.

- Declare the type-alias/free-function/method-like-function that delegate to 
    the collection method,using the `declare_colection_op` macro 
    (look at other invocations of the macro in the collection_ops module).


# Collection Functions to create


### Function:First

Returns the first element,
None_ if the colection is empty,Some_\< first\_element > if it is not empty.

### Function:Last

Returns the last element,
None_ if the colection is empty,Some_\< last\_element > if it is not empty.

### Function:Append

Joins 2 collections of the same ConstType.

### Function:Zip

Takes 2 collections,returning a single collection of pairs from both.
Truncating to the shorter collection.

### Function:FilterMap

Takes a collection of T and a `TypeFn(T)->TB where TB:Into<TryFold>` 
where the value is converted into a TryFold,
then if the value is TFVal the value is added to the collection,
if the value is TFBreak the value is skipped.

### Function:SkipWhile<P>(self, predicate: P) -> SkipWhile<Self, P> where

Returns the collection after removing the first elements until they don't satisfy the predicate.

### Function:TakeWhile

Returns the collection keeping the first elements until they don't satisfy the predicate.

### Function:Skip

Returns the collection after skipping the first `N` elements.

### Function:Take

Returns the collection of the first `N` elements.

### Function:Flatten

Takes a doubly nested collection and flatten it to a single level of nesting.

### Function:Partition

Splits the collection into 2 collections of the same ConstType ,
using a predicate to determine which collection is goes to.

### Function:PartitionAs

Splits the collection into 2 collections of a ConstType (passed in as a parameter),
using a predicate to determine which collection is goes to.

### Function:FindMap

Takes a collection and a `Func:impl TypeFn(T)->Option<T>` ,
when Func returns a Some_ this function returns that value.

### Function:Position

Returns the position,at which an element matches the predicate.

### Function:RPosition

Returns the position,starting from the end,at which an element matches the predicate.



# Collection trait

This trait is the main trait collections need to implement to call the functions in the collection_ops module:

```
trait Collection{
    /// The value this collection collects into.
    /// Range* collects into a type-list since you can't filter/map over a range directly.
    type CollectEmpty;

    /// Uses a CollectionItems to determine which functions (from this module) 
    /// get specialized implementations.
    type Items:CollectionItemsTrait;
}
```

The default value for CollectionItems is `collection_ops::DefaultCollectionItems`.

Each of the fields of the CollectionItems struct is a function matching the signature of 
the collection_ops::* function of a similar name
(except for associated functions like `repeat`,they don't take a self parameter).

For an example of overriding a function here is RangeType's implementation of Collection:

```

impl<This,Type> Collection for RangeType {
    /// Used when creating a new collection.
    type CollectEmpty = tlist![];

    type Items    = SetFields<DefaultCollectionItems,tlist!(
        ( coll_fn_fields::skip , SkipOverride ),
    )>;
}



fn SkipOverride[S,E,Skipped](ConstRange<S,E>,Skipped)
where[
    S:Add<Skipped,Output=S2>,
]{
    let S2;
    ConstRange<S2,E>
}

    


```
