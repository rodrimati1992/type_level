pub(crate) fn leak_box<'a,T>(box_: Box<T>) -> &'a mut T 
where T:'a+?Sized
{
    unsafe { 
        &mut *Box::into_raw(box_) 
    }
}

pub(crate) fn leak_vec<'a,T>(vec_: Vec<T>) -> &'a mut [T]
where T:'a
{
    leak_box(vec_.into_boxed_slice())
}

pub(crate) fn leak_string<'a>(str_: String) -> &'a mut str{
    leak_box(str_.into_boxed_str())
}