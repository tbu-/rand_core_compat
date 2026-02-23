A compatibility layer for `rand_core` providing adaptation between traits for
each version.

Select the `rand_core`/`rand` versions you want to have compatibility between
using the crate features:
- `rand_core_0_5`: `rand_core 0.5`/`rand 0.7`.
- `rand_core_0_6`: `rand_core 0.6`/`rand 0.8`.
- `rand_core_0_9`: `rand_core 0.9`/`rand 0.9`.
- `rand_core_0_10`: `rand_core 0.10`/`rand 0.10`.

You then most likely want to wrap your RNG, e.g. of version `rand_core
0.6`/`rand 0.8` in a [`Rng06`] struct. It'll implement the `RngCore` traits of
all the other versions you selected via features.
