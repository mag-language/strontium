# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
    Add new changelog entries here.
    Each entry may be annotated with "Added", "Changed", "Removed", and "Fixed" titles.

    Example:

    ## [1.0.0] - May 16, 2022

    ### Added
    - New visual identity.

    ### Changed
    - Start using "changelog" over "change log" since it's the common usage.

    ### Removed
    - Section about "changelog" vs "CHANGELOG".

    ### Fixed
    - Fix typos in recent README changes.
    - Update outdated unreleased diff link.
-->

## [0.9.1] - September 13, 2026

### Changed

- No changes to Strontium itself; released together with `magc` and `mag` 0.9.1 to keep the versions in sync.

## [0.9.0] - September 13, 2026

### Added

- `LoadType` instruction and `LoadTypeExecutor`, which store a register's type tag in another register for type-based dispatch.
- `CONCAT` calculation method for string concatenation.
- `InterruptKind::Panic`, which stops execution with an error message. The compiler uses it when no multimethod variant matches a call.

### Changed

- `Print` interrupts show `nothing` for empty registers instead of printing no output at all.

### Removed

- Runtime multimethod dispatch: the `Dispatch` opcode, `DispatchExecutor`, `MultimethodTable`, `register_method`, `dispatch`, and `DispatchPattern`. Dispatch is now compiled into bytecode by `magc`.

## [0.8.0] - May 10, 2026

### Added

- `DispatchPattern::Type(RegisterType)` variant for type-based multimethod dispatch — methods annotated with a type (e.g. `def foo(n Int)`) now generate a type-matching pattern instead of a wildcard, allowing multiple methods with the same name but different argument types to coexist correctly.
- `JumpCExecutor`: executes `JumpC` by jumping to the destination when the conditional register holds `Boolean(false)`, enabling compiled `if/then/else` branches.
- `CancellationToken`: shared atomic flag allowing embedders to interrupt VM execution mid-run (used by the REPL to handle Ctrl-C).

### Fixed

- Boolean values were encoded with type tag `4`, colliding with `Int32` (tag `4`). The encoder now uses the correct tag `13`, matching the decoder.

## [0.7.0] - May 9, 2026

### Added

- `DispatchPattern` enum representing either a specific value match or a wildcard (`Any`), used for multimethod dispatch at runtime.
- New instruction executors: `CopyExecutor`, `DispatchExecutor`, `JumpExecutor`, `LoadLocalExecutor`, and `StoreLocalExecutor`.
- A `--debug` flag to control VM output verbosity.
- An actual bytecode format so we can convert `Instruction`s to bytes.

### Fixed

- Interrupt handling reliability issues.

## [0.6.0] - October 22, 2023
### Added
- Start using [human-readable changelogs](https://keepachangelog.com/en/1.0.0/).
- The `Program` struct, which represents a parsed bytecode executable, along with utility methods to facilitate the conversion between binary file format and in-memory representation.
- The `Executor` trait, which modularizes instruction execution by associating opcodes with pieces of code executing instructions, along with the following new implementations:
  - `HaltExecutor`
  - `LoadExecutor`
  - `CalculateExecutor`
  - `InterruptExecutor`
- A new `executors` hash map in `Strontium` associating opcodes with `Executor` implementations.
- A `Registers` struct which stores register values and allocates new slots if needed.
- A `RegisterValue` enum which provides type tags for registers.
- Conversion utilities to convert `Instruction`s to a vector of bytes.

### Changed
- Replace the old `Memory` struct in favor of typed registers for atomic and complex data types. All virtual machine state is now stored in the in the `registers` property of the `Strontium` struct, including the bytecode and instruction pointer.

### Removed
- GitLab CI configuration.