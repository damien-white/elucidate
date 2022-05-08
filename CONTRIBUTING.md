# Contribution guidelines

Thank you for considering contributing to `elucidate`!

To ensure that your contribution is accepted and merged as painlessly as possible, we ask that you
please follow a few simple steps:

1. Open a new ticket, describing the changes you would like to make.
2. Participate in any discussions if your changes are non-trivial or unclear.
3. Briefly outline any potential breaking changes that would happen as a result of your
   contribution.
    1. If you are not sure about this part, that is okay. We'll figure it out!

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/dark-fusion/elucidate/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/dark-fusion/elucidate/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different from other Rust projects.

```shell
git clone https://github.com/dark-fusion/elucidate
cd elucidate
cargo test
```

### Useful Commands

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
