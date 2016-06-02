# rustc proxy script

The script in this directory lets us override the `libcore` used by
Cargo and rustc.

For various reasons, we need to build the EFI stub using a custom
`libcore`. But Cargo does not expose an option to override this crate --
it insists on using the system version instead. As a workaround, we
write a proxy script to add our `libcore` directly to the call to rustc.

See <https://github.com/rust-lang/cargo/issues/1606>
