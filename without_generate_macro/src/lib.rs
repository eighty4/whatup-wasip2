mod bindings;

use bindings::*;

struct FnBuildWasm;

impl fn_parsing::Guest for FnBuildWasm {
    fn parse_fn(spec: fn_parsing::ParseFnSpec) -> fn_parsing::ParseFnManifest {
        println!("Hey {}", &spec.project_dir);
        fn_parsing::ParseFnManifest {
            entrypoint: fn_parsing::FnEntrypoint {
                path: String::from("Great jeans!"),
            },
            //imports: ModuleImport::PackageDependency(DependencyImport{
            //    package: String::from("woot"),
            //    subpath: None,
            //}),
        }
    }
}

fn_parsing::export!(FnBuildWasm);
