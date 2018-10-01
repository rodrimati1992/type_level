use crate_::type_fn::TypeFn_;

impl<R> TypeFn_<R> for () {
    type Output = R;
}

///////////////////////////////////////////////////////////////////

impl<T0, Param> TypeFn_<Param> for (T0,)
where
    T0: TypeFn_<Param>,
{
    type Output = T0::Output;
}

impl<T0, T1, Param> TypeFn_<Param> for (T0, T1)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
{
    type Output = T1::Output;
}

impl<T0, T1, T2, Param> TypeFn_<Param> for (T0, T1, T2)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
{
    type Output = T2::Output;
}

impl<T0, T1, T2, T3, Param> TypeFn_<Param> for (T0, T1, T2, T3)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
{
    type Output = T3::Output;
}

impl<T0, T1, T2, T3, T4, Param> TypeFn_<Param> for (T0, T1, T2, T3, T4)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
{
    type Output = T4::Output;
}

impl<T0, T1, T2, T3, T4, T5, Param> TypeFn_<Param> for (T0, T1, T2, T3, T4, T5)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
{
    type Output = T5::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, Param> TypeFn_<Param> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
{
    type Output = T6::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, Param> TypeFn_<Param> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
{
    type Output = T7::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
{
    type Output = T8::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
{
    type Output = T9::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
{
    type Output = T10::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
    T11: TypeFn_<T10::Output>,
{
    type Output = T11::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
    T11: TypeFn_<T10::Output>,
    T12: TypeFn_<T11::Output>,
{
    type Output = T12::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, Param> TypeFn_<Param>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
    T11: TypeFn_<T10::Output>,
    T12: TypeFn_<T11::Output>,
    T13: TypeFn_<T12::Output>,
{
    type Output = T13::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, Param> TypeFn_<Param>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
    )
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
    T11: TypeFn_<T10::Output>,
    T12: TypeFn_<T11::Output>,
    T13: TypeFn_<T12::Output>,
    T14: TypeFn_<T13::Output>,
{
    type Output = T14::Output;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, Param> TypeFn_<Param>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
where
    T0: TypeFn_<Param>,
    T1: TypeFn_<T0::Output>,
    T2: TypeFn_<T1::Output>,
    T3: TypeFn_<T2::Output>,
    T4: TypeFn_<T3::Output>,
    T5: TypeFn_<T4::Output>,
    T6: TypeFn_<T5::Output>,
    T7: TypeFn_<T6::Output>,
    T8: TypeFn_<T7::Output>,
    T9: TypeFn_<T8::Output>,
    T10: TypeFn_<T9::Output>,
    T11: TypeFn_<T10::Output>,
    T12: TypeFn_<T11::Output>,
    T13: TypeFn_<T12::Output>,
    T14: TypeFn_<T13::Output>,
    T15: TypeFn_<T14::Output>,
{
    type Output = T15::Output;
}
