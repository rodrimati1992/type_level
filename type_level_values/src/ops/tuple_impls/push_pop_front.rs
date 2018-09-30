use crate_::ops::iteration_ops::{PopFront_, PushFront_};
use prelude::*;


impl<Value> PushFront_<Value>
for ()
{
    type Output=(Value,);
}

impl<L0,Value> PushFront_<Value>
for (L0,)
{
    type Output=(Value,L0,);
}

impl<L0,L1,Value> PushFront_<Value>
for (L0,L1,)
{
    type Output=(Value,L0,L1,);
}

impl<L0,L1,L2,Value> PushFront_<Value>
for (L0,L1,L2,)
{
    type Output=(Value,L0,L1,L2,);
}

impl<L0,L1,L2,L3,Value> PushFront_<Value>
for (L0,L1,L2,L3,)
{
    type Output=(Value,L0,L1,L2,L3,);
}

impl<L0,L1,L2,L3,L4,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,)
{
    type Output=(Value,L0,L1,L2,L3,L4,);
}

impl<L0,L1,L2,L3,L4,L5,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,);
}

impl<L0,L1,L2,L3,L4,L5,L6,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,);
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,Value> PushFront_<Value>
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,)
{
    type Output=(Value,L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,);
}







//////////////////////////////////////////////////////////////////////////////



impl PopFront_ for (){
    type Output=None_;
}



impl<L0,> PopFront_
for (L0,)
{
    type Output=Some_<(L0,())>;
}

impl<L0,L1,> PopFront_
for (L0,L1,)
{
    type Output=Some_<(L0,(L1,))>;
}

impl<L0,L1,L2,> PopFront_
for (L0,L1,L2,)
{
    type Output=Some_<(L0,(L1,L2,))>;
}

impl<L0,L1,L2,L3,> PopFront_
for (L0,L1,L2,L3,)
{
    type Output=Some_<(L0,(L1,L2,L3,))>;
}

impl<L0,L1,L2,L3,L4,> PopFront_
for (L0,L1,L2,L3,L4,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,))>;
}

impl<L0,L1,L2,L3,L4,L5,> PopFront_
for (L0,L1,L2,L3,L4,L5,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,> PopFront_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,)
{
    type Output=Some_<(L0,(L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,))>;
}

