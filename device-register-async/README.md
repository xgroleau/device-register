# device-register-async

[![crates.io](https://img.shields.io/crates/v/device-register-async)](https://crates.io/crates/device-register-async) [![documentation](https://docs.rs/device-register-async/badge.svg)](https://docs.rs/device-register-async)

An async version of the trait from the crate [device-register](device_register)
Note that you will need to use nightly and
add those features.

```
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]
```

For now you are probably better off using the traits directly, check the `manual-impl` test in the `deivce-register` crate.

License: MIT OR Apache-2.0
