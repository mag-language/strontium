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
- A `Program` struct which decouples the bytecode encoding/decoding logic from the machine.
- A new array containing a collection of structs which implement `Instruction` to abstract instruction execution into modules similar to how Pratt parsing works for text.

### Changed
- Use new banner hosted on Scaleway.

### Removed
- GitLab CI configuration.
- The `Memory` struct was removed in favor of dynamically typed registers.