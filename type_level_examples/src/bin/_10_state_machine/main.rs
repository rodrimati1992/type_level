//! This example demonstrates a type-level state machine.
//!
//!     
//!
//!

#![deny(overflowing_literals)]

#[macro_use]
extern crate type_level_values;

#[macro_use]
extern crate derive_type_level;
extern crate num_traits;
extern crate take_mut;

#[macro_use]
pub mod generic_variant;
pub mod channel_end;
pub mod ranged_usize;
pub mod user_input;

// use type_level_values::ops::{IfEager, TypeFn, TypeFn_};
use type_level_values::prelude::*;

use type_level_values::core_extensions::TryInto;

use self::generic_variant::*;

use self::user_input::readln_res;

use self::channel_end::*;

use std::thread;

/// Please use \[src\] to read the source code,it is more readable.
pub type ExampleOperations = tlist!(

    TransferTo<Server,SetText>,
    TransferTo<Client,CurrentPosition>,
    TransferTo<Server,StartPlaying>,

    construct!{Loop_Uninit=>
        op_f::kind=Finite<U5>,
        op_f::sequence=tlist![
            construct!(Branch_Uninit=>
                op_f::who_choses=Client,
                op_f::branches=g_variants![
                    tlist![ TransferTo<Server,SetText> ],
                    tlist![ TransferTo<Server,StartPlaying> ],
                    tlist![
                        TransferTo<Client,TextMessage>,
                        construct!{Loop_Uninit=>
                            op_f::kind=Infinite,
                            op_f::sequence=tlist![
                                construct!(Branch_Uninit=>
                                    op_f::who_choses=Server,
                                    op_f::branches=g_variants![
                                        tlist![ TransferTo<Client,CurrentPosition> ],
                                        tlist![ TransferTo<Server,Ping> ],
                                        tlist![ LoopBreak ],
                                    ],
                                ),
                            ],
                       }
                    ],
                    tlist![ LoopBreak ],
                ],
            ),
        ],
    },

    TransferTo<Server,StopPlaying>,
);

fn main() {
    let operations = ExampleOperations::MTVAL;

    let (client, server) = Channel::new(operations);

    let thread_handle = thread::spawn(
        move || -> Result<Channel<Client, tlist![Closed]>, ProtocolViolation> {
            let client = client.send(SetText {
                text: "hello".to_string(),
            })?;
            let (client, value) = client.recv()?;
            println!("client received:{:#?}", value);

            let client = client.send(StartPlaying)?;

            let client = client.loop_(|client| {
                let choice = loop {
                    println!(
                        "please choose between:\
                         \n\t- 0:Setting the text.\
                         \n\t- 1:Start playback.\
                         \n\t- 2:Show current playback position 50 times.\
                         \n\t- 3:Quit.\
                         "
                    );
                    let selection =
                        readln_res().expect("\n\nmust be able to accept user input\n\n");
                    match selection.parse() {
                        Ok(v) => break client.range_ty(v),
                        Err(_) => {
                            println!("invalid selection:'{}'.", selection);
                        }
                    }
                };

                let client = client.choose(choice, |v| {
                    g_variants!{match( v ){
                        client=>{
                            println!("Please write the text to send:");
                            let text=readln_res()
                                .expect("\n\nmust be able to accept user input\n\n");
                            client.send(SetText{text})
                        },
                        client=>{
                            client.send(StartPlaying)
                        },
                        client=>{
                            let (client,value)=client.recv()?;
                            println!("client received:{:#?}", value);
                            client.loop_(|client|{
                                client.branch(|v|{
                                    g_variants!(match (v) {
                                        client=>{
                                            let (client,value)=client.recv()?;
                                            println!("client received:{:#?}", value);
                                            Ok(client)
                                        },
                                        client=>{
                                            client.send(Ping)
                                        },
                                        client=>{
                                            client.break_()
                                        },
                                    })
                                })
                            })
                        },
                        client=>{
                            client.break_()
                        },
                    }}
                })?;
                Ok(client)
            })?;

            let client = client.send(StopPlaying)?;

            Ok(client)
        },
    );

    let server_res = (|| -> Result<Channel<Server, tlist![Closed]>, ProtocolViolation> {
        let (server, value) = server.recv()?;

        println!("server received:{:#?}", value);

        let server = server.send(CurrentPosition {
            ms: 100,
            playing: true,
        })?;

        let (server, value) = server.recv()?;

        println!("server received:{:#?}", value);

        let mut current_position = 200;

        let server = server.loop_(|server| {
            let server = server.branch(|v| {
                g_variants!{match( v ){
                    server=>{
                        let (server,value)=server.recv()?;
                        println!("server received:{:#?}",value);
                        Ok(server)
                    },
                    server=>{
                        let (server,value)=server.recv()?;
                        println!("server received:{:#?}",value);

                        current_position-=10;

                        Ok(server)
                    },
                    server=>{
                        let server=server.send(TextMessage("Entering infinite loop.".into()))?;
                        println!("server received:{:#?}", value);

                        let mut index=0usize;

                        server.loop_(|server|{
                            let choice={
                                let range_end=server.choice_range().end-1;
                                let unverified_choice=
                                    if index < 100 { index%range_end }else{ range_end };
                                index+=1;
                                unverified_choice.try_into().expect("\
                                    must be correct since i'm bounding it by _ % choice_range().end\
                                ")
                            };
                            current_position+=1;

                            server.choose(choice,  |v|{
                                g_variants!(match (v) {
                                    server=>{
                                        server.send(CurrentPosition{
                                            ms:current_position,
                                            playing:true,
                                        })
                                    },
                                    server=>{
                                        let (server,value)=server.recv()?;
                                        println!("server received:{:#?}",value);
                                        Ok(server)
                                    },
                                    server=>{
                                        server.break_()
                                    },
                                })
                            })
                        })
                    },
                    server=>{
                        server.break_()
                    },
                }}
            })?;

            current_position += 100;

            Ok(server)
        })?;

        let (server, value) = server.recv()?;
        println!("server received:{:#?}", value);

        Ok(server)
    })();

    println!("\n\nserver terminated with:{:#?}", server_res);
    println!(
        "\n\nclient terminated with:{:#?}",
        thread_handle.join().unwrap()
    );
}

////////////////////////////////////////////////////////////////////////////
//                                 Messages

#[derive(Debug, Clone)]
pub struct SetText {
    text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct StartPlaying;

#[derive(Debug, Clone, Copy)]
pub struct StopPlaying;

#[derive(Debug, Clone, Copy)]
pub struct Ping;

#[derive(Debug, Clone, Copy)]
pub struct Pong;

#[derive(Debug, Clone)]
pub struct TextMessage(String);

#[derive(Debug, Clone, Copy)]
pub struct CurrentPosition {
    ms: u64,
    playing: bool,
}

//////////////////////////////////////////////////////////////////////////////
