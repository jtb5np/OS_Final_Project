ironkernel
--------
![][img]

A fork of [rustboot] focusing on ARM functionality and aiming to extend it into a more fully functional kernel. Setup instructions below cribbed also from [rustboot](https://github.com/pczarn/rustboot).  This version includes a simple graphical shell with a window system.

## Setup

You need a few things to run ironkernel:

1. [rust-core]
2. Rust's 0.9 release (hopefully this will be updated to a more recent version of Rust sometime)
3. qemu
4. llvm
5. binutils for arm-none-eabi
6. Optionally for debugging
  * gdb
  * tmux

Clone this repository and update rust-core.

```bash
$ git clone https://github.com/wbthomason/ironkernel.git
$ cd ironkernel
$ git submodule update --init
### you can also pull latest rust-core:
$ git submodule foreach git pull origin master
```

### Arch Linux

Simply install all dependencies:
```
# pacman -S base-devel qemu rust llvm tmux
# yaourt -S arm-none-eabi-gcc
```
Note that you will want Rust 0.9 and LLVM 3.4
### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install qemu
```
### Everyone
Install binutils from source.

```bash
wget ftp://ftp.gnu.org/gnu/binutils/binutils-2.23.2.tar.gz
tar xzvf binutils-2.23.2.tar.gz
cd binutils-2.23.2
export ARMTOOLS=~/arm-none-eabi
./configure --target=arm-none-eabi --prefix=$ARMTOOLS
make all install
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure
$ make && make install
```

## Running it
You may have to make some small changes before it builds. 
Namely, you may need to adjust the rust prefix in the makefile (I did). Hopefully nothing else.
To compile, simply execute `make` command.

To run, use:
```bash
$ make run	# emulate default platform (ARM)
$ make debug # debug on arm
```

Using the graphical shell:

Use the arrow keys to move the cursor around the screen.  Press \ (backslash) to "click" the mouse.  Clicking on the taskbar menu button will show available options.  Currently, only "Open Window", which creates a default window, and "Close All", which closes all windows, actually do anything significant.  Clicking "Show Files" currently does nothing.  Clicking a window in the lower right corner and moving the cursor allows you to resize the selected window.  Clicking the bar at the top of a window and moving the cursor allows you to move the window.  Closing and minimizing windows work more or less as you would expect them to.  Lastly, you can press Tab to switch between open windows.


[rust-core]: https://github.com/thestinger/rust-core
[rustboot]: https://github.com/pczarn/rustboot
[rust]: https://github.com/mozilla/rust
[img]: http://i.imgur.com/9nE81nY.png
