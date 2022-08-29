use std::collections::HashMap;
fn main() {
    let inner_pkg_options= risc0_build::GuestOptions {
        code_limit: 10,
        features: vec![],
        test_mode: true,
    };

    let map = HashMap::from([("methods-guest", inner_pkg_options)]);
    risc0_build::embed_methods_with_options(map);
}
