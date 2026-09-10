# purust-unfoldable

Native Rust array implementations of PureScript's `Unfoldable` and `Unfoldable1`
classes for Purust.

`unfoldr` calls its step until it returns `Nothing`. `unfoldr1` first appends the
value returned by its step, then checks whether another state exists. Both
implementations use an iterative loop and preserve element order. They accept
polymorphic values, including records, tuples, arrays and functions, with the
typed Maybe and Tuple callback ABI emitted by Purust.

The PureScript modules provide `none`, `singleton`, `replicate`, `replicate1`,
`replicateA`, `replicate1A`, `range`, `iterateN`, and `fromMaybe` without changes
to their original semantics. Nonempty operations produce at least one element,
including when their requested count is zero or negative.

## Tests

```bash
bin/test
bin/test -c
```

The Bash runner uses Spago from the sibling `purust` compiler and prefers the
TAST compiler fork in `../../purescript`. Set `PURS` to select its executable.
Every run regenerates this package's TAST and Rust output. `-c` also rebuilds
Purust and clears this package's Spago cache.

`test/Main.purs` preserves all 17 assertions and the Collatz example from
`gopurs-unfoldable` unchanged. Seven additional test groups cover empty and
terminal steps, structured states and values, produced functions, count
boundaries, descending ranges, Maybe instances, deferred and replayed effects,
and 10,000-element unfolds. The runner checks exact stdout and requires empty
stderr. Native tests also verify both FFI functions' step counts and termination
in the default Rc mode and the optional Arc mode.
The `replicate1A` prerequisite in Arrays also has native coverage for deferred
effects, replay, Cartesian order, and a 20,000-element traversal on a 256 KiB
stack in both modes.

## Installation

Add `unfoldable` to your package dependencies and select this native package:

```yaml
workspace:
  extraPackages:
    unfoldable:
      path: ../purust-unfoldable
```

Module documentation is [published on Pursuit](https://pursuit.purescript.org/packages/purescript-unfoldable).
