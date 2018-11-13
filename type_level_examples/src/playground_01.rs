use type_level_values::prelude::*;


#[derive(TypeLevel)]
#[typelevel(reexport(Struct))]
struct Rectangle{
    x:u32,
    y:u32,
    w:u32,
    h:u32,
}


use self::type_level_Rectangle::fields;



#[derive(MutConstValue)]
#[mcv(Type="RectangleMCV",ConstValue="I")]
struct __RectangleMCV<I>{
    x:u32,
    y:u32,
    w:u32,
    h:u32,
    aa:u8,
    bb:u8,
    _marker:ConstWrapper<I>,
}




pub fn main(){

    println!("{:?}",::std::mem::size_of::<RectangleMCV<()>>());
    println!("{:?}",::std::mem::align_of::<u8>());
    println!("{:?}",::std::mem::align_of::<u32>());

}
