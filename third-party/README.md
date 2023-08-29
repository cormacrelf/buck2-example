# Reindeer configuration

What is this directory anyway?

[Reindeer][reindeer] generates BUCK files from Cargo.toml + Cargo.lock. We will
use it to create a BUCK package representing all the dependencies we want to
use from <https://crates.io>.

[reindeer]: https://github.com/facebookincubator/reindeer

Cargo has developed a number of conventions and features for
building crates. The main one is the serving, bundling, and downloading of
crates. Others include the setting of environment variables at compile time
(like `CARGO_PKG_VERSION`), and the `build.rs` file and the way its output is
interpreted.

Those conventions need to be translated into BUCK. You might ask, why not
create a `rust_crate` target, feed in a crate name, and have the rule do the
work? Firstly, there are already rules for doing nearly everything Cargo can
do, e.g. `http_archive` to download and extract a .crate bundle. But more
importantly, for Buck2 to work most efficiently, some parts of what Cargo does
need to be in separate targets so that Buck2 can leverage its cache. One 
obvious example is for those http_archive targets, but 

So reindeer spits out BUCK files that replicate Cargo features, typically by
adding targets and configuring the 'compile the crate' targets differently to
how buck does by default.

### What are fixups for then?

The `http_archive` part is easy. Implementing things like `build.rs` is not.
We need fixups to feed in data for reindeer to bridge the gap of not having
perfectly automated all of Cargo's behaviour.

Reindeer reads the fixups when it executes, i.e.

    reindeer --third-party-dir third-party buckify

For any crate in the graph, if there is a matching entry in the fixups
directory, it modifies the produced BUCK entries.

Most fixups seem to just need things like
`println!("cargo:rustc-cfg=blah")`. That is satisfied by using this fixup:

    [[builscript]]
    [buildscript.rustc_flags]

For the memchr crate, that makes the following difference to the BUCK file.
Note that the line `[[buildscript]]` makes a build script via the two
rules (one to build the build.rs file, then another run the script), and the
`[buildscript.rustc_flags]` configures the main crate's rust_library rule
using the output of that build script. In the case of rustc flags, the flags
parsed from the output (e.g. `cargo:rustc-cfg=blah`) are written to a file,
and that file is read by
`$(location :memchr-2.5.0--build-script-run[rustc_flags])`, and the `@file` arg
file syntax (reading one arg per line) is supported by rustc.

```diff
 http_archive(
     name = "memchr-2.5.0.crate",
     sha256 = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d",
     strip_prefix = "memchr-2.5.0",
     urls = ["https://crates.io/api/v1/crates/memchr/2.5.0/download"],
     visibility = [],
 )

 cargo.rust_library(
     name = "memchr-2.5.0",
     srcs = [":memchr-2.5.0.crate"],
     crate = "memchr",
     crate_root = "memchr-2.5.0.crate/src/lib.rs",
     edition = "2018",
     features = [
         "default",
         "std",
     ],
+    rustc_flags = ["@$(location :memchr-2.5.0-build-script-run[rustc_flags])"],
     visibility = [],
 )
+
+cargo.rust_binary(
+    name = "memchr-2.5.0-build-script-build",
+    srcs = [":memchr-2.5.0.crate"],
+    crate = "build_script_build",
+    crate_root = "memchr-2.5.0.crate/build.rs",
+    edition = "2018",
+    features = [
+        "default",
+        "std",
+    ],
+    visibility = [],
+)
+
+buildscript_run(
+    name = "memchr-2.5.0-build-script-run",
+    package_name = "memchr",
+    buildscript_rule = ":memchr-2.5.0-build-script-build",
+    features = [
+        "default",
+        "std",
+    ],
+    version = "2.5.0",
+)
```
