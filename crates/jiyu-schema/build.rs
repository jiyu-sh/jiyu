use capnpc::CompilerCommand;

fn main() {
    CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/code.capnp")
        .file("schema/trigger.capnp")
        .file("schema/request.capnp")
        .file("schema/response.capnp")
        .output_path("src")
        .run()
        .expect("schema compiler failed");
}
