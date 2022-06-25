# apple-dev-tools-wrapper

Some tooling aiming at replacing tools for development on Apple platforms.

So far this only provide a small replacement of ``xcrun``.

## xcrun

This is a small replacement of ``xcrun`` providing support only for ``--sdk/-sdk`` with ``--show-sdk-path``.
It will print ``${SDK_IN_UPPERCASE}_SDKROOT`` or, if ``--sdk`` isn't provided ``SDKROOT``.

## License

apple-dev-tools-wrapper is distributed under the terms of either the MIT license or the Apache
License (Version 2.0), at the user's choice.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
