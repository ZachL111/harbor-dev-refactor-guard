# harbor-dev-refactor-guard

`harbor-dev-refactor-guard` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies refactor behavior through seeded input scenarios, with deterministic summary checks and bounded memory input sets.

## Why It Exists

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Harbor Dev Refactor Guard Review Notes

`stress` and `baseline` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Features

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/harbor-dev-refactor-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `diagnostic quality` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture Notes

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The added Rust path is deliberately direct, with fixtures doing most of the explaining.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The same command runs the local verification path. The highest-scoring domain case is `stress` at 240, which lands in `ship`. The most cautious case is `baseline` at 166, which lands in `ship`.

## Limitations And Roadmap

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
