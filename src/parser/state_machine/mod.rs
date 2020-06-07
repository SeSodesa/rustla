/// This module contains specifications
/// of state machines used by the parser.

mod states;
mod transitions;

pub trait StateMachine {

  fn run();

}