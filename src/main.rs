/*!
This file is the entry point of the ruSTLa transpiler.
*/

use rustla;

fn main () -> Result<(), rustla::MainError> {
    match rustla::run() {
        Ok(()) => Ok(()),
        Err(mainerror) => Err(mainerror)
    }
}
