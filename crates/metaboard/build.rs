fn main() {
    cynic_codegen::register_schema("metaboard")
        .from_sdl_file("./src/schema/metaboard.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
