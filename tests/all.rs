use clap::arg;
use cucumber::{
    cli::{Args, Opts},
    World,
};
use futures_lite::future;
use testutils::world::RayTracerWorld;

const ALL_TESTS: [&str; 8] = [
    "lights",
    "materials",
    "world",
    "camera",
    "shapes",
    "spheres",
    "planes",
    "patterns",
];

#[derive(Args, Clone)]
struct CustomOpts {
    #[arg(long)]
    rt_tests: Option<String>,
}

fn main() {
    let mut tests: Vec<String> = vec![];

    let opts = Opts::<_, _, _, CustomOpts>::parsed();
    let tests_opt = opts.custom.rt_tests.clone();

    if let Some(tests_str) = tests_opt {
        tests.extend(tests_str.split(',').map(|s| s.to_string()));
    } else {
        tests.extend(ALL_TESTS.iter().map(|s| s.to_string()));
    }

    for t in tests {
        future::block_on(
            RayTracerWorld::cucumber()
                .with_cli(opts.clone())
                .run(format!("tests/features/{}.feature", t)),
        );
    }
}
