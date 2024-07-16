use std::collections::HashMap;
/*
use std::fmt;
use std::io;
fmt::Result<>;
io::Result<()>;

OR

use std::fmt::Result;
use std::io::Result as IoResult;
Result<>;
IoResult<()>;
*/

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}
