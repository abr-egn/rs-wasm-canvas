use std::{
    error::Error,
    fs::File,
    io::{Write},
};

use serde::Serialize;
use tinytemplate::TinyTemplate;

use rs_wasm_canvas::Point;

const TEMPLATE: &'static str = r#"
export class {name}Array \{
    private readonly _view: DataView;
    constructor(buffer: ArrayBuffer, ptr: number, count: number) \{
        this._view = new DataView(buffer, ptr, count * {size});
    }
    {{ for field in exports }}
    {field.0}(ix: number): number \{
        return this._view.getFloat32(ix * {size} + {field.1}, true);
    }
    {{ endfor }}
}
"#;

#[derive(Serialize)]
struct Context {
    name: &'static str,
    size: usize,
    exports: Vec<(&'static str, usize)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("export", TEMPLATE)?;
    let context = Context {
        name: "Point",
        size: std::mem::size_of::<Point>(),
        exports: Point::export(),
    };
    let s = tt.render("export", &context)?;
    let mut f = File::create("www/ts/export.ts")?;
    f.write_all(&s.into_bytes())?;

    Ok(())
}