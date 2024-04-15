fn main() {
    cynic_codegen::register_schema("metaboard")
        .from_sdl_file("../../subgraph/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
