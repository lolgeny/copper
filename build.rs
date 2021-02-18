use std::path::Path;
use std::fs;
use std::fmt::Write;
use convert_case::*;
use fs::write;

fn mc_type(file: &str, out_file: &str, enum_name: &str, mc_dir: &Path, out_dir: &Path) {
    let items = mc_dir.join(file);
    let mut out = format!(
r#"
#[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum {} {{
"#, enum_name);
    let mut fmt_out = format!(
r#"impl Display for {} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", match self {{
"#,
    enum_name);
    for item in fs::read_to_string(items).unwrap().split("\r\n") {
        let ident = item[10..].to_case(Case::Pascal);
        write!(
            out,
r#"    {},
"#,
            ident
        ).unwrap();
        write!(
            fmt_out,
r#"            {}::{} => "{}",
"#,
            enum_name, ident, &item[10..]
        ).unwrap();
    }
    out.write_str("}\n").unwrap();
    fmt_out.write_str(
r#"        })
    }
}
"#
    ).unwrap();
    out.write_str(&fmt_out).unwrap();
    fs::write(out_dir.join(out_file), out).unwrap();
}

fn main() {
    let out = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out);
    let mc_dir =  Path::new(".").join("src").join("minecraft");
    println!("cargo:rerun-if-changed=src/minecraft");
    mc_type("blocks.txt", "blocks.rs", "Block", &mc_dir, &out_dir);
    mc_type("items.txt", "items.rs", "Item", &mc_dir, &out_dir);
    mc_type("entity.txt", "entity.rs", "Entity", &mc_dir, &out_dir);
    mc_type("effects.txt", "effect.rs", "Effect", &mc_dir, &out_dir);
    mc_type("enchant.txt", "enchant.rs", "Enchant", &mc_dir, &out_dir);
    mc_type("structures.txt", "structures.rs", "Structure", &mc_dir, &out_dir);


    let mut loc_out = String::from(
r#"#[macro_export]
macro_rules! loc {
    (^$x:literal ^$y:literal ^$z:literal) => {
        $crate::core::Coordinates::Local($x, $y, $z)
    };     
"#
    );
    fn write_n(n: i32, name: char, loc_out: &mut String) {
        match n {
            0 => write!(loc_out, "${}:literal", name),
            1 => write!(loc_out, "~${}:literal", name),
            2 => write!(loc_out, "~"),
            _ => unreachable!()
        }.unwrap();
    }
    fn write_coord(n: i32, name: char, loc_out: &mut String) {
        match n {
            0 => write!(loc_out, "$crate::core::Coordinate::Absolute(${} as f64)", name),
            1 => write!(loc_out, "$crate::core::Coordinate::Relative(${} as f64)", name),
            2 => write!(loc_out, "$crate::core::Coordinate::Relative(0f64)"),
            _ => unreachable!()
        }.unwrap();
    }
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                loc_out.write_str("\t(").unwrap();
                write_n(x, 'x', &mut loc_out);
                loc_out.write_char(' ').unwrap();
                write_n(y, 'y', &mut loc_out);
                loc_out.write_char(' ').unwrap();
                write_n(z, 'z', &mut loc_out);
                loc_out.write_str(") => {$crate::core::Coordinates::Mixed(").unwrap();
                write_coord(x, 'x', &mut loc_out);
                loc_out.write_char(',').unwrap();
                write_coord(y, 'y', &mut loc_out);
                loc_out.write_char(',').unwrap();
                write_coord(z, 'z', &mut loc_out);
                loc_out.write_str(")};\n").unwrap();
            }
        }
    }
    loc_out.write_char('}').unwrap();
    write(out_dir.join("loc.rs"), loc_out).unwrap();
}