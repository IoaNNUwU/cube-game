use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("solid_macro_generated.rs");
    fs::write(
        dest_path, format!("\
/// Macro used as a shortcut to create default solid block state
/// without using Default::default() as arguments
/// ```rust
/// use block::solid;
///
/// use block::BlockState;
/// use block::SolidBlock;
/// assert_eq!(
///     BlockState::Solid(SolidBlock::Stone(Default::default())),
///     solid!(SolidBlock::Stone),
/// );
/// assert_eq!(
///     BlockState::Solid(SolidBlock::Stone(Default::default())),
///     solid!(Stone),
/// );
///
/// let block_state: BlockState = solid!(UnbreakableStone);
/// ```
// generated by cube-game/common_crates/block/build.rs
#[macro_export]
macro_rules! solid {{\n{}}}", gen_solid_macro_arms()),
    ).expect("Unable to write solid_macro_generated.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

type IdentWithNumberOfArgs = (String, usize);

fn gen_solid_macro_arms() -> String {
    let mut buf = String::new();

    let solid_enum_structure = parse_solid_enum_structure();

    for (number_of_args, idents) in solid_enum_structure.iter() {
        for ident in idents {
            match *number_of_args {
                0 => {
                    let arm1 = format!(
                        "    ($solid_enum_name:ident::{}) => (BlockState::Solid($solid_enum_name::{}));\n", ident, ident
                    );
                    let arm2 = format!(
                        "    ({}) => (BlockState::Solid(SolidBlock::{}));\n", ident, ident
                    );
                    buf.push_str(&arm1);
                    buf.push_str(&arm2);
                }
                1 => {
                    let arm1 = format!(
                        "    ($solid_enum_name:ident::{}) => (BlockState::Solid($solid_enum_name::{}(::core::default::Default::default())));\n", ident, ident
                    );
                    let arm2 = format!(
                        "    ({}) => (BlockState::Solid(SolidBlock::{}(::core::default::Default::default())));\n", ident, ident
                    );
                    buf.push_str(&arm1);
                    buf.push_str(&arm2);
                }
                n => {
                    let arm1 = format!(
                        "    ($solid_enum_name:ident::{}) => (BlockState::Solid($solid_enum_name::{}(::core::default::Default::default(){})));\n",
                        ident, ident, String::from(", ::core::default::Default::default()").repeat(n - 2)
                    );
                    let arm2 = format!(
                        "    ({}) => (BlockState::Solid(SolidBlock::{}(::core::default::Default::default(){})));\n",
                        ident, ident, String::from(", ::core::default::Default::default()").repeat(n - 2)
                    );
                    buf.push_str(&arm1);
                    buf.push_str(&arm2);
                }
            };
        }
    }
    buf.push_str("() => (BlockState::default());\n");
    buf
}

fn parse_solid_enum_structure() -> HashMap<usize, Vec<String>> {
    let crate_path = env::current_dir().expect("Unable to identify current dir");

    let solid_rs_path = Path::new(&crate_path).join("src/solid_block.rs");
    let solid_block_rs = fs::read_to_string(solid_rs_path.clone())
        .unwrap_or_else(|_| panic!("Unable to read {:?}", solid_rs_path));

    let solid_ident_with_number_of_args: Vec<IdentWithNumberOfArgs> = solid_block_rs.lines()
        .skip_while(|line| !line.contains("pub enum SolidBlock {"))
        .skip(1)
        .take_while(|line| !line.contains('}'))
        .map(|line| {
            let has_arguments = line.chars().any(|char| char == '(');

            let number_of_args = line.chars()
                .filter(|&char| char == ',')
                .count();

            let number_of_args = if has_arguments {
                number_of_args + 1
            } else {
                number_of_args - 1
            };

            let ident: String = line.chars()
                .skip_while(|&char| char == ' ')
                .take_while(|&char| char != '(' && char != ',')
                .collect();

            (ident, number_of_args)
        })
        .collect();

    let mut map: HashMap<usize, Vec<String>> = HashMap::new();

    for (ident, number_of_args) in solid_ident_with_number_of_args {
        map.entry(number_of_args).or_default().push(ident);
    };

    map
}