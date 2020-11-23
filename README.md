# Fakewrite!
Temporary name inspired by fakeroot! **Only Linux x86_64 is supported!**
## What is this?
Fakewrite logs writes, symlinks, file creation and deletion system calls. I'm currently using this tool to debug a large installation script which compiles other projects into libraries and executables. So, there was a need to log everything the scripts were making, linking and editing.
## How to run it
To run fakewrite from the command line, you just need to add it before the program you want to log! E.g:
```bash
  fakewrite bash
  fakewrite make build
```
## How do I get it?
### Via source code
For this, you will need a working rust compiler and cargo package manager. Just clone this repo, run `cargo build --release` and you'll find the executable at `target/release/fakewrite`.
### Releases
Just download a compiled binary from the **releases** page.
## TODO!
I'm planning on implementing the following for future releases:
- [ ] Pretty printing;
      - Currently the logs are a tad ugly.
- [ ] Outputting the logs into a json (or yml) file;
- [ ] Actually blocking the syscalls and fooling the tracee with data from memory or temporary files.
