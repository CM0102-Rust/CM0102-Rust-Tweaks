# New people: the feasibility study (11/04/2026)

*[Português](PESSOAS_NOVAS.md)*

Lucas's request: "the solution is to create people; the question is whether it's feasible, the effort doesn't matter". A read-only study (disassembly of the whole `.text`, pseudocode and saves; nothing changed, the game wasn't run). It follows on from [FORNADA_DE_REGENS.en.md](FORNADA_DE_REGENS.en.md) §5: in a long career, what limits regens is the slot (the person), not the record.

Tags: **[observed]** in the code or the saves; **[inference]** from the flow, to be proved on the test bench.

## 1. The answer

**It's feasible, in two ways.** No table, save item or editor ties the person count to a fixed number [observed]. The hard part is the 16 human managers, whom the game identifies as the last 16 people in the table.

| | Path 1: a stock made at creation | Path 2: grow in any save |
|---|---|---|
| How | the game already creates extra people when it sets up a career; the DLL asks for K more, keeps them dormant and releases them through the regen queue | the person table grows mid-career, after the humans |
| Patches | 1 hook in the database trim, plus the marking and the release | ~276 for the humans, ~15 for the grey players, ~8 vectors that grow along |
| Works for | new careers | new careers and existing saves |
| Save | same format, opens without the DLL | same format, but only opens correctly with the DLL; CMRTE gets the humans wrong |
| Limit | K fixed at creation (~20,000 without touching the allocation; more by enlarging the `malloc`) | no practical limit |

**Recommendation:** path 1 first. It fits in the two weeks and is already a real youth intake for new careers. Path 2 comes later, for existing saves, and reuses everything from path 1.

## 2. What the game already does at career creation

The database trim (`database_trim_staff_tables` 0x524d70) keeps the people of the chosen nations and **creates extra people**: 1%, or 2% with an option, of those it keeps. The calculation is at 0x525f84..0x525f8c (`fild`/`fmul`/`ftol`) [observed]:

- each person created gets a player record and preferences, is born in 1982 and has no club;
- they go **before** the 16 humans, and the trim moves the humans to the last 16 rows;
- everything after that (the 79-byte table, the subsystems, PlayerStats, training) is born with the final count.

**A hook on that `ftol` gives K extra people**, with records, and no patch at all for the humans, the grey players or saving, because the count never changes afterwards [the code is observed; the hook is a proposal].

Limits [observed]:

- the trim's `realloc` aborts if the block moves, so the natural headroom is about 20,000 (out of the 23,000 reserve). Beyond that, enlarge the `malloc` at 0x529e02 (people) and 0x529e11 (records);
- the cap at 0x525fa4 has a bug: if K goes over the limit, it adds instead of subtracting (`lea ebx,[esi+eax+0x52d]`), and the extra becomes huge. The hook fixes that.

## 3. Path 1 in detail

1. **Create:** the hook on the `ftol` returns K (for example, the sum of the quotas for the planned seasons).
2. **Put to sleep:** right after `create_game` 0x828110, before `game_run_to_start_date` 0x5cc850, the DLL marks the K people as retired: no club, with bits 0x30 of the 79-byte record ≠ 0. That's the same state as the 1,260 people waiting in the queue for a record in the test saves, so it's a state the game already knows [observed]. Their records go into the pool at the right time.
3. **Release:** on promotion day, the DLL puts the slots in the queue (`queue_staff_for_regen` 0x7a87d0) and the records in the pool; the game makes the regen (0x7abac0) and signs him with the quota's club (FORNADA_DE_REGENS §4.3).
4. **Save and load:** nothing to do; the count is the usual one.

To check:

- whether a dormant person passes inertly through the 104 loops over all people (the state is the queue's, but for years, not weeks);
- whether `game_run_to_start_date` already creates grey players (doesn't matter for path 1; matters for path 2);
- the cost per day with a large K: the loops get longer in proportion to the people.

## 4. Path 2 in detail

### 4.1 The humans

1,155 uses of the counts in the exe; 960 of them are the person count `[0xae2364]` [observed, a sweep of the whole `.text`, checked by byte search]:

| Use | In game | At creation and load |
|---|---|---|
| loop over all people | 104 | 11 |
| id bound/validation | 423 | 1 |
| human | 275 | 77 |
| allocation | 19 | 1 |
| grey players | 8 | 0 |
| save/load | 20 | 6 |
| other (random draws, ratios) | 15 | 0 |

With a new global `CH` ("end of the humans", fixed) replacing the count in the human uses:

- **191 places** only swap the 4 bytes of the address they read: lower and upper bounds, ids built from the human's index, no test;
- **76 places** only test `id ≥ count − 16`, relying on the implicit upper bound (above the count is a grey player). A new person after the humans would pass as a human, so each becomes a hook;
- **8 loops** over "everyone but the humans" stop at count − 16 and get rewritten: 0x5d6ef0, 0x69bcd1, 0x69c13a, 0x69c549, 0x7a5936, 0x7a6eab, 0x7a6ee9, 0x83a10d (and maybe the debug loop at 0x52d20a).

The 16-entry tables indexed by the human's index (HUMAN_MANAGER, awards, scouts, training, notes, grey players, fog, Staff_Records and the globals 0xb6b7d0, 0xb73390, 0xadab50) stay correct with `CH` [observed].

**Moving the humans to the end (renumbering)** was ruled out: their ids sit in news, jobs, searches, awards, contracts, histories and `club.dat`, and the list never closes. One miss silently corrupts the career.

**`CH` at load:** the humans have classification +0x5f = 0 (Tweaks already uses this), so it can be recomputed without a new file [inference].

### 4.2 The grey players

- **Lifetime:** they're created per match (0x6b43f8, 0x6b4a43, 0x6b5bc1, 0x84c339) and per squad screen (0x484215), and removed after the match (0x90d6b0), when the club signs someone (0x90d860) and at load (resolve 0x5315a0 clears the squads) [observed].
- **Space:** 53 blocks of 25 = 1,325 rows starting at the count; in a one-human game, about 7 blocks are alive at once [observed].
- **The ~400 `id ≥ count` tests** only mean "not a real person" and stay correct with any virtual base above the count. **Only `get_id_block` 0x90e760 depends on where the virtual area sits** [observed].
- **Fixed base:** 14 address swaps at 0x90e791..0x90e984 and one at 0x897a4f (Tactics). The alternative is to grow only when no grey player is alive.
- **A bug in the original, found along the way:** `relocate_virtual_players` 0x90e9f6 uses a signed comparison and never moves match grey players up to the viewed club's block, so two squad slots can point to the same row. That's the source of the ids ≥ N in the `TesteTelas` squads [observed].

### 4.3 The per-person tables

They have one entry per person, grow along (`realloc` at the start of the day; nobody keeps a pointer into them) and, at load, require exactly N records [observed]:

| Structure | Per person | New entry |
|---|---|---|
| the 79-byte table `[0xae2d0c]`, count `[0xae2d10]` (`general.dat`) | 79 B | the `create_game` template (0x82854b), with bits 0x30 = 0x20 |
| Discipline +0x14 (`discipline.dat`, 0x5559d1) | 29 B | 28 bytes of 0xff and +0x1c = 0 |
| Injury +0 (`injury.dat`, 0x62ac35) | 31 B | as at creation (0x62d031) |
| fog of war (`fog.dat`, 0x5b009e), per human | 9 B | level from 0x5b03e0, today's date |
| TRANSFER_MANAGER +0x86e (`transfer.dat`, 0x8bcb16) | 8 B | zeros |
| Staff_Records +0xc (`staff records.dat`) | 8 B | −1, −1 |
| AWOL +0xc, DISPUTE +4 | 1 B | zero |

Everything else is a list with ids inside (contracts, news, scouts, national squads, offers, awards, histories) and doesn't need to grow. The SearchEngine is allocated when the search screen opens: don't grow while it's open.

### 4.4 The person and record tables

They can't move: there are pointers to people in the squads, the preferences, HUMAN_MANAGER (0xb74c26), the people themselves (+0x61) and the matches [observed]. The headroom is reserved in the load `malloc` (0x529e02, 0x529e11), before the resolve, and in the trim's `realloc` (0x526003). At load, everything per person is only ready when `load_game_data` 0x82a9c0 returns; that's where, with no grey player alive, the count can go up.

## 5. The save and the editors

- **Saving:** each item writes the in-memory count; `index.dat` carries each table's count; the header and the tail don't depend on it. The format doesn't change [observed].
- **Editors:** SGE (the original and the rewrite), SaveChanger and Scouter take the count from the item size; CMExplorer from `index.dat`; CMRTE reads the count live but assumes the last 16 people are the humans [observed in the available sources]. **With path 1, they all work. With path 2, CMRTE gets the humans wrong.**

## 6. The size of the work

- **Path 1:** one hook in the trim, marking the dormant people, releasing them through the queue (with the fixed day and the quota from FORNADA_DE_REGENS) and the bench proofs (a career with K = 20,000: stable, time per day, save, editors). Fits in two weeks [inference].
- **Path 2:** ~276 patches for the humans (191 mechanical, which a tool can generate from the list; 76 hooks; 8 loops), ~15 for the grey players, 8 vectors that grow along, and proving each one. More than two weeks, but no known blocker [inference].

## 7. To check before coding

- The dormant person, for years, in the 104 loops (path 1).
- The debug loop at 0x52d20a; reading the 76 hooks one by one; `CONTRACT_MANAGER` +4; person pointers kept in the subsystems (path 2).
- What the "database size" values in `general.dat` mean.

The full reports and lists (the 1,155 uses, one per line) stayed in the session's scratch folder; they'll go into the repository when the code starts.

## 8. Path 1 on the test bench (11/04/2026, experimental)

**The name:** "fornada" comes from Football Manager's European Portuguese translation. In English FM it's the *Youth Intake* (the day, *Youth Intake Day*), and the keys follow that name (`intake_*`); in Spanish FM, *hornada de juveniles*.

The code is in `src/regens/fornada.rs`. Released in version 1.2 (11/05/2026, with Lucas's OK), still experimental. It's off without the keys, so nothing changes in the game for anyone who doesn't use them.

**The final design (11/05, evening; settled with Lucas point by point). It replaces the 11/05 afternoon one.** The principle: **a regen is a regen**. Where it comes from doesn't matter, and the program doesn't treat regens from different sources differently. The intake changes two things: the date (regens only arrive on intake day) and the number (every club with room gets `intake_per_club`, 3 by default). Everything else is the Regens tab.

1. **The date.** With the intake on, whoever retires waits in the game's queue, and regens are only born on intake day. The default is each country's season end (`season_end`); a fixed day is an option. Retirements can go to a single day (`retire_day`).
2. **The quota.** On the day, every club of the playable leagues (the background ones too, with `intake_clubs = selected`) has a quota of `intake_per_club`, limited to the squad room (50): with 2 places, 2; with none, none, and the human manager gets the no-room message. Room is counted on the day: whoever releases players up to the day before gets them. The human club gets them by the same rule.
3. **The day's regens.** The retired players of the day's nations are all born through the game's routine (`regenerate_as_youth_player` 0x7abac0: the inherited record's PA, positions, feet and technical and mental profile; CA, physicals, age, name and languages drawn), without going to the contest yet. The nation's free regens up to 21 (left over from earlier intakes) join them.
4. **The missing ones.** Retirements aren't enough for 3 per club (in the 10-year career 136 to 419 a year were missing with 278 clubs; with all leagues, about 4,200 a year). The missing ones are born the same way: the person comes from the reserve created with the career (hidden, 50 thousand asked; the game gives what fits in the database trim's room, 120 thousand with Brazil and England); the record is a mix of two records of the nation's retired players **of the same position** (goalkeeper with goalkeeper, defender with defender, midfielder with midfielder, forward with forward): positions and feet from one, each attribute from one of the two, and the PA drawn between the two PAs. The parents are the regens just born from retired players (their record is the retired player's), so the created ones follow the PA and position distribution of those who retire: the nation's own; with fewer than 50 that day, the other nations' of the day too (the regen's nation stays the club's); with fewer still, the nation's players aged 30 or more. Then the same 0x7abac0. Mixing avoids clones: copying one record would repeat the PA, positions, feet and technical and mental profile. (In the tests, active players as parents made the created ones too strong, median PA 112 to 118 against 70 to 80 for the retired; with the parents now, 71.)
5. **The distribution.** The nation's whole group (the day's, the free ones and the created ones) is handed out at once, from the highest PA down, each drawn by the Regens tab's curve (level, facilities, staff, surprise, temperature) among the clubs that still have quota. Every club ends with its quota; the curve decides **which** regens go to each. With the intake on, the season cap (`season_cap`) and squad room (`squad_limit`) don't apply: the quota is already the limit.
6. **The leftovers.** The day's regens left once every quota is full go to the nation's normal contest, among the clubs with room; with no club with room, they stay without a club, as in the original.
7. **Outside the guarantee.** Nations not chosen (and the background ones, by default) follow the game's original path.
8. **The message.** The human manager's club gets the list of who arrived, with the coach's view; with no room, the message says nobody arrived for lack of room. The phrases are in `intake_news.txt`.

What went (parallel rules, for stock youngsters only, that make no sense once a regen is a regen): `intake_pa_min`, `intake_pa_max`, `intake_pa_spread`, `intake_surprise`, `intake_squad_max`, `intake_only` and `intake_stock` (the reserve is fixed and hidden). The key that turns it on is `intake = 1`.

**Known limit:** the reserve fits in the database trim's room (the people and records of the nations left out of the career). With every league chosen there's little record room left; raising the allocation (0x529bfe, records; 0x529b39, people) waits until a career with every league shows the shortfall.

**The final design's result (11/05, evening).** Automatic holiday career, Brazil, England and Italy (278 clubs), intake at each country's season end (the default), from 2001 to 29/03/2008 (it stopped on a close-window box, not the DLL's; six and a half years were enough) [observed, `resultados/final5longa.log`]:

| Intake | Clubs | Born from the queue | Created | Given | No room | Leftovers signed | Reserve left |
|---|---|---|---|---|---|---|---|
| 2002 (BRA, day 10) | 112 | 1 | 335 | 336 | 0 | 0 | 49,665 |
| 2002 (ENG/ITA, day 169) | 166 | 413 | 117 | 436 | 11 | 42 | 49,548 |
| 2003 BRA | 112 | 168 | 168 | 336 | 0 | 0 | 49,379 |
| 2003 ENG/ITA | 166 | 749 | 0 | 404 | 18 | 298 | 49,379 |
| 2004 BRA | 112 | 135 | 201 | 336 | 0 | 0 | 49,178 |
| 2004 ENG/ITA | 166 | 442 | 105 | 403 | 13 | 91 | 49,073 |
| 2005 BRA | 112 | 130 | 206 | 336 | 0 | 0 | 48,867 |
| 2005 ENG/ITA | 166 | 534 | 93 | 446 | 9 | 97 | 48,774 |
| 2006 BRA | 112 | 153 | 180 | 336 | 0 | 0 | 48,593 |
| 2006 ENG/ITA | 166 | 697 | 0 | 492 | 1 | 148 | 48,593 |
| 2007 BRA | 112 | 193 | 140 | 336 | 0 | 0 | 48,452 |
| 2007 ENG/ITA | 166 | 840 | 33 | 498 | 0 | 244 | 48,419 |
| 2008 BRA | 112 | 216 | 114 | 336 | 0 | 0 | 48,305 |

- Every club with room got its quota in every intake: no quota missing, no nation without parents. The 50,000 reserve was created in full (room 120,467); six and a half years used 1,695, about 200 years at the same pace.
- Created regens only come in when the retired players aren't enough: in Brazil, half the intake or more; in England and Italy, none to 117.
- The leftovers are England's and Italy's, where more retire than the quotas add up to; they go to the normal contest, as agreed. That's why a club goes past 3 a year: the most regens at one club went from 5 (2002) to 27 (Varese, 2007).
- Average squad 35 to 38 (minimum 5 to 10, maximum 50 or 51); regens with PA 150 or more at each season end: 25, 42, 96, 165, 236, 339.

**All leagues, 10 years, before the fixes (11/06).** The same setup, now the shipped `tweaks.cfg`'s, with every nation of the original game selected (1,134 clubs), from 2001 to 2011-199, no crashes and no error boxes [observed, `resultados/todas10anos.log`]. The day-169 intake (most of Europe's season end):

| Year | Born from the queue | Created | Given | Missing | No room | Leftovers signed | Reserve left |
|---|---|---|---|---|---|---|---|
| 2002 | 1,176 | 1,068 | 2,121 | 66 | 36 | 91 | 47,919 |
| 2004 | 1,439 | 948 | 2,128 | 66 | 25 | 282 | 45,658 |
| 2006 | 2,411 | 445 | 2,313 | 35 | 0 | 656 | 44,041 |
| 2008 | 3,249 | 241 | 2,316 | 36 | 1 | 1,157 | 43,172 |
| 2011 | 3,055 | 118 | 2,294 | 43 | 3 | 876 | 42,372 |

- The 50,000 reserve fit with all leagues and used 7,600 in 10 years.
- Average squad from 29.8 (2001) to 36.3, steady from 2005 on; maximum 50 to 52.
- Regens with PA 150 or more at each season end: from 20 (2002) to 1,008 (2011).
- Two long stalls (over 60 s on one date, at 2001-194 and 2010-169), which the game got past on its own.
- **Save and load:** the 2011 save (on intake day) loaded with the reserve intact (42,372), and the next intake matched the continuous career's [observed, `resultados/todas10anos_carga.log`].
- **Missing** were clubs with room that didn't get their whole quota: 35 to 80 of 1,134 a year. The DLL now logs each one's reason (`intake-why` line) and, in a short all-leagues career (intake on day 200 of 2001), there were 119 places, for four reasons [measured, `resultados/porque_falta*.log`]:
  1. **B teams** (the partner via 0x540a50): the curve sends them no regens. They left the quota.
  2. **Creating from the reserve lost regens:** ~80 per intake came out of creation already at a club (the game placed them) and the place stayed empty. The DLL now tries again, up to 3 times per place. The `intake-diag` line counts the cases.
  3. **The squad counted with the B team's:** the game's squad count (0x53c590) adds the A team and the B team. In Spain, Portugal and Germany, A + B goes past 50, and Real Madrid, Porto and Bayern looked full. In the intake the limit is now the club's own 50 places.
  4. **The Basque clubs:** Athletic Bilbao and Eibar only sign Basques (0x53d7f0, reason 0x19 of 0x8ce990); Real Sociedad signs no Spaniard who isn't Basque (reason 0x1a). When only they still have a quota, the Spanish youngster gets Basque as second nationality (nation 0x9cf424) and goes to one of them. Eibar needed one more change: the Basque-regen veto of the game's distribution (0x7ad0e0) only lets him go to 5 clubs, and Eibar isn't one of them. Since 1.3, the DLL's veto (in the intake and in the `rule = new` contest) also accepts the clubs the game itself marks as Basque-only (0x53d7f0: Athletic and 21 more, Eibar among them) and checks the first nationality as well as the second (see `NICKPATCHER.en.md`).

  With the four fixes: all 1,134 clubs with their whole quota, 0 missing, 9 youngsters made Basque. At season end (the real intake, to July 2002, `resultados/fim_temporada.log`): the 9 intakes with 0 missing; day 169's, 788 clubs, 9 made Basque. History checks out: Athletic has fielded only Basques since 1912; Real Sociedad dropped the rule in 1989 (Aldridge) but only signed a non-Basque Spaniard in 2001 (Boris); for Eibar there's no firm source, and the DLL follows what the game says about it.
- **The sink club** (Guiseley with 41 regens in 2003) came from the intake creating regens: the nation list (0x7ad590) sent 1 in 10 to a club with reputation ≤ 2. Since 11/06 the intake holds that path, and the biggest pile in 2003 is Prato with 12: see `TESTES.en.md` §4.5.

**All leagues, 10 years, with the fixes (11/07).** The same career (1,134 clubs, 2001 to 2011-199), with the whole quota, the Basque rule and the nation list held; no crashes and no error boxes, 36,580 s [observed, `resultados/final10anos.log`]. The day-169 intake:

| Year | Born from the queue | Created | Given | Missing | No room | Leftovers signed | Reserve left | Made Basque |
|---|---|---|---|---|---|---|---|---|
| 2002 | 1,202 | 1,077 | 2,205 | 0 | 28 | 74 | 47,937 | 9 |
| 2004 | 1,473 | 970 | 2,177 | 2 | 19 | 320 | 45,724 | 5 |
| 2006 | 2,394 | 400 | 2,328 | 2 | 1 | 643 | 44,220 | 6 |
| 2008 | 3,223 | 247 | 2,324 | 0 | 2 | 1,229 | 43,389 | 3 |
| 2011 | 3,042 | 137 | 2,311 | 0 | 1 | 929 | 42,691 | 4 |

- **Missing:** of 112 intakes, 95 with 0; the rest with 1 to 6 places, nearly all German clubs (Werder Bremen in 5 years; Dortmund and Freiburg with 2 in 2007), against 35 to 80 a year before. The logged reason is refusal 33 (0x21) of 0x8ce990 for 1 to 6 youngsters, with about 106 acceptable that day; not investigated yet.
- Average squad from 33.5 (2002) to 36.2, steady from 2005 on; maximum 50 to 52. Regens with PA 150 or more at each season end: from 27 (2002) to 1,012 (2011).
- **No sink club:** the biggest pile went from 5 (2002) to 42 (Stabæk in 2011: 19 by the quota and 16 as leftovers, over 10 years); negative-reputation clubs with 0 to 29 regens, none by the quota. In 2009 and 2010 the biggest was Atlético de Madrid B (40 and 43, none from the intake or the contest: signed later by the game itself).
- **The long stalls are the game's:** the slowest intake took 14.7 s (day 169, about 780 clubs; the others 0.02 s to 4.5 s, the `ms` field of the `intake` line). The stalls over 60 s fell at 2001-194, before the first intake, in the managers' shortlist (0x83fa5d and 0x83fb57, `shortlist_manager`), and at the 2006 and 2010 season end (day 169), in player statistics (0x7c6d40, `player_stats`) and at 0x9453d7, the game's own routines running that same day.

### 8.1 The keys

Since the final design (11/05, evening). The stock keys (`intake_stock`, `intake_pa_min`, `intake_pa_max`, `intake_pa_spread`, `intake_surprise`, `intake_squad_max`) and `intake_only` are gone; a 1.2 cfg with `intake_stock` > 0 and no `intake` line turns the intake on.

| Key | Default | What it does |
|---|---|---|
| `intake` | 0 (off) | 1: the intake on (new career only) |
| `intake_per_club` | 3 | regens per club per intake, limited to the squad room (50) |
| `intake_day` | `season_end` | each nation at the end of its own season (the 35 of the key-nation table 0xb63d60; the others on day 181), or one day of the year for everyone (0 to 365; 181 = 30/06 or 01/07) |
| `intake_clubs` | `playable` | `playable`: clubs of the playable leagues; `selected`: the background leagues too; the nations left out follow the original path |
| `intake_news` | 1 | the inbox message for the human manager's club, or the no-room notice |
| `intake_news_numbers` | 0 | 1: the message also shows CA and PA |
| `retire_day` | `original` | the day of the year (0 to 365) for every player retirement, or `original` (the game's dates) |

With the intake on, the Regens tab's `season_cap` and `squad_limit` don't apply: the quota is the limit.

### 8.2 How it works

1. **Create:** a hook on the database trim's `ftol` (0x525f8c) asks the game itself for K more people, within the headroom (people, records and preferences), and fixes the buggy cap (0x525f9b).
2. **Put to sleep:** before `game_run_to_start_date` (called at 0x829fd9), the K people are marked retired (bits 0x30 = 0x20 of the 79-byte record), with no club and outside the regen queue.
3. **Release:** on `intake_day`, after the day's queue (called at 0x5cda7a), each club gets up to `intake_per_club` dormant people (a club with 50 in the squad is skipped):
   - the record becomes a copy of a real player from the club's nation, with the intake PA;
   - it goes to the front of the record pool, and `regenerate_as_youth_player` 0x7abac0 makes the regen;
   - the clubs' contest (0x7acea2) is forced to the intake club; if the regen still has no club, the signing 0x7a6f10 runs.
4. **Save:** nothing to do. A dormant person is a retired person outside the queue, a state the save already keeps.
5. **Intake only** (`intake_only = 1`, 11/05): `process_regen_queue` 0x7a65d0 has two halves: the birth loop (0x7a65f0..0x7a66b7, the queue at +0x10/+0x14 and the record pool at +0x3c/+0x40) and the non-player refill (+0x1c/+0x20, from 0x7a66bd on), which doesn't touch the queue. The DLL calls the routine with the queue count set to zero (and puts it back afterwards): the loop doesn't run and the other half goes on. In the intake, retired players waiting in the queue whose record is still in the pool join the stock, ahead of the created ones; each leaves the queue (as 0x7a65d0 removes the head: the ones behind move up, the last slot gets -1) and his record leaves the pool, so no other regen inherits the same record later. So the intake renews itself with retirements. Non-players in the queue stay retired there. With this, the clubs' contest (`rule = new`, `season_cap`...) and `regen_origins` no longer come in: the intake already picks the club, and `regen_origins` sits on the call inside the queue (0x7a664f). Ages (`ages`) and names (`regen_names`) still apply, since they live inside 0x7abac0.
6. **The message** (`intake_news = 1`, 11/05): a human-managed club gets a news item of the game's own with the intake youngsters and the youth coach's view of each. The path is the same the CLZ NickDLL uses for its messages: build a `NewsItem` (0x77bbe0), deliver it through `News` (`send_to_club_manager` 0x7861e0, which only delivers to a human manager) and write the text when it's shown. It uses type 6009 (board finances) with a code the game doesn't know, and without the DLL the original writer would show uninitialised stack memory. Here the type is 0x1388, the manager-to-manager message of network games: its text lives in the item's own slots (from slot 3), so **without the DLL the item shows a short line** stored in the language of the day ("Youth intake: 3 youngsters joined from the youth team."). With the DLL, `NewsItem::write_text` 0x77bc80 is hooked at its entry and, for the item with the marks (slot 2 "FORN", slot 20 "INTK"), writes the title and body in the game's language at display time (`langlib`, [0x9eb2dc], codes at 0x9eb244: POR and POR_BRA → Portuguese, ESP and ESP_MEX → Spanish, the rest English), with clickable names (link 0x7d8, the person link). The manager-to-manager message comes with a "Respond" button; `NewsItem::get_action_button` 0x782b70 is also hooked at its entry and returns "no button" for our item. Seen in the game (11/05): it shows under Messages, with no button, and after switching the language in the options the same item comes out in English. Slots 22.. keep, for up to 14 youngsters, the id and the view (CA, PA and both bands), worked out on the day: the save carries it all in news.dat.
   - **Current level:** CA against the average of the squad's best 11: up to 5 below, "is already ready for the first team"; up to 20, "could get chances soon"; up to 40, "needs a year or two in the youth team"; below that, "is still very raw".
   - **Potential:** by PA: 170+, "could become one of the best in the world"; 150+, "has the potential to play for his country"; 130+, "could be a regular at a big club"; 100+, "should make a career in the top flight"; below, "is unlikely to go beyond the lower divisions".

### 8.3 The results (automatic holiday career, 278 clubs, 2001 → 2006-200)

Stock of 3,000, 3 per club, both careers in parallel, under the same load [observed]:

| | Control (no intake) | Intake |
|---|---|---|
| Time to 2006-200 | 3,523 s | 3,423 s |
| Players (season 2002 → 2006) | 19,346 → 19,293 | 22,255 → 22,191 |
| Average squad (2003 / 2005 / 2006) | 33.9 / 33.3 / 31.8 | 38.4 / 40.4 / 37.5 |
| Cumulative regens to 2006 | 3,554 | 6,730 |
| Regens with PA ≥ 150 (2006) | 197 | 318 |
| Crashes, error boxes | none | none |

- **Each year's intake:** ~740 regens across the 278 clubs (40 to 50 full clubs, 8 to 13 regens left without the club). The 3,000 stock ran out at the 4th intake (2005), as expected.
- **The dormant people stayed inert:** a year before the 1st intake and four years before the last one. None entered the queue or a club ahead of time.
- **The intake regens:**
  - 15 to 20 years old (mode 17);
  - median PA 102, median CA 54;
  - positions: M 37%, D 31%, A 23%, GK 8%.
- **No measurable time cost:** 3,000 more people didn't change the career's time. A first comparison showed the intake 75% slower, but the control had run later, on an idle machine; in parallel, the time is the same.
- **A stock of 20,000** (two careers to 2006, 11/05):
  - created in full (headroom of 120,400 people) and put to sleep;
  - 5 intakes of ~750, no crashes, and 16,200 left;
  - 39,300 players against the control's 19,300.
- **Bench time is noise:** with the other sessions' batches running alongside, the same setup ranged from 2,692 to 3,523 s (control) and from 3,043 to 4,484 s (20,000). No cost stands out from that noise.
- **Taking the record away from the dormant person until the release** (`+0x61 = 0`, an experiment to keep it out of the free-player scans) gained no time and was dropped: it would mean rebuilding the person ↔ record pair on every load.
- **Fixed:** in the first version the spread went past `intake_pa_max` (up to 185 with the maximum at 170); now the maximum is the cap.
- **The two 10-year careers with the curve** (11/05, 2001 → 2011-185, the repository's `tweaks.cfg` with a 20,000 stock, intake on day 181 and retirements on 174; no crashes, save written on the last intake day and loaded again): A = the guarantee from the stock only (7,019 s); B = the guarantee first from the game's clubless regens (up to 21, from the club's nation), then the stock (5,737 s). In 2011: stock used over 10 intakes 6,490 (A) and 2,946 (B); clubs with 3 or more 264 and 278 of 278; average squad 40.3 (rising) and 37.2 (steady at 36.4 to 38.0 since 2004); clubless regens in the world 1,168 and 463; regens with PA ≥ 150 1,373 and 1,081; the biggest pile of regens in one squad (the "dump club") 44 (Bangu) and 39 (Corinthians de Caicó). B stays. Open in both: the dump club grows year after year (5 to 39 in B), in small Brazilian clubs (solved on 11/06: the nation list, see `TESTES.en.md` §4.5).
- **`intake_squad_max = 40`** (11/05): average squad ~37 instead of 40; 140 to 160 clubs skipped per year, ~400 regens given per intake. The key does what it says.
- **`intake_clubs = selected` with a background league** (ARG, to 2004): the intake reached 323 clubs (the 278 playable ones plus Argentina's 45), ~850 per year, no crashes; the 3,000 stock ran out at the 3rd intake. Background clubs receive like the others.
- **Long career with 20,000** (2001 → 2011-201, 8,489 s, 11/05): no crashes, 10 intakes of 730 to 800, 12,400 left. With `intake_squad_max = 50` the average squad grows every year (41.2 in 2006, 44.7 in 2011), and regens with PA ≥ 150 go from 346 to 1,008: hence the suggested default of 40. Sleeping people count as free in the statistics (14,000 to 17,000 "free").
- **`intake_day = season_end`** (11/05, to 2003): Brazil receives on day 10 (112 clubs, ~335 regens) and England and Italy on day 169 (166 clubs, ~330 to 430), each at the end of its own season. The first version gave an extra intake on the career's first day: during setup the loop goes through every nation; the DLL now makes the same date check as the start of 0x59cdd0 (0x456770) and ignores those passes.
- **`intake_only = 1`** (11/05, to 2004-216, 2,322 s, no crashes): no births and no contest through the seasons (`births 0`, `contest 0` in every line); three intakes of ~750 (278 clubs), 767, 677 and 516 of them from the queue of retired players, and the created stock only fell from 3,000 to ~2,650. Players from 22,287 to 22,105 in three years (−0.8%); average squad 35.3 → 37.8; 16 to 21 youngsters per intake end up without the club (they become free agents). The log's "stock left" also counts the retired players left in the queue (hence above 3,000 in the first one).

### 8.4 To do

- ~~**Inbox message**~~: done and seen in the game (11/05), step 6 above. The request: on intake day, each human-managed club gets a news item with the youngsters who arrived (name, age, position) their current level (CA) and potential (PA), both **in bands**, like a youth coach's assessment ("can already train with the first team", "could become an international player"); the CA band compared with the club's own squad (the same youngster may be ready at a small club and far from it at a big one); the exact numbers only with a key. Through the exe's own news system (`g_news` 0xb7431c, the text in `NewsItem::write_text` 0x77bc80), without NickDLL. The text comes from the DLL: English, Portuguese and Spanish built in, by the game's language (English when there's no translation), and a phrase file next to the DLL for other languages.
- Measure time on an idle machine (the long career reached 2011 with no crashes).
- Save and load with Lucas: the save must open with and without the DLL; check the editors.
- ~~The intake date per nation~~: done (11/05), `intake_day = season_end`. Each key nation's season end goes through `end_season_for_nation` 0x59cdd0 (called at 0x5d5ea2, in the loop over the 35 key nations); the DLL notes the nation and releases it in the day's queue.
- ~~The configurator tab~~: done (11/05), tab **Intake (experimental)** under Career tweaks, with the 8 keys and help in both languages.
- Positions: regens come out with the positions of the copied real player (G 8%, D 31%, M 37%, A 23%), the nation's own picture; no adjustment for now.
- ~~Full squads~~: `intake_squad_max` left in the final design; the limit is the quota up to 50 places (average squad 35 to 38 in the 11/05 evening career).
