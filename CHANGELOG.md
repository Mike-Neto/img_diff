### 5.0.0

#### Breaking

Reworked error handling by creating img_diff own Result & Error types.

#### Dependencies

- updated image@0.21.2 to image@0.22.2
- updated criterion@0.2.11 to criterion@0.3.0
- updated structopt@0.2.18 to structopt@0.3.6
- updated predicates@1.0.1 to predicates@1.0.2
- updated assert_cmd@0.11.1 to assert_cmd@0.12.0
- swapped tempdir@0.3.7 (deprecated) to tempfile@3.1.0

#### CI

Dropped FreeBSD support (following on cross)

### 4.0.0

Removed Dssim and a bunch of other dependencies.

Now the value shown is a percentage. More details above.

Uniform value no matter the image type.

### 3.0.2

Fixed some issues and migrated to using tools as recommended by the CLI WG

- Migrated to StructOpt
- Migrated to assert_cmd
- Added human friendly panic

Removed all unwraps and provide error messages.

Updated dependencies.

More typo fixes.

Updated future features with things from the CLI WG suggestions.

### 3.0.1

Formatted using cargo fmt.

Fixed clippy issues.

Fixed typos and updated docs.

Updated dependencies.

### 3.0.0

Removed Multi-threaded flag making that the default.

Upgraded to Rust Edition 2018
