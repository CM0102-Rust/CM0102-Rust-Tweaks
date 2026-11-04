# The regen intake: the study (11/04/2026)

*[Português](FORNADA_DE_REGENS.md)*

A read-only study (disassembly, pseudocode and saves; no code changed, the game wasn't run), asked for by Lucas before any code. The question: can we redesign the regen cycle, with every retirement on one day, every regen on one promotion day, a guaranteed number of regens per club and player records created by the DLL? It complements [DISTRIBUICAO_DOS_REGENS.en.md](DISTRIBUICAO_DOS_REGENS.en.md) and phase 6 of the [ROADMAP](../ROADMAP.md).

Tags: **[observed]** in the code or the saves; **[inference]** from the flow, to be proved on the test bench.

## 1. The short answer

| Piece | Possible? | How |
|---|---|---|
| Every retirement on one day | **yes, simple** | change the date at the single entry point for retirements (0x7a8510) |
| Every regen on a promotion day | **yes, simple** | hold the queue (0x5cda7a) on the other days |
| N guaranteed regens per club | **yes, medium** | the club comes from the quota, in the dispute hook Tweaks already has (0x7ad0e0) |
| New player records (more regens than retirements) | **possible, but of little use** | extra rows in the player table at load, plus the two parallel tables; they only become regens while there are idle slots (§5) |
| More regens per season | **yes** | more retirements, drawing on the ~13,000 without a club: slot and record together, with the same population (§5) |
| New people (slots) | **no** | the 16 human managers are the last people in the table; and in a long career the slot is the limit (§5) |

## 2. Slot and record

Every player regen needs two pieces:

- The **slot** is the retired **person** (name, birth date, id, personality). The regen becomes that person, with a new name and age.
- The **record** is the **player record** (`player.dat`, 70 bytes): PA, positions, feet and attributes. It decides whether the regen is any good.

Early in a career **slots are plentiful and records are scarce**: in `TesteCMRTE` (Dec 2001), 845 non-player slots already due were waiting for a record, with the pool empty [observed]. **That doesn't last:** in the all-leagues save in 2063 there are no idle slots; the queue only has future-dated slots, almost all of them players (§5).

## 3. The cycle today

Everything lives in an object at `[0xb7434c]`, with four tables (pointer, count, capacity). They grow by `realloc` in steps of 250, with no limit, and are saved in the save's `retire.dat` item (0x7ad7c0 writes, 0x7adcd0 reads) [observed]:

| Table | Record | What it is |
|---|---|---|
| +4 | 26 B | scheduled retirements: id, club, announcement date, retirement date, reason, announced |
| +0x10 | 12 B | the slot queue: id, date |
| +0x1c | 16 B | the non-player record pool |
| +0x3c | 16 B | the player record pool: record id, nation, second nation, owner |

The daily loop (0x5cda73..0x5cdaa9) calls, in this order [observed]:

1. `find_new_retirements` 0x7a5790, only on days 0, 80, 173 and 250: schedules who will retire. The date is today (unemployed) or the end of the nation's season (age).
2. `process_retirements` 0x7a59e0, every day: on the date, the record goes to the pool and the slot to the queue (today, or today + 1 to 65 days for those aged 30 or more). Non-players either go to the queue (no club, manager, human club, or 1 in 4) or become a new non-player in the same slot (0x7ab340).
3. `process_regen_queue` 0x7a65d0, every day: while there is a slot due and a record in the pool, `regenerate_as_youth_player` 0x7abac0 joins the two. Then the nation's clubs compete for the youngster (0x7ad0e0) and `player_regen_sign_for_club` 0x7a6f10 signs him.

Numbers from the saves: 2,481 to 7,780 scheduled retirements, about 2,000 to 4,000 retirements per season [observed in the saves; the per-season rate is inference].

**A side finding:** the "will retire at the end of the season" notice almost never shows up in the original. The sort order of the list leaves behind anyone already past the announcement date, and the loop stops before reaching them. The saves had 0 announced out of 7,501 [observed + inference].

## 4. The pieces of the redesign

### 4.1 Retirements on a fixed day

- **Where:** `add_retirement` 0x7a8510 is the single entry point for all four sources. The DLL replaces the retirement date there with the chosen day, and goes through the list once when a save loads (for the ones already scheduled). The daily routine re-sorts the list by itself [observed].
- **Detail:** the comparison is a strict "before today", so to retire on 31/07, write 30/07 [observed].
- **Until that date, the person plays on as normal** [observed].
- **Cancellations:** on the date, the game drops anyone on loan, with an open offer, or who changed clubs less than 209 days ago [observed]. The longer the wait, the more drop out, and a fixed day inside the transfer window makes it worse [inference]. **Suggestion:** make the fixed day the end of each nation's season (the original already uses that date for age), not one calendar date for everyone.
- **Cost:** thousands in one day. The `regen_origins.csv` from the 1.1 test folder shows 262 regens in the same second, so the heavy day should take a few seconds [inference].

### 4.2 Regens only on the promotion day

- **Where:** the call to `process_regen_queue` at 0x5cda7a. On other days, the DLL zeroes the record pool count (+0x40) around the original call, so the queue doesn't move; the coaching-record distribution still runs [inference from the code].
- **The queue date:** write today in the two calls to `queue_staff_for_regen` 0x7a87d0 (0x7a64c9 and 0x7a64e2), which Tweaks already intercepts.
- **The shared record** (the retiree in the queue still points to the record) only grows in number. The routines read skip the retiree (0x7a7eb0) [observed]. The remaining risk is editors and routines not yet read [inference].
- **Careful:** if the slot has a club when its turn comes, 0x7abac0 drops the slot without using the record [observed].

### 4.3 N regens per club

- **Where:** the dispute, 0x7ad0e0, which Tweaks already hooks (with the nops at 0x7ace6a/0x7ace89). The DLL returns the quota's club. The signing (0x7a6f10), the promoted-youngster news (0x7a8ba0, human club only) and the rest come from the game itself [inference].
- **Full squad:** 0x7a6f10 looks for a free spot among the squad's 50 (+0xd7) and returns 0 if there is none. The DLL checks first and moves on to the next club in the quota [observed; the rule is a proposal].
- **Contract:** ends in +2 years, or +1 to 6 for those over 18. AI clubs get at most 5 years and a shirt number; at the human club the youngster goes to the reserves (0x4cc0f0) [observed].
- **The limit:** the supply is the nation's records. Without new records, "3 per club" only works out in nations that free enough records.

### 4.4 New records

#### Why not people

- The 16 human managers are the last counted people: about 100 places test `id >= count − 16` and about 60 compute `count − id − 1` [observed].
- The 79-byte per-person table (0xae2d0c) is created at its exact size (0x82849a), and Staff_Records follows the non-player count [observed].
- The 23,000 reserve people only exist while the career is being created (used up by 0x527240 and the database trim). None are left in the saves [observed].

The person table can't grow without changing those places. That limits new records (§5).

#### Why records, yes

- **Count:** `[0xae236c]`, table `[0xae23c4]`, referenced by absolute address in `person+0x61`. No routine changes the count during a career; the save writes the in-memory count (0x531044) [observed].
- **The allocation** is a `malloc` of the player table alone (0x529e0c at load) or a `realloc` in the database trim (0x526010, new game). The DLL can add K rows there beforehand without moving any pointer [observed; the hook is a proposal].
- **The 1,325 rows after the count belong to the grey players:** `get_id_block` 0x90e760 hands out blocks of 25 starting at the count. Raising the count while grey players are live (a match, a club opened by the human) overwrites their records [observed]. So the DLL's reserve goes **after** the 1,325, or the count only rises when no grey players exist.
- **Two parallel per-record tables** must grow along with it:
  - PlayerStats `[0xb74350]`: 39 B per record at +0x30, with its own count at +0x34. 0x7c6650, called by 0x7abac0, writes there without a bounds check [observed].
  - Training `[0xde1f44]`: 6 B per record at +0x651d. Loading `training.dat` requires exactly `[0xae236c]` entries (0x8b11c2..0x8b11d2), or it fails [observed].
- **Load order:** raise the count only after `training.dat` and PlayerStats are read [the limit is observed; the order is a proposal].
- **Sweep:** only 43 references to the player table in the exe; the others (diary, match, virtual_staff) are dynamic [observed]. The risk is a separate count outside those.

#### A new record

The game already builds records from a template: `create_virtual_player` 0x90e0a0 copies the 70 bytes of a real player and changes only the id, the number, the CA and the reputations. That proves a raw copy is accepted [observed]. The record has no pointers, and its links (owner, PlayerStats, training) all go by row index [observed].

Minimal recipe:

1. A counted row `i`, with no owner and not in the pool.
2. Copy the 70 bytes of a living template player of the same nation and position.
3. Set `+0 = i`, `+4 = 0` (number), `+0x45 = 10` (morale) and `+7 = PA` drawn at random (1..200).
4. Initialise the training entry (as at 0x8b3c50); 0x7abac0 zeroes PlayerStats.
5. Add `{i, nation, 0, −1}` to the pool (+0x3c), growing it with the game's `realloc` (0x945667).

Consistency rules: absolute attributes from 1 to 20, one position = 20, one side = 20 (except keepers), one foot = 20. The database's PA codes (−1, −2, −9) only apply at creation. In memory, the 18 technical attributes are stored relative to the CA (`(10 × rating − CA) / 2`), so a template taken from the database file needs converting [observed].

### 4.5 What the record's PA gives

0x7abac0 draws the CA as a fraction of the PA, by position, for a nation in the game or with 350 people or more [observed]:

| Position | CA |
|---|---|
| striker | 0.3·PA + random(0.6·PA) |
| centre-back | 0.25·PA + random(0.55·PA) |
| goalkeeper | 0.25·PA + random(0.5·PA) |
| others | 0.25·PA + random(0.6·PA) |

There's also a cap by age: 80, 90, 100 and 110 between 15 and 20, and about 75% of regens are under 19. In practice, PA 100 to 130 gives an average CA of 55 to 70, and PA 180 to 200 tops out at 80 to 110 [inference from the formulas]. Nations outside the game with fewer than 350 people use other formulas and ages of 17 to 27.

## 5. The numbers (correction of 11/04, Lucas's question)

The first version of this study said "the queue already has slots to spare", so new records would be enough. That was a reading of early-career saves. The numbers from the all-leagues save (`All Leagues - 3.9.68 - Compress.sav`, 2001 to 2063, 62 seasons) show something else [observed, `conta.py`, `hoje.py` and `np.py`, read-only]:

| | All leagues, 2063 |
|---|---|
| Clubs in the database | 10,892 |
| Clubs in a nation with a selected league | 7,283 (3,889 in a playable league) |
| Clubs with anyone in the squad | 6,752 (13 in the squad on average; 9 full) |
| People (fixed table) | 158,790: 114,708 with a player record, 44,801 with a non-player record |
| Living player regens per birth year (2036 to 2043) | 6,624 to 7,833 per year |
| Queue and pool in the save | 1,504 slots, all future-dated (1,428 of them players), and 1,913 records waiting for those dates |
| Without a club | 9,303 player regens and 4,054 non-players |

**What the numbers say:**

- **The natural rate is about 7,500 player regens per season** with all leagues, one per player retirement.
- **3 per club** means 11,667 per season for the 3,889 clubs in playable leagues alone, and 20,256 for the 6,752 with a squad. That's 4,200 to 12,800 short per season.
- **In a long career the slot is the limit, not the record.** Every regen needs a person, and the person table is fixed. The 845 to 1,260 idle slots in the early saves are a stock that runs out (the database's non-players retiring early). In 2063 there are no idle slots.
- **So new records alone don't give extra regens for long:** without new people, they'd only use up the initial slot stock and then sit waiting in the pool.
- **With a fixed population, extra regens per season can only come from extra retirements.** Today about 7,500 of ~115,000 players are renewed per season (an average career of about 15 seasons). For 11,700 per season, the average career would drop to about 10.
- **The pool for that already exists in the world:** 9,303 player regens and 4,054 non-players without a club. Retiring the idle ones earlier frees a slot and a record together, with no new table; it's `retire_idle_players`, at a higher dose.

### What's left for growing the population

New people would mean changing the ~160 places that treat the last 16 people as human managers (or moving the humans), the 79-byte table (0x82849a) and Staff_Records. That's out of reach for now; the viable path is renewing the existing population faster.

### The effect of the fixed day and of new records

- **With the fixed day alone** (4.1 to 4.3), nobody is added. The number of regens stays the same, just bunched together.
- **With new records** (4.4), extra regens only while there are idle slots (early in the career).
- **With extra retirements**, extra regens at any stage, with the same population.

- **With the fixed day alone** (4.1 to 4.3), nobody is added. The number of regens stays the same, just bunched together, and the first season has no extra retirements, only grouped ones.
- **With new records** (4.4), each record is one more player, for good. 3 per club across 300 clubs = 900 per season. More players means more retirements years later, which slows the growth; the balance has to be measured in long careers.
- **More regens without new records:** retire the players nobody wants earlier (`retire_idle_players`, already in Tweaks).

## 6. Risks and what still needs checking

- Live grey players at the moment the count goes up.
- Forgetting PlayerStats or training, or raising the count before `training.dat` loads (load error).
- A separate count outside the 43 references.
- What the 6 training bytes per record mean.
- Orphan records (counted, no owner, not in the pool): the `Teste.sav` from the 1.1 test folder has 130, while `TesteTelas` and `Lucas.sav` have none [observed]. If the game already lives with orphans, a record that was created but not yet handed out is a tolerated state [inference]. Where the 130 came from is still unknown.
- Editors (CMRTE, CM3SaveEditor) with larger counts, and with the shared record lasting longer.
- The NickPatcher and CLZ exes, which change retirements (RetirePatchV4).

## 7. Proposed order

1. **The fixed day** (4.1 and 4.2), adding nothing to the world: measures the cancellations, the cost of the day and the effect on clubs.
2. **The per-club quota** (4.3) with the regens already being born: measures how many clubs fill their quota in each nation.
3. **The retirement rate**, to reach the number of regens per season we want, drawing on those without a club.
4. **New records** (4.4) only if idle slots are left over; new people only as a separate project.

Each step gets its own key in `tweaks.cfg`, off by default, and is proved on the test bench before the next one.

## 8. Addresses

| Address | What it is |
|---|---|
| 0x5cda73..0x5cdaa9 | the daily loop: retirements, queue, new retirements |
| 0x7a5790 | `find_new_retirements` (days 0, 80, 173, 250) |
| 0x7a59e0 | `process_retirements` |
| 0x7a8510 | `add_retirement`, the single entry point |
| 0x7a87d0 | `queue_staff_for_regen` (calls at 0x7a64c9, 0x7a64e2) |
| 0x7a65d0 | `process_regen_queue` (called at 0x5cda7a; empty pool check at 0x7a6618) |
| 0x7abac0 | `regenerate_as_youth_player` |
| 0x7ab340 | `regenerate_as_non_player` |
| 0x7ad0e0 | the dispute for the youngster (hooked by Tweaks) |
| 0x7a6f10 | `player_regen_sign_for_club` |
| 0x7ad7c0 / 0x7adcd0 | writes and reads `retire.dat` |
| 0x529bfe, 0x52a04b..0x52a05a | the grey players' 1,325 rows at load |
| 0x529e0c / 0x526010 | the player table allocation: load / database trim |
| 0x8b11c2..0x8b11d2 | loading `training.dat` requires the exact count |
| 0x90e760 / 0x90e0a0 | grey player blocks / record from a template |
| `[0xae236c]`, `[0xae23c4]` | player record count and table |
| `[0xb7434c]` | the regen object |
| `[0xb74350]`, `[0xde1f44]` | PlayerStats and training |
