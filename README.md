drawille-nostd
===========

This repository contains a `no_std` version of the 
[drawille](https://github.com/ftxqxd/drawille-rs) crate. 
Contains all its functionality, but is slower due to `BTreeMap` usage for character 
array instead of `FnvHashMap`. Additionally, all `colored` functionality was removed
because of the enormous amount of std-dependent stuff that was used in there.

Usage
-----

```toml
[dependencies]
drawille-nostd = "0.1.1"
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

### Contribution

See [LICENSE-MIT](LICENSE) for contribution rules.