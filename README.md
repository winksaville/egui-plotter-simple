# egui-plotter example

This is an example of how to use [egui-plotter](https://github.com/Gip-Gip/egui-plotter/tree/2a4b0609cca8c5918cafa87ebc2770c52c5f7524#examples).

The initial commit didn't compile and was fixed by changing
egui-plotter so eframe and egui are v0.23 instead of v0.22.

I did this by forking egui-plotter by creating a [PR #9](https://github.com/Gip-Gip/egui-plotter/pull/9)
that changes the dependencies to v0.23.

Note: currently the using
```
egui-plotter = { git = "https://github.com/winksaville/egui-plotter.git", branch = "update-eframe-egui-to-0.23" }
```
so it compiles, if `egui-plotter = "0.3"` is used I see a compile `error[E0107]: struct takes 0 generic arguments but 1 generic argument was supplied`

## How to run

```bash
cargo run
```
