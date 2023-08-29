# buck2 example

There are very few examples of buck2 in use on the internet. This is an attempt
at getting at least something working with the rust rules.

### What's in it

- `prelude` as a git submodule referring to [this][prelude]. There is no reason
  why you couldn't use
  git-subtree for this, but at present a submodule is better so you can fetch
  the right commit hash of prelude and checkout exactly that one.
- `third-party`, a directory for using [`reindeer`][reindeer]
- `platforms/BUCK` -- a few predefined platforms, including
  `platforms//:wasm32` for building a crate under wasm32.

[prelude]: https://github.com/facebook/buck2-prelude
[reindeer]: https://github.com/facebookincubator/reindeer

## Installing all the stuff

Here's some commands to get you going. You will need rustup installed, and the
stable toolchain also installed and set as your default.

```sh
cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer

DIR=$(pwd)
git clone https://github.com/cormacrelf/buck2-example.git
cd buck2-example
git submodule update --init --recursive

git clone https://github.com/facebook/buck2 "$DIR/buck2"
cd "$DIR/buck2"
# match the .buckversion file
git checkout $(cat "$DIR/buck2-example/.buckversion")
cargo install --path app/buck2
cargo install --path integrations/rust-project

# now build some stuff
cd "$DIR/buck2-example"
buck2 test //src/krate:
buck2 run //src/krate:krate-bin
buck2 build //src/wasmlib:wasmlib --target-platforms platforms//:wasm32

# generate a rust-project.json file for a list of targets
rust-project develop --prefer-rustup-managed-toolchain //src/krate:
# you can then use rust-analyzer and it will pick up that file
# instead of using cargo workspaces etc.
```

### Which version of buck2, should I use buckle, etc

Buckle is good, _ish_. It may not be useful for installing the rust-project
binary from the same git comit as buck2 (which you almost certainly need for
rust development), you can use it for getting the correct commit hash for your
prelude submmodule.

```sh
cargo install --locked buckle
ln -s ~/.cargo/bin/buckle ~/.local/bin/buck2
echo '2023-08-15' > .buckversion

# then this thing tells you to update your prelude. You have to read the output
# of the command! It's one line at the top. It won't exit the command, which it
# probably should if there is a prelude mismatch and you're using a tagged release.
# :/
buck2 build //src/krate:krate

# prints out e.g.
cd /path/to/buck/project/prelude && git fetch && git checkout 01234abcdef
```

### How to get rust-analyzer working

There is a tool called `rust-project` for that. It is built for a specific version of
the prelude, just like buck2, so it should be built from the same commit of
buck2 as the buck2 binary is. Buckle is not useful for this. You will need a
copy of the buck2 source code.

```sh
git clone https://github.com/facebook/buck2 ~/src/buck2
cd ~/src/buck2
git submodule update --init --recursive
# match the .buckversion file
git fetch && git checkout "$(cat ~/src/buck2-example/.buckversion)"
cargo install --locked --path app/buck2
cargo install --locked --path integrations/rust-project

cd ~/src/buck2-example
rust-project develop --prefer-rustup-managed-toolchain //src/krate:
nvim
```

#### Issues

- `rust-project` has the functionality to run rustc in check mode on the
  targets that contain the file you just saved. However, how to get rust-analyzer
  to give you that filename when it runs your `rust-project check` command
  is another question entirely.
- At the moment you will basically need to fork rust-analyzer to get that
  filename, or devise a way to update the LSP configuration dynamically in your
  editor (e.g. with an `autocmd BufWritePre`).
- <https://github.com/facebook/buck2/issues/402> is a blocker for getting
  `rust-project check` to produce any results if there are errors and
  not just warnings

#### Neovim setup

Then a Neovim setup looks something like this, I haven't looked into how to
autoconfigure this whenever you're in buck2 project using some kind of
lspconfig hooks or anything, but I'm sure it's possible. Most likely you only
have one repo that needs this anyway, what with buck2 being for monorepos.

```lua
let rust_analyzer_opts = {}
if string.startswith(vim.fn.getcwd(), "/home/...you.../src/buck2-example") then
   rust_analyzer_opts = {
      checkOnSave = {
         # THIS IS NOT REAL FUNCTIONALITY IN RUST-ANALYZER
         # THIS IS A SIMULATION OF HOW IT COULD WORK
         invocationStrategy = "current_file",
         overrideCommand = { "rust-project", "check" },

         # for now you can just hard-code a single file path
         overrideCommand = { "rust-project", "check", "src/krate/src/lib.rs" },
      },
      procMacro = {
         enable = true,
      },
      linkedProjects = {
         "/home/...you.../src/buck2-example/rust-project.json",
      },
   }
end

local rust_tools_opts = {
   -- ...
   server = {
      -- on_attach, etc
      settings = {
         ["rust-analyzer"] = rust_analyzer_opts,
      }
   }
}

rust_tools.setup(rust_tools_opts)

```

### platforms//...

- The `execution_platform`s are execution platforms.
- The `platform`s are not. The main difference is that you can't use these in
  your .buckconfig's `[buck2] execution_platforms` setting.
- Either kind can be used with like `--target-platforms platform//:wasm32`
- You can also use them in the target platform selector spec (see
  .buckconfig)
