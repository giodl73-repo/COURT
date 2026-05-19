# FONTES MIT Game-Course Research for COURT

## Research question

What should COURT prove before becoming a scalable experience framework, using
the local FONTES MIT game-course corpus as the design baseline?

Decision supported: COURT foundation specs and role review before expanding
`court-core` or RACKET.

## Local corpus read

The relevant FONTES MIT game-course packages are:

| Course | Local sources read | Primary implication |
|---|---|---|
| CMS.300 Introduction to Videogame Studies | `sources/mit/ocw/cms-300-videogame-studies/*`, `sources/tables/mit-cms-300-surfaces.json` | COURT needs analysis vocabulary for player identity, narrative, simulation, values, aesthetics, journalism, and game criticism. |
| CMS.301 Introduction to Game Design Methods | `sources/mit/ocw/cms-301-game-design-methods/*`, `sources/tables/mit-cms-301-surfaces.json` | COURT needs iterative design, critique, playtesting, documentation, and research-facing workflow hooks. |
| CMS.608 / CMS.864 Game Design | `sources/mit/ocw/cms-608-game-design/*`, `sources/tables/mit-cms-608-surfaces.json` | COURT needs rules clarity, mechanics/dynamics/aesthetics, player experience, randomness, multiplayer, simulation, social play, and team design practice. |
| CMS.611J / 6.073 Creating Video Games | `sources/mit/ocw/cms-611j-creating-video-games/*`, `sources/tables/mit-cms-611j-surfaces.json` | COURT and RACKET need production, project-management, focus-testing, QA, UI, postmortem, and scope-control gates. |
| CMS.615 Games for Social Change | `sources/mit/ocw/cms-615-games-for-social-change/*`, `sources/tables/mit-cms-615-surfaces.json` | COURT needs impact/intent assessment and explicit limits for serious or civic game experiences. |
| 11.127J / CMS.590J Computer Games and Simulations | `sources/mit/ocw/11-127j-computer-games-and-simulations/*`, `sources/tables/mit-11-127j-surfaces.json` | COURT needs educational/simulation goals, field testing, learning assessment, board/digital documentation, and project-based learning support. |

## Findings

### COURT-01: Rights boundaries are part of the framework contract

Source references:

- `cms-300-videogame-studies/custody.md` lines 15-20 and
  `sources/tables/mit-cms-300-surfaces.json` lines 7-14.
- `cms-608-game-design/custody.md` lines 15-20 and
  `sources/tables/mit-cms-608-surfaces.json` lines 68-85.
- `cms-611j-creating-video-games/custody.md` lines 16-22 and
  `sources/tables/mit-cms-611j-surfaces.json` lines 68-105.

Observation: OCW-owned pages/resources are reusable as derived text under the
recorded policy, but readings, commercial games, student builds, external
references, and media bytes are boundaries.

Implication: COURT specs must model provenance and boundary metadata for
experience nodes. A scene object, assignment prompt, game reference, media item,
or student build cannot be treated as the same kind of reusable asset.

Confidence: high.

### COURT-02: Experience design starts with analysis, not rendering

Source references:

- `cms-300-videogame-studies/work.json` lines 13-14 and
  `cms-300-videogame-studies/custody.md` lines 10-20.
- `cms-300-videogame-studies/inventory.md` lines 21-38.

Observation: CMS.300 foregrounds game analysis, cultural framing, player
identity, narrative, simulation, value systems, aesthetics, violence,
journalism, and criticism.

Implication: COURT should carry analysis tags and review questions on
snapshots/scenes, not only drawable entities. RACKET can render, but COURT must
preserve why an experience exists and how it should be read.

Confidence: high.

### COURT-03: Iteration and critique must be first-class

Source references:

- `cms-301-game-design-methods/work.json` lines 13-14 and 37-48.
- `cms-301-game-design-methods/custody.md` lines 10-21 and 36-40.
- `cms-301-game-design-methods/inventory.md` lines 23-39.

Observation: CMS.301 emphasizes rapid prototyping, playtesting, design
iteration, player-centered design, crit sessions, peer feedback, documentation,
and research-facing practice.

Implication: COURT should define experience revisions, playtest observations,
critique states, and design-document hooks. A snapshot/action API alone is too
thin if it cannot track why an experience changed.

Confidence: high.

### COURT-04: Rules clarity is a usability feature

Source references:

- `cms-608-game-design/work.json` lines 13-14 and 44-55.
- `cms-608-game-design/custody.md` lines 10-20 and 36-41.
- `cms-608-game-design/inventory.md` lines 23-40.

Observation: CMS.608 centers non-digital game design, rule writing, iterative
prototyping, playtesting, MDA, player experience, randomness, multiplayer,
simulation, social play, and team design.

Implication: COURT needs explicit rule/action affordances: legal actions,
illegal-action explanations, written-rule surfaces, testable player goals, and
simulation boundaries. RACKET should never infer game rules from visuals.

Confidence: high.

### COURT-05: The first real engine must prove production workflow, not just drawing

Source references:

- `cms-611j-creating-video-games/work.json` lines 13-14 and 37-55.
- `cms-611j-creating-video-games/custody.md` lines 10-22 and 38-43.
- `cms-611j-creating-video-games/inventory.md` lines 24-41.

Observation: CMS.611J highlights small-team production, project management,
game-engine onboarding, focus testing, data analysis, QA/bug reporting,
postmortems, UI work, scope control, serious-game projects, and presentation.

Implication: RACKET's first engine proof should include frame plans, input, and
smoke tests, but the COURT/RACKET roadmap should also require focus-test
scripts, bug/report capture, backlog scope, postmortem records, and UI
inspection before claiming "engine" maturity.

Confidence: high.

### COURT-06: Impact and learning goals need explicit assessment hooks

Source references:

- `cms-615-games-for-social-change/work.json` lines 13-14 and 30-41.
- `cms-615-games-for-social-change/custody.md` lines 10-19 and 32-37.
- `11-127j-computer-games-and-simulations/work.json` lines 13-14 and 37-48.
- `11-127j-computer-games-and-simulations/custody.md` lines 10-19 and 33-38.

Observation: CMS.615 and 11.127J extend design into social-change games,
educational games, simulations, field testing, learning assessment, and
project-based learning.

Implication: COURT should separate "experience objective" from "rendered
objective." A product may need a learning goal, impact claim, simulation claim,
field-test target, or civic/educational assessment that survives across browser,
native, and authored-scene engines.

Confidence: high.

## Recommendations

### Adopt now

1. Add COURT foundation specs for provenance, action/rule affordances,
   scene/UX intent, playtest observations, and assessment hooks.
2. Add COURT role review before adding more framework code.
3. Keep RACKET as a consumer of COURT contracts, not a rule owner.

### Prototype behind a compatibility boundary

1. Typed action model with legal/illegal/recoverable states.
2. Scene-node provenance and rights-boundary metadata.
3. Playtest/focus-test event records.
4. Assessment hooks for learning, impact, or simulation claims.

### Reject or defer

1. Do not make COURT renderer-first.
2. Do not copy MIT third-party readings, commercial-game content, student builds,
   or media bytes into COURT/RACKET.
3. Do not claim full engine maturity until production workflow and testing gates
   exist, not only rendering.

