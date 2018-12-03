/*

    These impls are automatically generated using the scripts in impls_scripts.rs

*/

use crate_::collection_ops::{Filter_, FoldL_, FoldR_, Map_};
use crate_::type_fn::TypeFn_;

////////////////////////////////////////////////////////////////////////////////////////
////                    TypeFn_
////////////////////////////////////////////////////////////////////////////////////////

impl<OrRes0> TypeFn_<OrRes0> for tlist![] where {
    type Output = OrRes0;
}

impl<L0, OrRes0, OrRes1> TypeFn_<OrRes0> for tlist![L0,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
{
    type Output = OrRes1;
}

impl<L0, OrRes0, L1, OrRes1, OrRes2> TypeFn_<OrRes0> for tlist![L0, L1,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
{
    type Output = OrRes2;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, OrRes3> TypeFn_<OrRes0> for tlist![L0, L1, L2,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
{
    type Output = OrRes3;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, OrRes4> TypeFn_<OrRes0>
    for tlist![L0, L1, L2, L3,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
    L3: TypeFn_<OrRes3, Output = OrRes4>,
{
    type Output = OrRes4;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, OrRes5> TypeFn_<OrRes0>
    for tlist![L0, L1, L2, L3, L4,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
    L3: TypeFn_<OrRes3, Output = OrRes4>,
    L4: TypeFn_<OrRes4, Output = OrRes5>,
{
    type Output = OrRes5;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, OrRes6> TypeFn_<OrRes0>
    for tlist![L0, L1, L2, L3, L4, L5,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
    L3: TypeFn_<OrRes3, Output = OrRes4>,
    L4: TypeFn_<OrRes4, Output = OrRes5>,
    L5: TypeFn_<OrRes5, Output = OrRes6>,
{
    type Output = OrRes6;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        OrRes7,
    > TypeFn_<OrRes0> for tlist![L0, L1, L2, L3, L4, L5, L6,]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
    L3: TypeFn_<OrRes3, Output = OrRes4>,
    L4: TypeFn_<OrRes4, Output = OrRes5>,
    L5: TypeFn_<OrRes5, Output = OrRes6>,
    L6: TypeFn_<OrRes6, Output = OrRes7>,
{
    type Output = OrRes7;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        L7,
        OrRes7,
        OrRes8,
        OrRes9,
        Rem,
    > TypeFn_<OrRes0> for tlist![L0, L1, L2, L3, L4, L5, L6, L7, ..Rem]
where
    L0: TypeFn_<OrRes0, Output = OrRes1>,
    L1: TypeFn_<OrRes1, Output = OrRes2>,
    L2: TypeFn_<OrRes2, Output = OrRes3>,
    L3: TypeFn_<OrRes3, Output = OrRes4>,
    L4: TypeFn_<OrRes4, Output = OrRes5>,
    L5: TypeFn_<OrRes5, Output = OrRes6>,
    L6: TypeFn_<OrRes6, Output = OrRes7>,
    L7: TypeFn_<OrRes7, Output = OrRes8>,
    Rem: TypeFn_<OrRes8, Output = OrRes9>,
{
    type Output = OrRes9;
}

////////////////////////////////////////////////////////////////////////////////////////
////                    FoldL_
////////////////////////////////////////////////////////////////////////////////////////

impl<L0, OrRes0, DefaultVal, Op> FoldL_<DefaultVal, Op> for tlist![L0,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
{
    type Output = OrRes0;
}

impl<L0, OrRes0, L1, OrRes1, DefaultVal, Op> FoldL_<DefaultVal, Op> for tlist![L0, L1,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
{
    type Output = OrRes1;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, DefaultVal, Op> FoldL_<DefaultVal, Op>
    for tlist![L0, L1, L2,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
{
    type Output = OrRes2;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, DefaultVal, Op> FoldL_<DefaultVal, Op>
    for tlist![L0, L1, L2, L3,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
{
    type Output = OrRes3;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, DefaultVal, Op>
    FoldL_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L4), Output = OrRes4>,
{
    type Output = OrRes4;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, DefaultVal, Op>
    FoldL_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L4), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L5), Output = OrRes5>,
{
    type Output = OrRes5;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        DefaultVal,
        Op,
    > FoldL_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5, L6,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L4), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L5), Output = OrRes5>,
    Op: TypeFn_<(OrRes5, L6), Output = OrRes6>,
{
    type Output = OrRes6;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        L7,
        OrRes7,
        OrRes8,
        Rem,
        DefaultVal,
        Op,
    > FoldL_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5, L6, L7, ..Rem]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L4), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L5), Output = OrRes5>,
    Op: TypeFn_<(OrRes5, L6), Output = OrRes6>,
    Op: TypeFn_<(OrRes6, L7), Output = OrRes7>,
    Rem: FoldL_<OrRes7, Op, Output = OrRes8>,
{
    type Output = OrRes8;
}

////////////////////////////////////////////////////////////////////////////////////////
////                    FoldR_
////////////////////////////////////////////////////////////////////////////////////////

impl<L0, OrRes0, DefaultVal, Op> FoldR_<DefaultVal, Op> for tlist![L0,]
where
    Op: TypeFn_<(DefaultVal, L0), Output = OrRes0>,
{
    type Output = OrRes0;
}

impl<L0, OrRes0, L1, OrRes1, DefaultVal, Op> FoldR_<DefaultVal, Op> for tlist![L0, L1,]
where
    Op: TypeFn_<(DefaultVal, L1), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L0), Output = OrRes1>,
{
    type Output = OrRes1;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, DefaultVal, Op> FoldR_<DefaultVal, Op>
    for tlist![L0, L1, L2,]
where
    Op: TypeFn_<(DefaultVal, L2), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L1), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L0), Output = OrRes2>,
{
    type Output = OrRes2;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, DefaultVal, Op> FoldR_<DefaultVal, Op>
    for tlist![L0, L1, L2, L3,]
where
    Op: TypeFn_<(DefaultVal, L3), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L2), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L1), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L0), Output = OrRes3>,
{
    type Output = OrRes3;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, DefaultVal, Op>
    FoldR_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4,]
where
    Op: TypeFn_<(DefaultVal, L4), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L3), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L2), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L1), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L0), Output = OrRes4>,
{
    type Output = OrRes4;
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, DefaultVal, Op>
    FoldR_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5,]
where
    Op: TypeFn_<(DefaultVal, L5), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L4), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L3), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L2), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L1), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L0), Output = OrRes5>,
{
    type Output = OrRes5;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        DefaultVal,
        Op,
    > FoldR_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5, L6,]
where
    Op: TypeFn_<(DefaultVal, L6), Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L5), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L4), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L3), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L2), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L1), Output = OrRes5>,
    Op: TypeFn_<(OrRes5, L0), Output = OrRes6>,
{
    type Output = OrRes6;
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        L7,
        OrRes7,
        OrRes8,
        Rem,
        DefaultVal,
        Op,
    > FoldR_<DefaultVal, Op> for tlist![L0, L1, L2, L3, L4, L5, L6, L7, ..Rem]
where
    Rem: FoldR_<DefaultVal, Op, Output = OrRes0>,
    Op: TypeFn_<(OrRes0, L7), Output = OrRes1>,
    Op: TypeFn_<(OrRes1, L6), Output = OrRes2>,
    Op: TypeFn_<(OrRes2, L5), Output = OrRes3>,
    Op: TypeFn_<(OrRes3, L4), Output = OrRes4>,
    Op: TypeFn_<(OrRes4, L3), Output = OrRes5>,
    Op: TypeFn_<(OrRes5, L2), Output = OrRes6>,
    Op: TypeFn_<(OrRes6, L1), Output = OrRes7>,
    Op: TypeFn_<(OrRes7, L0), Output = OrRes8>,
{
    type Output = OrRes8;
}

////////////////////////////////////////////////////////////////////////////////////////
////                    Map_
////////////////////////////////////////////////////////////////////////////////////////

impl<L0, OrRes0, Op> Map_<Op> for tlist![L0,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
{
    type Output = tlist![OrRes0,];
}

impl<L0, OrRes0, L1, OrRes1, Op> Map_<Op> for tlist![L0, L1,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
{
    type Output = tlist![OrRes0, OrRes1,];
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, Op> Map_<Op> for tlist![L0, L1, L2,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
{
    type Output = tlist![OrRes0, OrRes1, OrRes2,];
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, Op> Map_<Op> for tlist![L0, L1, L2, L3,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
{
    type Output = tlist![OrRes0, OrRes1, OrRes2, OrRes3,];
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, Op> Map_<Op>
    for tlist![L0, L1, L2, L3, L4,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
{
    type Output = tlist![OrRes0, OrRes1, OrRes2, OrRes3, OrRes4,];
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, Op> Map_<Op>
    for tlist![L0, L1, L2, L3, L4, L5,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
{
    type Output = tlist![OrRes0, OrRes1, OrRes2, OrRes3, OrRes4, OrRes5,];
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, L6, OrRes6, Op>
    Map_<Op> for tlist![L0, L1, L2, L3, L4, L5, L6,]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
{
    type Output = tlist![OrRes0, OrRes1, OrRes2, OrRes3, OrRes4, OrRes5, OrRes6,];
}

impl<
        L0,
        OrRes0,
        L1,
        OrRes1,
        L2,
        OrRes2,
        L3,
        OrRes3,
        L4,
        OrRes4,
        L5,
        OrRes5,
        L6,
        OrRes6,
        L7,
        OrRes7,
        Rem,
        RemOut,
        Op,
    > Map_<Op> for tlist![L0, L1, L2, L3, L4, L5, L6, L7, ..Rem]
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Rem: Map_<Op, Output = RemOut>,
{
    type Output = tlist![
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        ..RemOut
    ];
}
