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

## Unreleased
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