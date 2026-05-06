# Harbor Dev Refactor Guard Walkthrough

This walk-through keeps the domain vocabulary close to the data instead of burying it in prose.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 166 | ship |
| stress | diagnostic quality | 240 | ship |
| edge | review cost | 173 | ship |
| recovery | safe rewrite | 207 | ship |
| stale | change width | 184 | ship |

Start with `stress` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

If `baseline` becomes less cautious without a clear reason, I would inspect the drag input first.
