# Working on The Art of Rust

Read [PROGRESS.md](PROGRESS.md) and [CURRICULUM.md](CURRICULUM.md), inspect
`git status --short --branch` and recent commits, then resume the
recorded chapter or validation step. Keep Zavqiddin Hasanov's
attribution, Rust 2021, Rust 1.75.0, and the December 31, 2023 technical
cutoff. The edition label describes technical scope.

- Keep canonical chapters in `src/part-NN/`. Register chapters that
  have substantive content in `src/SUMMARY.md`. Keep planned work in the
  curriculum and progress records.
- Research each chapter with primary sources. Follow the teaching,
  proof, code, and review protocols in [STYLE_GUIDE.md](STYLE_GUIDE.md).
  Balance educational progression, mathematics, language and compiler
  accuracy, security, performance, editing, and executable verification.
- Keep exercise statements in `exercises/` and complete answers in
  `solutions/`. Publish them with mdBook includes. Give every problem
  a stable ID and every code example a validation classification.
- Run `python scripts/validate.py` and record actual results, deliberate
  limitations, and the exact next step in [PROGRESS.md](PROGRESS.md).
  A successful partial run supports the checks it executed. Mark
  chapters complete only after the relevant gates pass.
- Inspect the scoped diff and check for secrets before making a
  conventional commit. Push completed increments to `main` when
  authorized. Follow branch protection and record any local or remote
  Git blocker. Ask the owner before applying or changing a license.
