![Crates.io](https://img.shields.io/crates/v/toy-arms?style=for-the-badge) 
![Docs.rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)

# :alembic: toy-arms
Windows game hack helper utilities in rust.

This crate has some useful macros, functions and traits.

# How to use it

Include toy-arms in your dependencies table in `Cargo.toml`.

```toml
[dependencies]
toy-arms = "0.6.0"
```

# :fire: Practical examples
## internal
With this crate, making dll is simple as this:

```rust
// A neat macro which defines entry point instead of you.
// Also, you dont have to alloc/free console by yourself, console will show up only when debug compile.
toy_arms::create_entrypoint!(hack_main_thread);

// Main thread
fn hack_main_thread() {
    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("using toy-arms {}", i);
    }
}
```

## external
On the other hand, following code is how tamper with memory externally is like.

Since this one shows some example usage of the crate's features, it looks a bit fancier.

```rust
use toy_arms::{MemoryEx, VirtualKeyCode};

fn main() {
    // This const has to be up to date.
    const DW_FORCE_ATTACK: usize = 0x31EDB20;
    // Getting process information
    let memex = MemoryEx::from_process_name("csgo.exe");
    println!("process id = {}, \nprocess handle = {:?}", memex.process_id, memex.process_handle);

    // You can get module information by using get_module_info
    let module_info = memex.get_module_info("client.dll").unwrap();
    println!("{}", module_info.module_name);

    // read fetches the value at where the address is pointing.
    // U have to specify the type of the value with turbofish
    println!("{:?}", memex.read::<u32>(memex.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize).unwrap());

    loop {

        // write helps you tamper with the value.
        memex.write::<u32>(memex.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize, &mut 0x5).unwrap();

        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
```

# :globe_with_meridians: Other examples?
Take a look at examples directory, you'll see more examples!
To run the example:
```shell
cargo build --example EXAMPLE_NAME
```