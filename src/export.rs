use std::fs::File;
use std::io::{Result, Write};

use rs_wasm_canvas::Point;

fn main() -> Result<()> {
    let s = Point::export();
    let mut f = File::create("www/ts/export.ts")?;
    f.write_all(&s.into_bytes())?;

    Ok(())
}