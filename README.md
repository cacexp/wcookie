# Rust Web Cookie (wcookie)

`wcookie` is an HTTP Cookie implementation written in Rust.

Cookie semantics is defined in [RFC6265](https://datatracker.ietf.org/doc/html/rfc6265):

* Cookies are sent from web servers to user agents in `Set-Cookie` HTTP Response headers as defined in [Set-Cookie header](https://datatracker.ietf.org/doc/html/rfc6265#section-4.2).
* User agents, can incluide Cookies in `Cookie` HTTP request headers when some criteria are met, as defined in [Cookie header](https://datatracker.ietf.org/doc/html/rfc6265#section-5.4).

`wcookie` can be use for managing cookies at both web client and server implementations.

# Related projects

`wcookie` is used by [wclient Rust Web Client](https://github.com/cacexp/wclient).

# Documentation

Please, visit [docs.rs](https://docs.rs/wcookie/) to get access to the last version of the library documentation and user manual.

# Contributions

I appreciate you interest and welcome any comment, bug, idea, etc.

Please, use the [GitHub Issues](https://github.com/cacexp/wcookie/issues) to provide feedback.

# License

Copyright 2021 Juan A. Cáceres (cacexp@gmail.com)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.