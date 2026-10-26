# Embedded recordings and attribution

These licenses cover the recordings only. noizen's source code is licensed
under MIT; that license does not relicense the recordings. No Blanket application
code is copied; Blanket's application license is not the license for these recordings.

## Rain

“rain ambience” by **alex36917**, original:
https://freesound.org/people/alex36917/sounds/524605/

Licensed **Creative Commons Attribution 4.0 International**:
https://creativecommons.org/licenses/by/4.0/ (legal text: `CC-BY-4.0.txt`).
The original page explicitly permits redistribution/remixing with attribution;
its HTML links to the 4.0 license. Verified during implementation.
Edited for Blanket by **Porrumentzio**; supplied as Vorbis by Blanket.
Original description: light/medium rain, possible thunder and distant train horn.
No endorsement by the authors is implied.

## Fireplace

“Fireplace” by **ezwa**, original:
https://soundbible.com/1543-Fireplace.html

**Public Domain**, as explicitly stated on the original source page (verified
during implementation). Real fire with small crackles and air movement.
Supplied as Vorbis by Blanket.

## Exact acquisition and modifications

Both files copied unchanged from Blanket commit
`775f2a767230a9681850d1b1e085be58656f1382`:
https://github.com/rafaelmardojai/blanket/tree/775f2a767230a9681850d1b1e085be58656f1382/data/resources/sounds

License/author/editor manifest:
https://github.com/rafaelmardojai/blanket/blob/775f2a767230a9681850d1b1e085be58656f1382/SOUNDS_LICENSING.md

SHA-256:
- rain.ogg: b84c1daf259a24283be40b7663656d17676df4eb60c1e0332820570db891848a
- fireplace.ogg: a6395379a32c2dc30dd870c4571ded4a5311435aacfe354b3a82a9b3ab98620b

noizen decodes to mono 44.1 kHz and overlaps the last/first 100 ms with a linear
crossfade in memory; the embedded files remain unchanged. Distributions must
retain this attribution and the CC BY license notice. Release archives ship
these notices alongside the executable.
