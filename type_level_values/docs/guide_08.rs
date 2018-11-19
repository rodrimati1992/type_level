doc_code_snippets! {
    mod "guide_08",
    type_ident=Guide08,
    template=r##"

This chapter demonstrates type-level functions,
by which this crate means an implementor of the TypeFn_ trait.

The TypeFn_ trait is simply defined like this:
```   
trait TypeFn_<Params>{
    type Output;
}
```

Lets say that we want to represent a state machine without tying it to a specific type.

We define  a very simple type-level state machine representing an arcade game,

These are the 3 different states:
    
- GameOver:In which the game is shows the game over screen.
Transitions to Demo if the player quits.
Transitions to Playing if the player presses play and has enough coins.
    
- Demo:showing the demo for the game.
Transitions to Playing if the player presses play and has enough coins.

- Playing:in which one is playing the game.
Transitions to GameOver if the player loses.


//@use_codeblock:states-enum,ignore
 
This are the states.

//@use_codeblock:action-enum,ignore

These are the actions which transition between states.

//@use_codeblock:game-action-fn,ignore

This is a type-level function equivalent to `fn(Game,Action)->Game`,
which calls a helper function `GameActionHelper` 
and returns the return value of the helper function.

Let bindings here are used to emulate temporary (type) variables,
and can be used before their declaration.

//@use_codeblock:game-action-helper-fn,ignore

This helper function changes the current state of the state machine.

Every one of the function branches follows the pattern of 
declaring the `NewGame` variable,
which it initializes with the output of calling a function,
and then immediately returns.

The branches for Continue and StartGame (second and third)
use function composition where a tuple composed of TypeFn_ is itself 
a TypeFn_ that passes its parameter to the first function,
and then passes the return value of each function as the parameter to the next,
returning the return value of the last function.

The `S:GameOverTrait`/`S:DemoTrait`/`S:PlayingTrait`  constraints were used instead of 
the enum variants themselves (GameOver/Demo/Playing) because this produces better error messages
when one attempts to do operations that require a specific state.

The Piped_ trait is used here to pass Self as the parameter to the function it has as a parameter.

Each of the functions called in the where clauses of the function branches 
use type parameters as a way to capture variables.


//@use_codeblock:take-coins-fn,ignore

This helper function makes `Ammount` coins disappear from the arcade.

In here we use the If function,which takes captures predicate/then/else functions,
implementing TypeFn_ such that it takes some state that it passes to the predicate,
if the predicate returns True it runt Then with the state,otherwise it runs Else with the state.

The Panic function emulates panics by intentionally causing a compile-time error
which contains the message we passed to it.

The ConstLtMt,MapFieldMt,and SubMt functions are method-like functions
which captures all the parameters exceot for the 'Self' parameter 
(which is by convention the first parameter of the *Op functions named similarly).

This is equivalent to :
```ignore
fn take_coins(ammount:u32)->impl FnOnce(Game)->Game {
    move|game| {
        if game.coins<amount{
            panic!("not enough coins");
        }
        game.map_field( 
            game_f::coins, 
            |field| field-ammount
        )
    }
}
```

//@use_codeblock:insert-coins-fn,ignore

This helper function inserts `Ammount` coins into the arcade.

This is equivalent to :
```ignore
fn insert_coins(ammount:u32)->impl FnOnce(Game)->Game {
    move|game|{
        game.map_field( 
            game_f::coins, 
            |field| field+ammound 
        )
    }
}
```

//@use_codeblock:initial-game-type,ignore

This is the state of the game when it starts.

//@use_codeblock:arcade-machine-struct,ignore

This is the type which represents the arcade machine,
where the state of the arcade machine is in the `G` ConstValue-parameter.

//@use_codeblock:arcade-machine-new,ignore

This is the constructor for the arcade machine,creating it with no coins and in the Demo state.

Note that the built-in tuple struct constructor requires using the original type name 
`ArcadeMachine_Ty` instead of `ArcadeMachine`.

//@use_codeblock:arcade-action,ignore

This is a method which takes an action and returns the state of the arcade machine
after taking the action.

This method demonstrates a frequently used pattern of using generic parameters
as type aliases within the where clause,with the `_NewGame` type parameter .

It is recommended that if one creates a function following 
the "type parameter as type alias" pattern ,
that it be possible to specify difficult to infer type parameters by adding 
a `_:VariantPhantom<TypeParameter>` parameter,
one can then specify the type by passing a parameter like 
eg:`Vec::T`/`Vec::<u32>::T`/`u32::T`/`Option::T`/`Option::<usize>::T` .


//@use_codeblock:main,ignore

This demonstrates the state machine,
try commenting out the code from bellow which contains the entire example ,
to familiarize with the kind of type errors this produces.










<br><br><br><br><br><br><br><br><br><br>
<hr>
<br><br><br><br><br><br><br><br><br><br>


# The entire thing

//@use_codeblock:all,rust

"##,

    code=r##"


//@codeblock-start:all




#[macro_use]
extern crate derive_type_level;
#[macro_use]
extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::ops::{Panic,IsZeroOp,If,ConstLtMt};
use type_level_values::std_ops::{AddMt,SubMt};
use type_level_values::field_traits::{SetField_,SetFieldMt,MapFieldMt};

//@codeblock-start:states-enum

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants,Traits),
)]
pub enum States{
    GameOver,
    Demo,
    Playing,
}


//@codeblock-end:states-enum


//@codeblock-start:action-enum

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Variants,Traits),
)]
pub enum Actions{
    InputCoins(usize),
    Continue,
    StartGame,
    LoseGame,
    QuitGame,
}

//@codeblock-end:action-enum


//@codeblock-start:game-struct

#[derive(TypeLevel)]
#[typelevel(
    derive(ConstEq,ConstOrd),
    reexport(Struct,Traits),
)]
pub struct Game{
    pub coins:usize,
    pub state:States,
}


pub use self::type_level_Game::fields as game_f;

//@codeblock-end:game-struct


//@codeblock-start:game-action-fn

type_fn!{
    pub fn GameAction[G,Action](G,Action)
    where[
        G:GameTrait,
        GameActionHelper:TypeFn_<(G,G::state,Action),Output=NewGame>
    ]{
        let NewGame;NewGame
    }
}

//@codeblock-end:game-action-fn


//@codeblock-start:game-action-helper-fn

type_fn!{
    pub fn 
    GameActionHelper[G,S,Ammount](G,S,InputCoins<Ammount>)
    where[ InsertCoins<Ammount>:TypeFn_<G,Output=NewGame> ]
    { let NewGame;NewGame }
    
    GameActionHelper[G,S](G,S,Continue)
    where[ 
        S:GameOverTrait,
        G:Piped_<(
            TakeCoins<U1>,
            SetFieldMt<game_f::state,Playing>,
        ),Output=NewGame> 
    ]{ let NewGame;NewGame }

    GameActionHelper[G,S](G,S,StartGame)
    where[ 
        S:DemoTrait,
        G:Piped_<(
            TakeCoins<U1>,
            SetFieldMt<game_f::state,Playing>,
        ),Output=NewGame> 
    ]{ let NewGame;NewGame }


    GameActionHelper[G,S](G,S,LoseGame)
    where[ 
        S:PlayingTrait,
        G:SetField_<game_f::state,GameOver,Output=NewGame>,
    ]{ let NewGame;NewGame }


    GameActionHelper[G,S](G,S,QuitGame)
    { InitialGame }
}


//@codeblock-end:game-action-helper-fn



//@codeblock-start:take-coins-fn


#[doc(hidden)]
pub struct NotEnoughCoins;

pub type TakeCoins<Ammount>=
    MapFieldMt<
        game_f::coins,
        If<ConstLtMt<Ammount>,
            Panic<NotEnoughCoins>,
            SubMt<Ammount>,
        >
    >;
 

//@codeblock-end:take-coins-fn




//@codeblock-start:insert-coins-fn

pub type InsertCoins<Ammount>=
    MapFieldMt<
        game_f::coins,
        AddMt<Ammount>,
    >;

//@codeblock-end  :insert-coins-fn




//@codeblock-start:initial-game-type

pub type InitialGame=Construct<GameType,(
    (game_f::coins,U0),
    (game_f::state,Demo),
)>;

//@codeblock-end  :initial-game-type



//@codeblock-start:arcade-machine-struct

#[derive(MutConstValue)]
#[mcv(
    derive(Copy,Clone,Debug),
    Type="ArcadeMachine",ConstValue ="G",
)]
pub struct __ArcadeMachine<G>(pub ConstWrapper<G>);

//@codeblock-end  :arcade-machine-struct

//@codeblock-start:arcade-machine-new

impl ArcadeMachine<InitialGame>{
    pub fn new()->Self{
        ArcadeMachine_Ty(ConstWrapper::NEW)
    }
}

//@codeblock-end  :arcade-machine-new


//@codeblock-start:arcade-action

impl<Game> ArcadeMachine<Game>{
    pub fn action<Action,_NewGame>(
        self,_action:Action
    )->ArcadeMachine<_NewGame>
    where 
        GameAction:TypeFn_<(Game,Action),Output=_NewGame>
    {
        ArcadeMachine_Ty(ConstWrapper::NEW)
    }
}

//@codeblock-end:arcade-action


pub fn main(){
    //@codeblock-start:main
    
    ArcadeMachine::new()
        .action(InputCoins(U1::CW))
        .action(StartGame)
        .action(InputCoins(U1::CW))
        .action(InputCoins(U1::CW))
        .action(LoseGame)
        .action(InputCoins(U1::CW))
        .action(Continue)
        .action(QuitGame);

    //@codeblock-end  :main
}

"##,
}

