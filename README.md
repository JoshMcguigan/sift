# Sift

Sift is a unix-style filter, inspired by [iPipeTo](https://github.com/ruyadorno/ipt) and other similar filters. Sift takes input from stdin and creates a text-based interface for selecting from that list. It then sends the selection to stdout for further processing.

### Usage

Running the command below will display an interface to select among all of the available git branches. Branches can be selected using the up/down arrows and the enter key. Once a branch is selected it will be checked out.

```
git branch | sift | xargs git checkout
```

List the files in a directory and choose one to delete.

```
ls | sift | xargs rm
```

The examples assume you have installed sift using `cargo install`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
