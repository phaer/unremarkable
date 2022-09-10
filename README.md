# unremarkable-notes

Is a work-in-progress rust library to parse and render files from the [Remarkable 2](https://remarkable.com/) e-ink tablet.

It works with the file structure found in `~/.local/share/remarkable/xochitl`, whether they are pure
remarkable notebooks, ePubs or PDFs. Features include higlight- and annotation extraction as well as
rendering via [lines-are-rusty](https://github.com/ax3l/lines-are-rusty).

## Usage

We use the [nix package manager](https://nixos.org/) for our build & release process, 
see [here how to install it](https://nixos.org/download.html) for your platform.

### Development

``` shell
nix develop
```

Opens a shell which includes all dependencies, rust tooling and environment variables from `.envrc`.

Use this shell to build & and run a development build locally.

``` shell
cargo build
```

### Release

``` shell
nix build
```

Will download and build a static cross-compiler toolchain to produce a static binary, which isn't too useful yet,
as we ship a library only, but will be once we a CLI interface and could be handy to build your own.

We plan provide a public binary cache via [cachix](https://cachix.org), populated via [github](https://github.com) actions,
after we provide a CLI again.

## Resources

* [lines-are-rusty](https://github.com/ax3l/lines-are-rusty) is used to render Remarkable´s `.rm` files to `.svg` and `.pdf` files.
  We currently use a [custom fork](https://github.com/phaer/lines-are-rusty), but going to try upstreaming changes when they stabilize.
  
* [rmfakecloud](https://ddvk.github.io/rmfakecloud/) is a libre re-implementation of Remarkable´s cloud api. It's helpful to understand
the synchronization API. We are using a private instance during development.
 
