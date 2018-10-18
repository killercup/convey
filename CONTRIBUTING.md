# Contributing guide

Thanks for your interest in contributing to this project!
We very much look forward to
your suggestions,
bug reports,
and pull requests.

Note:
This project and everyone participating in it is governed by
[this Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## Submitting bug reports

Open an issue in which you:

- Say which version of this library, the Rust compiler, and other dependencies (if relevant) you are using
- Explain what you are trying to do
- Try to provide a small, reproducible example of the issue.

## Submitting feature requests

First,
please check the exising (open and closed) issues
for one that already covers the feature you are proposing.
If there is none,
feel free to open an issue.

In this issue, please describe

- What you want to achive
- How you imaging this library to help you with that
- What needs to changed/added/removed to make this possible.

Ideally,
provide a list of criteria
that needs to be met by a future version of this library
so you can use this feature.

## Submitting a pull request

Do you want to submit a quick typo fix or something similar?
No need to read further, just send a PR!
Thanks!

Before starting to work on anything non-trivial,
please make sure that:

- There is an existing issue describing the feature
- The maintainers agree that this should be implemented
- Nobody else is working on this

In general,
it's a good idea to let somebody know
that you are working on a feature
before sending a PR.

Many of our issues cover multiple aspects,
and it doesn't always make sense
to implement everything in one single PR.
Please nevertheless try to include something like
`Addresses part of #42`
(where `#42` is the issue this feature is described in)
in your PR description and/or commit messages.

### Required setup

To work on this project, you'll need

- At least a beginner level understanding of Rust.
  If this is the first Rust project you contribute to,
  we recommend you read some of the [freely available documentation][rust-docs] first.
- A recent Rust compiler and working cargo
  - Also see the section on code style and CI for additional tools
- A Github account and working knowledge of git
- Basic communication skills in English

[rust-docs]: https://doc.rust-lang.org/

### Code style

Every PR is checked by a continuous integration system, which
compiles the code (`cargo build`),
runs all the tests (`cargo test`),
checks code formatting (`cargo fmt`),
and checks various lints (`cargo clippy`).

You can see the full test suite in [this file][ci].
It makes sense to run all of this locally
before submitting your pull requests or updates to it.

[ci]: .travis.yml

### Commit messages

Your commits and their messages
are what separates total confusion
and easily recognizable indent behind a change
in six months.

Please write them in a very explicit mannor,
and above all describe the indent behind the change you make.
They should be written in full English sentences
and contain examples for usages
as well as describe what you tried to do
and what didn't work.
It's totally fine to write multiple paragraphs for a very small diff.

A commit should be no longer than 200 lines of diff
without a good and explicitly stated reason.
It's recommended to separate larger pull requests
into many small commits
that build upon each other.

### What then?

After submitting your pull request,
it's our turn!

Please bear with us as we try to review your code,
we tend to be quite busy
and this is not the only open source project we maintain.
If no maintainer as responsed after a week,
feel free to ping them by writing a comment,
or reaching out to them on other channels.

Usually,
you'll get some feedback in the form
of comments on the code
or the general architecture of the implementation.
Please push any changes you make
after the first round of feedback
as new commits to the same branch.

## Thanks!

We're very much looking forward to your contributions!
