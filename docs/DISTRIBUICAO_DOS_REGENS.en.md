# How regens are distributed

*Versão em português: [DISTRIBUICAO_DOS_REGENS.md](DISTRIBUICAO_DOS_REGENS.md)*

How CM 01/02 3.9.68 creates player regens and picks a club for each one. Written for the champman0102.co.uk thread "[RELEASED] 4 Playable Polish Divisions" (questions from Here2Win79 and alex91). Sources: the routines already rewritten in `rust/src/motor.rs` (compared against the original with no divergence), the `cm0102.exe` disassembly and three saves. Tags: **[observed]** = read in the code or counted in a save; **[inference]** = my conclusion.

Companion to [MARCA_DO_REGEN.en.md](MARCA_DO_REGEN.en.md) (§4 and §8: the queue, the player-data pool, what a regen inherits).

## 1. Summary

- **There is no per-club "youth intake".** One player regen is born for each player record that retires, and then the clubs of his nation compete for him [observed].
- **How many per season:** the same number of players who retire (or become coaches) in the whole database. The potential (PA) is the retired player's. So the number of high-PA youngsters a nation gets each season is the number of high-PA players from that nation who retire [observed].
- **How the club is picked:** each eligible club gets a score, the game draws a number between 0 and each club's score, and the highest draw wins the youngster [observed].
- **Why big AI clubs get so many:** three terms in the score. Training 20 adds 7000 + reputation/2, every time; with Training 19, only 45% of the time. An AI club with a decent manager and high Training gets its score raised up to its own reputation. And a squad under 40 adds another 1000. Nothing limits the number of youngsters per club except the squad of 50 [observed].
- **The human club** doesn't use its manager (the human has no attributes). It gets a small flat bonus and, with Training 20, the same reputation floor, only for PA above 120 [observed].

## 2. When the game creates regens

| Step | Routine | What it does |
|---|---|---|
| Retirement search | `find_new_retirements` 0x7a5790, on days 0, 80, 173 and 250 of the year (`is_player_regen_day` 0x5d6250) | Every person goes through the age rule (`player_regen_check_retirement` 0x7a7310). Skipped: under-30s at a club in a competition that runs in the game, the 16 human managers and anyone already listed [observed] |
| Retirement | `process_retirements` 0x7a59e0, daily | A retiring player goes into the **queue** (`queue_staff_for_regen` 0x7a87d0) and his record (`player.dat`) goes into the **player-data pool**. Someone who becomes a coach only leaves his record. The queue date is today (under 30) or today + 1 to 65 days [observed] |
| Birth | `process_regen_queue` 0x7a65d0, daily | While the queue head is due **and** the pool has a record: `regenerate_as_youth_player` 0x7abac0 on the head's slot, with the pool's first record [observed] |

The pool is the bottleneck. Non-players also enter the queue, but without a record. So **player regens per season = player records freed per season** [observed; see MARCA_DO_REGEN §8].

Counts in saves [observed]:

| Save | Span | Living player regens |
|---|---|---|
| `TesteTelas.sav` (stock database, Lucas's leagues) | ~7 months (Jul 2001 to Feb 2002) | 793 |
| `Valencia.sav` (CLZ) | 2025 to 2028 | 3,857 |
| `All Leagues - 3.9.68 - Compress.sav` (all leagues) | 2001 to 2063 (62 seasons) | 127,863 of 158,790 people |

In the all-leagues save, that's at least ~2,060 regens per season still alive at the end. It's a floor, because regens retire too and become regens again [calculation].

## 3. The youngster: age, ability, nation

From `regenerate_as_youth_player` 0x7abac0 [observed]:

- **PA (+7 of the record):** unchanged. It's the retired player's (B, the record owner). Positions, feet and much of the attributes also come from him.
- **Nation:** B's. If the nation's reputation (+0x88) is below 100, there's a 1 in 4 chance of switching to another random nation, also with reputation below 100 (0x7abcd7).
- **Age** (nation in the game or reputation ≥ 350; 0x7ac58e): random(10000):
  - = 0: 15 to 18;
  - < 5000: 16 to 18;
  - < 9976: 17 to 20;
  - otherwise: 21 to 26.
  - Result: **~75% of regens are under 19**.
  - In a nation not in the game with reputation < 350, age ranges from 17 to 27.
- **CA (+5)**, by position (0x7ac3dd):

| Position | CA |
|---|---|
| striker | 0.3·PA + random(0.6·PA) |
| defender or sweeper | 0.25·PA + random(0.55·PA) |
| goalkeeper | 0.25·PA + random(0.5·PA) |
| others | 0.25·PA + random(0.6·PA) |

  Then an age cap, 249 times in 250 (or 149 in 150): 80 up to 17; 100 at 18; 90/110 in the 17-to-20 branch; none from 21 up.
- **Reputation:**
  - home (+9) = 250 + random(750) + 20·CA;
  - world (+0xd) = 1 + random(20) + 5·CA;
  - current (+0xb) = home + random(50).

## 4. Which path to a club

End of 0x7abac0, 0x7ace65..0x7acf5c [observed]:

| Situation | Path |
|---|---|
| Under 19, nation in the game (+0x11c), 3 times in 4 | **The national contest**: `pick_best_club_for_player` 0x7ad0e0 |
| Nation in the game, all other cases | 1 time in 10, the nation list (0x7ad590); 9 times in 10, **no club** |
| Nation not in the game | 14 in 15, the nation list; otherwise 1 in 10 |

- The nation list (`build_nation_club_lists` 0x7acfc0) only takes a club from a nation in the game if its reputation is ≤ 2 (0x7ad075). In practice **it's empty for playable nations**, and a youngster on that path also ends up clubless [observed; that no club has reputation ≤ 2 is an inference from the usual databases].
- In playable nations, ~56% of regens (0.75 × 0.75) go through the contest; the rest start without a club and rely on the market [calculation].
- Clubless and under 20: current and home reputation are capped at 1250, and world reputation is set to 1 (0x7acf6b).
- Saves [observed]:
  - `TesteTelas`: 280 of 793 clubless;
  - `Valencia`: 214 of 3,857 (the rest signed later).

## 5. The national contest: `pick_best_club_for_player` 0x7ad0e0

Every club in the table (national teams excluded) takes part, even from leagues not in the game, if [observed]:

1. the youngster is **not foreign** there (0x53d980 with groups: his nation, his second nation, and the British Isles and Ireland group, Australia and [0x9cf3f4], and others). In practice: **only clubs from his nation**;
2. it is not the B team of a pair (0x540a50);
3. it is not barred by the Basque rule;
4. it can approach the player (`can_club_bid_for_player` 0x8ce990);
5. it has **fewer than 50 in the squad** (`database_count_club_squad` 0x53c590; the `cmp bl, 0x32` at **0x7ad1bd**).

Each gets `random(score)` (0x7ad220), a number from 0 to score − 1. **The highest wins** (starting at −25000; a tie keeps the first). No eligible club: the youngster stays clubless.

## 6. The club score: 0x7ad220

Notation: R = club reputation (+0x80, 0..10000). Tr = Training (+0x7f, 1..20; what players call "facilities"). PA, CA = the youngster's. rep = the youngster's current reputation (+0xb).

**1. Base** [observed]:

    score = 5000 + R − |rep − R| − |50·PA − R| − |50·CA − R|

- It's a broken line in R. It rises 4 per point below the lowest of the three points (rep, 50·CA, 50·PA). It rises 2 between the lowest and the middle one. It's **flat between the middle and the highest**. It **falls 2 per point above the highest**.
- The top is worth 5000 + lowest + middle − highest [calculation].
- Example, PA 170, CA 80, rep 2250 (points 2250, 4000, 8500): a top of 2750 for R between 4000 and 8500; R = 9500 gives 750.
- On its own, the base **prefers clubs whose reputation matches the youngster's level** and punishes clubs that are too big.

**2. Training** [observed]. Applies if PA > random(20) + 100: always with PA ≥ 120, never with PA ≤ 100.

| Tr | Chance | Bonus |
|---|---|---|
| 20 | always | R/2 + 7000 |
| 1 to 19 | random(Tr) > random(20), i.e. (Tr − 1)/40: 45% at 19, 22.5% at 10, 0 at 1 | R/2 + 5000 + 50·Tr |

The step from 19 to 20 is huge [calculation].

**3. Active competition and PA > 50** [observed]. The club plays in a competition that runs in the game (`[[0xadadfc] + 4·id]`).
- Human club only, PA > 120: +500 or +250 from byte +4 of the club's season state (`[0xae2d14]` + 6·id), negative or zero [meaning of the byte not confirmed].
- Every club, by squad size:

| Reputation | Squad < 35 | < 40 | < 45 | rest |
|---|---|---|---|---|
| R > 5750 | +1000 | +1000 | +750 | +250 |
| 4250 < R ≤ 5750 | +750 | +500 | +250 | +250 |
| R ≤ 4250 | +250 | +250 | +250 | +250 |

**4. The manager** (+0xcf → non-player record +0x69). Not the youth coach [observed].
- **AI club:**
  - + (Youngsters − 10) × 30 (Youngsters = +0x22);
  - with PA > random(75) + 100: + (v − 10) × 60, where v = f(manager CA + 2·JudgingPotential) is `chance_to_know` 0x544410: x = (CA + 2·JP)·0.05 + 10, v = (x + 0.1x²)/3 + 0.5, from 1 to 20;
  - **the floor:** if v + Youngsters + Tr > 40, R > 4250 and PA > 80, the score is raised to R (if already above R, +250).
- **Human club:** +200. With PA > 120: +25·Tr, and with Tr = 20 the same floor at R.

**5. Nation** [observed]:
- same nation as the club: +1000;
- Welsh clubs ([0x9d0430], [0x9d0434], [0x9d043c]): Welsh youngster +1000, English −750;
- Scottish club: Welsh or English youngster −750, Scottish +1000.

**6. 16 bits.** Only the low word of the score goes into the draw. With R ≤ 10000, all terms together stay under ~31,700 (base ≤ 15,000, Training ≤ 12,000, competition ≤ 1,500, manager ≤ 1,150, nation ≤ 2,000). So **the 16-bit cut doesn't flip the choice** in normal databases. Only with reputations above the scale (which the editor doesn't allow) would the score pass 32,767 and turn negative, and that club would never win [calculation].

## 7. Why big AI clubs get so many

A simulation of the formula above ([inference]: made-up clubs, 20,000 draws; script in `regens/tools/simula_nota.py`):
- big AI club: R 9000, Tr 20, good manager;
- mid AI club: R 6500, Tr 14, average manager;
- human: R 6500, Tr 20;
- second-tier AI club: R 4000, Tr 10;
- 40 small clubs: R 1500, Tr 3.

| Youngster | Big AI (9000) | Human (6500) | Mid AI (6500) | Second tier (4000) | 40 small |
|---|---|---|---|---|---|
| PA 170, CA 80 | 44% | 42% | 9% | 4% | 1% |
| PA 130, CA 65 | 27% | 51% | 11% | 5% | 5% |
| PA 90, CA 45 | 59% | 0% | 0% | 15% | 26% |
| PA 170, no human club | 68% | n/a | 18% | 10% | 3% |

What explains it [observed in the formula]:
1. **The AI floor.** With a reasonable manager and high Training, the big club's score is at least its reputation (9000 in the example). That wipes out the base penalty for a club that's "too big", and it applies **to any PA above 80**. That's why the big AI club also takes the average youngsters (the PA 90 row), which the human and the mid club can't reach.
2. **Training 20** adds 11,500 to the R 9000 club, every time. With 19, only 45% of the time.
3. **No per-club limit.** The score is recalculated for every youngster, and the same club wins again until its squad reaches 50. The only brake is the squad bonus, which drops from +1000 to +250 from 45 up.

In saves [observed; regens at their **current** club, which may no longer be the one they were created at; reputation and Training go together]:

| Club reputation | `TesteTelas` (7 months) | `Valencia` (3 seasons) | all leagues (62 seasons), PA ≥ 150 per club |
|---|---|---|---|
| < 1000 | 0.00 | 0.11 | 0.007 |
| 2000–2999 | 0.11 | 0.75 | 0.11 |
| 4000–4999 | 0.16 | 1.69 | 1.15 |
| 6000–6999 | 0.22 | 3.26 | 3.29 |
| 8000–8999 | **1.68** | 3.75 | 6.19 |
| 9000+ | 0.11 | 3.80 | 5.54 |

Regens per club by Training, in `Valencia`: Tr 1 = 0.09, Tr 10 = 0.96, Tr 16 = 2.62, **Tr 19 = 2.40, Tr 20 = 5.81**. The jump from 19 to 20 is the step of the guaranteed bonus [observed in the save; the cause is inferred from the formula].

In `TesteTelas`, Palmeiras, São Paulo, Vasco (50 in the squad), Santos (46) and Cruzeiro (48) got 5 to 8 regens each in 7 months. Those are the full squads alex91 describes. The 9000+ clubs got almost none: they're European, from nations that had freed few records that early [inference].

## 8. The human club

Differences [observed]:
- The human manager isn't in the score. Instead: +200; with PA > 120, +25·Tr (up to +500) and the reputation floor **only with Tr = 20**.
- With PA > 120 and an active competition: +250/+500 from byte +4 of the club state.
- Everything else is the same as the AI (base, Training, squad, nation).
- On signing: the reserves (0x4cc0f0), contract bits +0x4f/+0x35 and the youth promotion news item (0x7a8ba0). At an AI club, the contract is capped at today + 5 years.

For anyone expecting "10 youngsters a year with facilities 20": **the club doesn't generate youngsters**. The expected number is the sum, over the nation's youngsters who enter the contest, of the chance of winning each one. That chance depends on the other clubs in the nation, and with Training 20 a mid-reputation human club is competitive for PA > 120. For PA ≤ 120 the human club has no floor, and the big AI club with a good manager takes them [inference from the numbers in §7].

## 9. The xeno / Saturn "Regen Tweak"

- **0x7ad1bd** `cmp bl, 0x32` → `0x2d`: in the national contest (§5), only clubs with fewer than **45** in the squad take part (the original is 50). It touches the only brake there is. **Credit goes to xeno and Saturn.**
- **0x7a6f40** `cmp eax, 0x32`, **0x7a6f45** `cmp eax, 0x32` and **0x7a6f48** `jne` are in `player_regen_sign_for_club` 0x7a6f10: the search for the first free slot among the squad's 50 (+0xd7). Saturn's full version puts 45 in both `cmp`s and swaps the `jne` for `jl`: it only signs if there's a free slot among the first 45. petermf23's patch put those three back to the original and kept only 0x7ad1bd [observed in the bytes].
- Practical effect of the difference [inference]:
  - The contest already drops clubs with 45 or more. The lock at signing only matters where the count and the slots differ: an A/B pair (count 0x53c590 adds the partner), or slots held by people who aren't counted.
  - And on the other path, the nation list (0x7ad590), which has its own limit of 50 at **0x7ad6aa** (`cmp al, 0x32`) and only works for nations not in the game. With the full tweak those clubs stop at 45; with petermf23's, they go up to 50.

## 10. Addresses

| Address | Name (community map) | Role |
|---|---|---|
| 0x5d6250 | `is_player_regen_day` | days 0, 80, 173, 250 |
| 0x7a5790 | `find_new_retirements` | retirement search |
| 0x7a7310 | `player_regen_check_retirement` | age rule |
| 0x7a59e0 | `process_retirements` | queue and player-data pool |
| 0x7a87d0 | `queue_staff_for_regen` | joins the queue |
| 0x7a65d0 | `process_regen_queue` | daily birth |
| 0x7abac0 | `regenerate_as_youth_player` | age, CA, reputation, path (0x7ace65..0x7acf5c) |
| 0x7ad0e0 | `pick_best_club_for_player` | the national contest; squad < 50 at 0x7ad1bd |
| 0x7ad220 | (club score) | the §6 formula |
| 0x7ad590 | `pick_club_from_nation_list` | nation list; squad < 50 at 0x7ad6aa |
| 0x7acfc0 | `build_nation_club_lists` | reputation ≤ 2 in playable nations (0x7ad075) |
| 0x7a6f10 | `player_regen_sign_for_club` | free slot (0x7a6f40..0x7a6f48), contract |
| 0x544410 | `chance_to_know` | manager's v |
| 0x53d980 | `database_is_staff_foreign` | nation filter |
| 0x90cfc0 | `aleatorio` | random(n) = 0..n−1 |

Club fields: +0x7f Training, +0x80 Reputation, +0xcf Manager, +0xd7 Squad[50]. Manager record: +4 CA, +0x18 JudgingPotential, +0x22 Youngsters. Player record: +5 CA, +7 PA, +9/+0xb/+0xd reputations.
