pub mod join;
pub mod leave;

use serenity::framework::standard::macros::group;

use crate::commands::join::*;
use crate::commands::leave::*;

#[group]
#[commands(join, leave)]
pub struct Commands;
