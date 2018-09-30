


////////////////////////////////////////////////////////////////////////////////////////
////                    FoldL_ 
////////////////////////////////////////////////////////////////////////////////////////

/*


use std::fmt::Write;

fn main(){
    let recursive_case=8;
    let pre_rec=recursive_case-1;

    for i in 1..=recursive_case {
        let mut buffer=String::new();
        let _=write!(buffer,"impl<");
        for j in 0..i {
            let _=write!(buffer,"L{0},OrRes{0},",j);
        }
        if i==recursive_case {
            let _=write!(buffer,"OrRes{},Rem,",i);
        }
        let _=writeln!(buffer,"Op,DefaultVal> FoldL_<DefaultVal,Op>");
        let _=write!(buffer,"for tlist![");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        if i==recursive_case {
            let _=write!(buffer,"..Rem");
        }
        let _=writeln!(buffer,"]");
        let _=writeln!(buffer,"where");
        let _=writeln!(buffer,"\tOp:TypeFn_<(DefaultVal,L0),Output=OrRes0>,");
        for j in 0..(i-1) {
            let _=writeln!(buffer,"\tOp:TypeFn_<(OrRes{0},L{1}),Output=OrRes{1}>,",j,j+1);
        }
        if i==recursive_case {
            let _=writeln!(buffer,"\tRem:FoldL_<OrRes{},Op,Output=OrRes{}>,",
                pre_rec,
                recursive_case
            );
        }
        let returned=if i==recursive_case { i }else{ i-1 };
        let _=writeln!(buffer,"{{\n\ttype Output=OrRes{0};\n}}",returned);
        println!("{}",buffer );
    }
    
}

*/



////////////////////////////////////////////////////////////////////////////////////////
////                    FoldR_ 
////////////////////////////////////////////////////////////////////////////////////////



/*


use std::fmt::Write;

fn main(){
    let recursive_case=8;

    for i in 1..=recursive_case {
        let returned=if i==recursive_case { i }else{ i-1 };
    
        let mut buffer=String::new();
        let _=write!(buffer,"impl<");
        for j in 0..i {
            let _=write!(buffer,"L{0},OrRes{0},",j);
        }
        if i==recursive_case {
            let _=write!(buffer,"OrRes{},Rem,",i);
        }
        let _=writeln!(buffer,"Op,DefaultVal>\n\tFoldR_<DefaultVal,Op>");
        let _=write!(buffer,"for tlist![");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        if i==recursive_case {
            let _=write!(buffer,"..Rem");
        }
        let _=writeln!(buffer,"]");
        let _=writeln!(buffer,"where");
        if i==recursive_case {
            let _=writeln!(buffer,"\tRem:FoldR_<DefaultVal,Op,Output=OrRes0>,");
        }else{
            let _=writeln!(buffer,"\tOp:TypeFn_<(DefaultVal,L{}),Output=OrRes0>,",i-1);
        }
        for j in 0..returned {
            let _=writeln!(buffer,"\tOp:TypeFn_<(OrRes{0},L{1}),Output=OrRes{2}>,",
                            j,returned-j-1,j+1);
        }
        let _=writeln!(buffer,"{{\n\ttype Output=OrRes{0};\n}}",returned);
        println!("{}",buffer );
    }
    
}


*/




////////////////////////////////////////////////////////////////////////////////////////
////                    Map_
////////////////////////////////////////////////////////////////////////////////////////

/*


use std::fmt::Write;

fn main(){
    let recursive_case=8;

    for i in 1..=recursive_case{
        let is_recursive=i==recursive_case;
        let mut buffer=String::new();
        let _=write!(buffer,"impl<");
        for j in 0..i {
            let _=write!(buffer,"L{0},OrRes{0},",j);
        }
        if is_recursive {
            let _=write!(buffer,"Rem,RemOut,");
        }
        let _=writeln!(buffer,"Op>\n\tMap_<Op>");
        let _=write!(buffer,"for tlist![");
        for j in 0..i {
            let _=write!(buffer,"L{0},",j);
        }
        if is_recursive {
            let _=write!(buffer,"..Rem");
        }
        let _=writeln!(buffer,"]");
        let _=writeln!(buffer,"where");
        for j in 0..i {
            let _=writeln!(buffer,"\tOp:TypeFn_<L{0},Output=OrRes{0}>,",j);
        }
        if is_recursive {
            let _=writeln!(buffer,"\tRem:Map_<Op,Output=RemOut>,");
        }
        let _=write!(buffer,"{{\n\ttype Output=tlist![");
        for j in 0..i{
            let _=write!(buffer,"OrRes{},",j);
        }
        if is_recursive {
            let _=write!(buffer,"..RemOut");
        }
        let _=writeln!(buffer,"];\n}}");
        println!("{}",buffer );
    }
    
}

*/