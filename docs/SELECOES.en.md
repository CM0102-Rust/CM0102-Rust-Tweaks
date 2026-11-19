# Manage any national team (`manage_any_team`)

[Português](SELECOES.md)

A forum request (17/11/2026): with a small national team (the example was Puerto Rico), you can't finalize the call-up, because the nation has fewer players than the minimum. The `manage_any_team = 1` key handles both ends: any main national team can be taken, and the human manager's national team gets grey players. With `grey_signing = 1`, the national team's grey players can be made permanent.

## 1. What the original does

- **Who can be taken** (`club_can_be_managed`, 0x544820, cdecl(team) → al). Used by the apply button, manager hiring, the regens and the team choice when starting a career. A national team (id between total − 2 × nations and total − nations − 1) only passes if the nation has more than 150 people in the database (+0x88, `NumberStaff`; the test at 0x544859). In the 3.9.68 database, Puerto Rico has 9 people and San Marino 12 [measured in `nation.dat`].
- **The team choice** (Setup.cpp, 0x820d40) left national teams out of the list (0x53b900 at 0x8226b1), although it can draw them.
- **Grey players** (`virtual_staff.cpp`): the game creates them before each match, for both teams (0x90d2f0, called from `match_day.cpp`), and when a team's screen is opened (0x90d4f0, called from `club_screens.cpp`; each human manager has one "open" team at a time, and the grey players leave the previous one). A team's creation (0x90dae0), when the team is a national team, asks whether it has a human manager (`0x600a50(team, 1, 1, 0, 0)` at 0x90dc6d) and, if so, leaves without creating any. So the human's national team only had its real players [observed: San Marino with 5 called up and no grey player after loading].

## 2. What the key changes

- Every main national team can be taken (the hook at 0x544820's entry calls the original and, when it refuses a national team of the first set, accepts). Those of the second set (the under-21s) keep the original rule.
- In the team choice, a **National teams** item at the end of the nation selector, which also shows with a single nation; with it, the list shows only national teams. The row of a national team whose nation has no continent (+0x71) crashed; now its text is empty.
- The automatic career (`auto_setup.cfg`, the call at 0x827d0b) keeps the original rule.
- The human manager's main national team gets grey players like a club: the call at 0x90dc6d answers "no human manager" for main national teams.

## 3. Making permanent at a national team

With `grey_signing = 1`, the Make permanent item also shows on the profile of a grey player in the human manager's national team (the manager's +0x24). It works as at a club (the person, the grey player's identity, CA and PA, personality and regen mark), with one difference: **he's called up with no club**, in the grey player's place in the list (+0xd7), with no contract and no wage. The link to the national team (+0x24 the team, +0x28 the job, 0x0b for a player, and the dates up to +0x38) is copied from a real called-up player. Any club can sign him; the retirement of anyone a year without a club and without a national team (0x7a8050) doesn't take him while he's called up.

## 4. The spots in the exe

| Spot | What it does | Change |
|---|---|---|
| 0x544820 (`56 8B 74 24 08`) | who can be taken | hook at the entry |
| 0x827d0b | the automatic career | calls the original |
| 0x8226b9 | the list left national teams out | the `jne` goes |
| 0x8227b1 | a national team's row without +0x71 | new code |
| 0x822698 | the list's filter by nation | new code (the National teams item) |
| 0x82257a | the end of the nation selector | the National teams item |
| 0x822306, 0x822693 | the selector only with more than one nation | with one too |
| 0x90dc6d | the human's national team with no grey players | `humano_na_selecao` |

If any spot of the choice screen differs from 3.9.68, the screen stays as it is (the rule of who can be taken still applies). If 0x544820's entry differs, the key stays off and the log says why. The `jle` at 0x822696 already removed (the current NickPatcher does it) counts as compatible: the 0x822693 spot stays as it is and the rest of the screen turns on ([NICKPATCHER.en.md](NICKPATCHER.en.md)).

**NickPatcher:** no option or MiscPatches patch writes to these spots (`tools/exe/nickpatcher_vs_tweaks.py`). Its `manageanyteam` option changes another byte of the same routine (0x5448aa, the active nation of a club with a division) and doesn't free small national teams; the two live together, because the hook calls the original first.

## 5. Proofs

[TESTES.en.md](TESTES.en.md) §7: San Marino taken when starting the career, grey players in the national team, 19 made permanent, saved and loaded. Still to do: a match on the calendar, to see a match call-up with the players made permanent.
