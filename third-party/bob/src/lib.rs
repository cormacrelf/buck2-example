#[cfg(not(bob_the_builder))]
fn function() {
    compile_error!("failed to get rustc cfgs from build.rs");
}

pub fn other_function() {}
