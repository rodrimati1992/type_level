use crate_::ops::Map_;
use crate_::ops::TypeFn_;

impl<Op> Map_<Op> for () where {
    type Output = ();
}

impl<L0, OrRes0, Op> Map_<Op> for (L0,)
where
    Op: TypeFn_<L0, Output = OrRes0>,
{
    type Output = (OrRes0,);
}

impl<L0, OrRes0, L1, OrRes1, Op> Map_<Op> for (L0, L1)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
{
    type Output = (OrRes0, OrRes1);
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, Op> Map_<Op> for (L0, L1, L2)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
{
    type Output = (OrRes0, OrRes1, OrRes2);
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, Op> Map_<Op> for (L0, L1, L2, L3)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
{
    type Output = (OrRes0, OrRes1, OrRes2, OrRes3);
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, Op> Map_<Op>
    for (L0, L1, L2, L3, L4)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
{
    type Output = (OrRes0, OrRes1, OrRes2, OrRes3, OrRes4);
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, Op> Map_<Op>
    for (L0, L1, L2, L3, L4, L5)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
{
    type Output = (OrRes0, OrRes1, OrRes2, OrRes3, OrRes4, OrRes5);
}

impl<L0, OrRes0, L1, OrRes1, L2, OrRes2, L3, OrRes3, L4, OrRes4, L5, OrRes5, L6, OrRes6, Op>
    Map_<Op> for (L0, L1, L2, L3, L4, L5, L6)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
{
    type Output = (OrRes0, OrRes1, OrRes2, OrRes3, OrRes4, OrRes5, OrRes6);
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
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
    );
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
        L8,
        OrRes8,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        L11,
        OrRes11,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
    Op: TypeFn_<L11, Output = OrRes11>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
        OrRes11,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        L11,
        OrRes11,
        L12,
        OrRes12,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
    Op: TypeFn_<L11, Output = OrRes11>,
    Op: TypeFn_<L12, Output = OrRes12>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
        OrRes11,
        OrRes12,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        L11,
        OrRes11,
        L12,
        OrRes12,
        L13,
        OrRes13,
        Op,
    > Map_<Op> for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
    Op: TypeFn_<L11, Output = OrRes11>,
    Op: TypeFn_<L12, Output = OrRes12>,
    Op: TypeFn_<L13, Output = OrRes13>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
        OrRes11,
        OrRes12,
        OrRes13,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        L11,
        OrRes11,
        L12,
        OrRes12,
        L13,
        OrRes13,
        L14,
        OrRes14,
        Op,
    > Map_<Op>
    for (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
    )
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
    Op: TypeFn_<L11, Output = OrRes11>,
    Op: TypeFn_<L12, Output = OrRes12>,
    Op: TypeFn_<L13, Output = OrRes13>,
    Op: TypeFn_<L14, Output = OrRes14>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
        OrRes11,
        OrRes12,
        OrRes13,
        OrRes14,
    );
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
        L8,
        OrRes8,
        L9,
        OrRes9,
        L10,
        OrRes10,
        L11,
        OrRes11,
        L12,
        OrRes12,
        L13,
        OrRes13,
        L14,
        OrRes14,
        L15,
        OrRes15,
        Op,
    > Map_<Op>
    for (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
        L15,
    )
where
    Op: TypeFn_<L0, Output = OrRes0>,
    Op: TypeFn_<L1, Output = OrRes1>,
    Op: TypeFn_<L2, Output = OrRes2>,
    Op: TypeFn_<L3, Output = OrRes3>,
    Op: TypeFn_<L4, Output = OrRes4>,
    Op: TypeFn_<L5, Output = OrRes5>,
    Op: TypeFn_<L6, Output = OrRes6>,
    Op: TypeFn_<L7, Output = OrRes7>,
    Op: TypeFn_<L8, Output = OrRes8>,
    Op: TypeFn_<L9, Output = OrRes9>,
    Op: TypeFn_<L10, Output = OrRes10>,
    Op: TypeFn_<L11, Output = OrRes11>,
    Op: TypeFn_<L12, Output = OrRes12>,
    Op: TypeFn_<L13, Output = OrRes13>,
    Op: TypeFn_<L14, Output = OrRes14>,
    Op: TypeFn_<L15, Output = OrRes15>,
{
    type Output = (
        OrRes0,
        OrRes1,
        OrRes2,
        OrRes3,
        OrRes4,
        OrRes5,
        OrRes6,
        OrRes7,
        OrRes8,
        OrRes9,
        OrRes10,
        OrRes11,
        OrRes12,
        OrRes13,
        OrRes14,
        OrRes15,
    );
}
