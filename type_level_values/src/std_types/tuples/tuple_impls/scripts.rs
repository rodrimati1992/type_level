//////////////////////////////////////////////////////////////////////////////////////////
////                       Map_ script
//////////////////////////////////////////////////////////////////////////////////////////

/*

use std::fmt::Write;

fn main(){

    for i in 0..=16{
        let mut buffer=String::new();
        let _=write!(buffer,"impl<");
        for j in 0..i {
            let _=write!(buffer,"L{0},OrRes{0},",j);
        }
        let _=writeln!(buffer,"Op> Map_<Op>");
        let _=write!(buffer,"for (");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        let _=writeln!(buffer,")");
        let _=writeln!(buffer,"where");
        for j in 0..i {
            let _=writeln!(buffer,"\tOp:TypeFn_<L{0},Output=OrRes{0}>,",j);
        }
        let _=write!(buffer,"{{\n\ttype Output=(");
        for j in 0..i{
            let _=write!(buffer,"OrRes{},",j);
        }
        let _=writeln!(buffer,");\n}}");
        println!("{}",buffer );
    }
    
}

*/

////////////////////////////////////////////////////////////////////////////////////////
////                            TypeFn_ impl
////////////////////////////////////////////////////////////////////////////////////////

/*
    
use std::fmt::Write;

fn main(){

    for i in 1..=16{
        let mut buffer=String::new();
        let _=write!(buffer,"impl<");
        for j in 0..i {
            let _=write!(buffer,"T{0},",j);
        }
        let _=writeln!(buffer,"Param> TypeFn_<Param>");
        let _=write!(buffer,"for (");
        for j in 0..i {
            let _=write!(buffer,"T{0},",j);
        }
        let _=writeln!(buffer,")");
        let _=writeln!(buffer,"where");
        let _=writeln!(buffer,"\tT0:TypeFn_<Param>,");
        for j in 1..i {
            let _=writeln!(buffer,"\tT{}:TypeFn_<T{}::Output>,",j,j-1);
        }
        let _=writeln!(buffer,"{{\n\ttype Output=T{0}::Output;\n}}",i-1);
        println!("{}",buffer );
    }
    
}


*/

////////////////////////////////////////////////////////////////////////////////////////
////                            Push impl
////////////////////////////////////////////////////////////////////////////////////////

/*


use std::fmt::Write;

fn main(){

    for i in 0..16{
        let mut type_params=String::new();
        for j in 0..i {
            let _=write!(type_params,"L{0},",j);
        }
        for field in i..=i{
            let mut buffer=String::new();
            let _=writeln!(buffer,"impl<{}Value> Push_<Value>",type_params);
            let _=writeln!(buffer,"for ({})",type_params);
            let _=write!(buffer,"{{\n\ttype Output=(");
            for field_out in 0..i{
                if field_out == field {
                    let _=write!(buffer,"Value,");
                };
                let _=write!(buffer,"L{},",field_out);
            }
            if field==i {
                let _=write!(buffer,"Value,");
            }
            let _=writeln!(buffer,");\n}}");
            println!("{}",buffer );
            
        }
    }
    
}



*/

////////////////////////////////////////////////////////////////////////////////////////
////                            Pop impl
////////////////////////////////////////////////////////////////////////////////////////

/*

use std::fmt::Write;

fn main(){

    for i in 1..=16{
        let mut type_params=String::new();
        for j in 0..i {
            let _=write!(type_params,"L{0},",j);
        }
        for field in 0..1{
            let mut buffer=String::new();
            let _=writeln!(buffer,"impl<{}> Pop_",type_params);
            let _=writeln!(buffer,"for ({})",type_params);
            let _=write!(buffer,"{{\n\ttype Output=Some_<(L{},(",i-1);
            for field_out in 0..i-1{
                let _=write!(buffer,"L{},",field_out);
            }
            let _=writeln!(buffer,"))>;\n}}");
            println!("{}",buffer );
            
        }
    }
    
}

*/

////////////////////////////////////////////////////////////////////////////////////////
////                            PushFront_ impl
////////////////////////////////////////////////////////////////////////////////////////

/*


use std::fmt::Write;

fn main(){

    for i in 0..16{
        let mut type_params=String::new();
        for j in 0..i {
            let _=write!(type_params,"L{0},",j);
        }
        {
            let mut buffer=String::new();
            let _=writeln!(buffer,"impl<{}Value> PushFront_<Value>",type_params);
            let _=writeln!(buffer,"for ({})",type_params);
            let _=write!(buffer,"{{\n\ttype Output=(");
            let _=write!(buffer,"Value,");
            for field_out in 0..i{
                let _=write!(buffer,"L{},",field_out);
            }
            let _=writeln!(buffer,");\n}}");
            println!("{}",buffer );
            
        }
    }
    
}




*/

////////////////////////////////////////////////////////////////////////////////////////
////                            PopFront_ impl
////////////////////////////////////////////////////////////////////////////////////////

/*

use std::fmt::Write;

fn main(){

    println!("{}",r"\
        impl PopFront_ for (){ \n\t\
            type Output=None_; \n\
        } \n\
    ");

    for i in 1..=16{
        let mut type_params=String::new();
        for j in 0..i {
            let _=write!(type_params,"L{0},",j);
        }
        for field in 0..1{
            let mut buffer=String::new();
            let _=writeln!(buffer,"impl<{}> PopFront_",type_params);
            let _=writeln!(buffer,"for ({})",type_params);
            let _=write!(buffer,"{{\n\ttype Output=Some_<(L0,(");
            for field_out in 1..i{
                let _=write!(buffer,"L{},",field_out);
            }
            let _=writeln!(buffer,"))>;\n}}");
            println!("{}",buffer );
            
        }
    }
    
}


*/

////////////////////////////////////////////////////////////////////////////////////////
////                           Reverse impl
////////////////////////////////////////////////////////////////////////////////////////

/*


use std::fmt::Write;

fn main(){

    let mut buffer=String::new();

    let _=write!(buffer,"type_fn!{{\n\tpub fn");

    for i in 0..=16{
        let _=write!(buffer,"\n\tReverse_Override[");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        let _=write!(buffer,"]\n\t\t(");
        let _=write!(buffer,"(");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        let _=write!(buffer,")");
        let _=write!(buffer,")\n\t{{ (");
        for j in (0..i).rev() {
            let _=write!(buffer,"L{0},",j);
        }
        let _=write!(buffer,") }}");
        
    }

    let _=write!(buffer,"}}");

    println!("{}",buffer );
    
}



*/
