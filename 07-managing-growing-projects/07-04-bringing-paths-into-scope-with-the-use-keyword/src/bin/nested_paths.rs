use std::{cmp::Ordering, io};

// Style A
use std::io;
use std::io::Write;

// Style B, equivalent to Style A
use std::io::{self, Write};

// Bring all public items defined in a path into scope with the glob operator
// Be careful when using the glob operator! Glob can make it harder to tell what
// names are in scope and where a name used in your program was defined.
use std::collections::*;
