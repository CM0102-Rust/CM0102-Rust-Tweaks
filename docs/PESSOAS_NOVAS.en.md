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
