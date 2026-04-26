# Contributing to netray.info

Thanks for taking an interest. This document is the canonical contribution
guide for the entire netray.info ecosystem. The same file lives in every
public repo (the five inspector tools, lens, the shared libraries, this
meta-repo) and the rules apply identically across them — only the build
commands and issue-filing destination differ per repo (see the repo's own
`README.md` and `CLAUDE.md` for those).

## Project context

netray.info is an open-source network intelligence suite. The **five
inspectors** (IP, DNS, TLS, HTTP, email) and **lens** (the unified domain
health checker that sits at the apex `netray.info`) are independent
services with their own repos. Everything you see at
[netray.info](https://netray.info) — including its hosted instance — runs
on the same code that's published here.

The full product strategy, including how the suite fits together and where
it's heading, lives in
[`specs/product-strategy.md`](https://github.com/netray-info/netray.info/blob/main/specs/product-strategy.md)
in the meta-repo.

## How this project is governed

netray.info is a **maintainer-led project**. The maintainer (Lukas Pustina)
has final say on direction, scope, design, and acceptance. Contributions
are welcome but not guaranteed acceptance — opening an issue first for
non-trivial work is the cheapest way to find out if a change fits the
roadmap.

A commercial monitoring tier is planned in the future. The inspector,
scoring, result-sharing, and self-host layers will remain open source
under MIT indefinitely. The future commercial component will live in its
own repo under its own license; existing public code is unaffected. See
[`specs/product-strategy.md`](https://github.com/netray-info/netray.info/blob/main/specs/product-strategy.md)
§3 *Licensing and contribution posture* for the full reasoning.

## Before you contribute

- **Check open issues** to see if your idea or bug is already tracked.
- **For small fixes** (typos, broken links, obvious bug fixes, dependency
  bumps), open a PR directly.
- **For features, design changes, or anything that affects the public API,
  the scoring profile, or shared types**, please open an issue first to
  discuss before writing code. This protects your time more than ours —
  reviews of "should this exist?" go faster on a paragraph than on a
  thousand-line diff.
- **For security issues**, do not file a public issue. Email the
  maintainer (address in the repo's `Cargo.toml` / `package.json`).

## How to submit a change

1. Fork the repo and create a topic branch off `main`.
2. Make focused, scoped changes. One logical change per PR is much easier
   to review than a sweep that touches many concerns.
3. Follow the project's coding conventions (see *Conventions* below).
4. Write or update tests where the change has logical risk.
5. Run the project's CI-equivalent locally before pushing — typically
   `make ci` or `make test` (check the repo's `Makefile`).
6. Sign off your commits (see *Developer Certificate of Origin* below).
7. Push and open a PR with a clear description: what changes, why, and
   how to verify.

## Conventions

These apply to every repo in the ecosystem. Repo-specific rules layer on
top in each repo's `CLAUDE.md`.

- **Conventional commits.** Subject line under 72 characters, prefix from
  `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, `test:`, `style:`,
  `build:`, `ci:`. Optional scope: `fix(parser):`. Body explains *why*,
  not *what* (the diff already shows what).
- **Scoped commits.** Don't mix formatting changes with functional
  changes. Don't modify unrelated modules "while you're in there."
- **No `Co-Authored-By: Claude`** lines or other AI co-author trailers.
- **Use `example.com`** in docs and test data. No real domains other
  than the project's own (`netray.info` and its subdomains, where
  relevant).
- **No emojis** in code, docs, or commit messages unless explicitly
  requested.
- **No external fonts** anywhere in the suite. System font stacks only.
- **No backwards-compat shims for removed code.** Deletes are deletes.
- **No bypass of failing checks** (`--no-verify`, `#[allow(...)]`,
  `eslint-disable`) without an explanation in the same diff.

## Developer Certificate of Origin (DCO)

By contributing to this project, you certify that the contribution is
yours to submit and that you grant the project a license to use it under
the repo's open-source license (MIT for all current public repos in the
netray.info ecosystem).

Concretely: every commit must include a `Signed-off-by` trailer matching
the author's `git config user.email`. Add it automatically with the `-s`
flag:

```sh
git commit -s -m "feat(parser): add DKIM Ed25519 key parsing"
```

The trailer looks like:

```
Signed-off-by: Random J Developer <random@developer.example.org>
```

This is the standard
[Developer Certificate of Origin v1.1](https://developercertificate.org/),
the same one used by the Linux kernel, Docker, GitLab, the CNCF, and most
major OSS projects. Full text:

> By making a contribution to this project, I certify that:
>
> (a) The contribution was created in whole or in part by me and I have
>     the right to submit it under the open source license indicated in
>     the file; or
>
> (b) The contribution is based upon previous work that, to the best of
>     my knowledge, is licensed under an appropriate open source license
>     and I have the right under that license to submit that work with
>     modifications, whether created in whole or in part by me, under
>     the same open source license (unless I am permitted to submit
>     under a different license), as indicated in the file; or
>
> (c) The contribution was provided directly to me by some other person
>     who certified (a), (b) or (c) and I have not modified it.
>
> (d) I understand and agree that this project and the contribution are
>     public and that a record of the contribution (including all
>     personal information I submit with it, including my sign-off) is
>     maintained indefinitely and may be redistributed consistent with
>     this project and the open source license(s) involved.

PRs without sign-off will not be merged. There is no copyright assignment;
you keep ownership of your contribution. The sign-off is purely a license
grant under the repo's existing terms.

If you accidentally pushed a commit without `-s`, fix the most recent one
with:

```sh
git commit --amend --signoff
git push --force-with-lease
```

Or for a whole branch:

```sh
git rebase --signoff main
git push --force-with-lease
```

## Licensing of contributions

Contributions are licensed under the same license as the repo you're
contributing to — MIT for all current public repos in the netray.info
ecosystem. The DCO sign-off is the contribution mechanism; no separate
CLA is required.

If a future repo is published under a different license (for example, the
planned monitoring service), it will state its license clearly and the
same DCO process will apply.

## Reviewing and merging

The maintainer reviews PRs as time allows. Expect a first response within
a week for substantive PRs; small fixes typically merge faster. There is
no SLA — this is a side project run alongside other work.

Reasons a PR might not merge:

- The change doesn't fit the roadmap. Issues filed in advance prevent
  this.
- The change introduces a maintenance burden disproportionate to its
  benefit (e.g., a config flag with no clear caller).
- The change conflicts with stated conventions (in this file or in the
  repo's `CLAUDE.md`).
- Tests are missing where the change has logical risk.
- The diff is unscoped — formatting + functional + unrelated changes
  bundled together.

A PR that gets closed without merging is not a personal judgment. It's
usually a fit issue. Filing an issue first avoids almost all of these.

## Questions

For project-wide questions, the meta-repo (`netray-info/netray.info` on
GitHub) is the right place to file. For tool-specific questions, file in
the relevant tool's repo. For anything sensitive, email the maintainer.

Thank you for contributing.
