# Reputation and attendance (study)

*Versão em português: [REPUTACAO_E_PUBLICO.md](REPUTACAO_E_PUBLICO.md)*

Study of 07/11/2026. Lucas's request:
- small clubs' **attendance** is too low and does not grow with success: it doesn't even fill their own stadium;
- a **player** who stands out doesn't gain enough reputation to become wanted by the market, and a successful **club** also takes too long to become attractive.

Method, the same as for the regens: map it in the code (every routine below is already rewritten in Rust in the CM0102 project and compared with the original), measure it in an automatic career with every league in the game, and propose keys whose default equals the original. The market itself is in [MERCADO.en.md](MERCADO.en.md).

**[observed]** = read in the code; **[measured]** = automatic career; **[inference]** = conclusion still to prove.

## 1. Attendance

**In each match** (0x85cba0) [observed]: the home attendance is the **club's average** (+0x73) × a factor (the competition's reputation over the league's, the table position, derby, the away club's distance), and stays **between the club's minimum (+0x77) and record (+0x7b)**. Stadium capacity only limits it afterwards.

**At the end of each league's season** (0x5d6280) [observed]: the average moves by the result (champion, runner-up, promotion, mid-table, relegation), with small additions and caps of 2,000, 3,000 or 5,000 × the league's reputation, and a weight per nation (0.1 in Wales, both Irelands and Australia; 0.25 in the Nordic countries and the USA; 0.5 in Scotland). At the end it is again kept between the minimum and the record.

**The record (+0x7b) only rises with a title or a promotion** [observed: its only three writes]: +20 × the league's reputation for a title, +10 × for a promotion (only if the record is within 500 of the average) and the floor for promoted clubs. **Nothing raises it for a full stadium, a good season or reputation.** A club that has a great season without winning anything hits the record with its average and stays stuck there.

**The stadium** (0x59cfa0, season end) [observed]: it grows if the real average passes 92% of capacity, which the record almost always prevents, or if capacity is below 2 × the reputation above 4,250, and with a balance of 5.25 million or more.

## 2. Club reputation

Per match (0x6d3de0) [observed]: a moving average heading to `reputation + (level / 2 + 5) × (team rating − average) × 50`, with weight **202 − 10 × level** (level is the competition's reputation, up to 20). In a level-10 league the weight is 102: a team 0.5 above average gains **+2.4 per match**, about 90 points in a season. The maximum step is 5 × level per match. Title: at most +750, up to the competition's reputation × 500. Promotion: +500. Relegation: −500.

## 3. Player reputation

Per match (0x6d26b0) [observed]: the three reputations (home, current, world) move by the match **rating** against the competition's average. **Goals and appearances don't count**, only the rating. The limits:
- **league cap:** 4,000 + 300 × level per match (a level-10 league: 7,000). A player in a low-reputation league can't pass it by playing;
- **ability cap:** CA × 50 + 2,500 (world: CA × 50 + 1,250);
- **world reputation** barely moves in club matches: weight 202 − 10 × level;
- maximum step per match: 250 to 375 × (2 − reputation / 10,000).

## 4. What the career and the survey showed

**Baseline career** (every league, 2001 to 2005) [measured]: average occupancy of 49% for clubs with reputation up to 2,000 and 85% for those from 8,000 to 10,000; 153 of the 1,122 clubs end with the average stuck at the record. A test career that pulled the average by reputation at season end freed the record (60 stuck), but was dropped: it grows with no cause on the pitch, and the small clubs barely change.

**The database** [measured in the 2001-052 tables]:
- the **record** (+0x7b) is the historical record, not today's support (Albion Rovers: average 250, record 18,321, a 2,496 stadium); it is above capacity for 415 of the 1,111 clubs;
- the **average** is today's picture; for 150 clubs it was filled from reputation (reputation² ÷ 2,000; nearly all of those, 128, also with the minimum = average ÷ 5), like Brazil's big clubs;
- **the stadium's expansion ceiling** (+0x44) is 0 for 69% of the clubs in level 16 to 20 leagues.

**Artificial growth already in the game** [observed, and measured in the baseline]: when it doesn't expand, the club converts standing places into seats and also adds the new places to total capacity, so a stadium with standing room grows every year, by a random draw and the balance, regardless of crowds or ceiling. In the baseline, 34 of the 66 stadiums that grew had ceiling 0 (Bahia: 115,000 → 134,000 with an average of 21,125 → 4,225). The "reputation × 2 above capacity" trigger also expands with no crowds.

**What the game already has that helps** [observed]:
- each club's predicted position (+0x72 of the competition entry) and the current one (+0x73);
- per match, a bonus for doing better than predicted; at season end, the average moves by (predicted − final + 1) and blends in the season's real attendance;
- the real attendance per competition (+5 home games, +6 summed attendance [inference]) and per season in the club records [inference];
- each match's round code (120 quarter-finals, 130 semi-final, 150 final) and the competition type.

**What spoils it** [observed]: the fixed record (it cuts the match and the average), the average's league caps (2,000, 3,000 or 5,000 × level) and, per match, the cut in leagues of level up to 4 (above 1,500 × level, only 20% of the excess).

**The stadium** (0x59cfa0) [observed]: owner, league and a balance above 5.25 million; a trigger of one league season above 92%; a 21,000-place build with a balance above 20 million, up to the ceiling and a third of it; no stadium above 125,000 expands; a cost of about 100 per place (875 for professional clubs, all seated). A real 2001 build cost, in order of magnitude, 1,000 to 3,500 pounds per place [inference, from memory].

## 5. The keys (implemented on 08/11, `src/publico.rs`, branch `estudos`)

Principle: attendance is **the support × the moment × the match**. It grows from what happens on the pitch and cools when performance returns to the expected; nothing grows without a cause. **The default is ours** (08/11, Lucas): with a `tweaks.cfg`, each missing key takes our rule; to go back to the original game, write the value in the "original" column. With no `tweaks.cfg` at all, nothing changes.

| Key | Our default | Original | What it does |
|---|---|---|---|
| `attendance` | `on` | `off` | the attendance switch (the keys from `attendance_league_caps` to `title_race`): `off` = the original game, whatever the rest says |
| `stadium` | `on` | `off` | the stadium switch (the `stadium_*` keys): `off` = the original game |
| `stadium_seating_growth` | `off` | `on` | converting to seats no longer adds to capacity |
| `stadium_reputation_trigger` | `off` | `on` | removes the "reputation × 2 above capacity" trigger |
| `attendance_league_caps` | `off` | `on` | removes the average's league caps and the weak-league cut per match |
| `attendance_record` | `real` | `fixed` | the record becomes the largest crowd that actually came |
| `pent_up_demand` | 0.1 | 0 | the share of a match's unmet demand that becomes a new record (10,000 people left out: the ceiling rises 1,000) |
| `tradition_weight` | 0.5 | 0 | a club finishing 5 places above its prediction moves 25% of the way to its historical record |
| `knockout_ceiling` | `stadium` | `record` | in knockout rounds (category and round code) only the stadium limits |
| `title_race` | `on` | `off` | the big game by the table: both can still be champions; up to ×1.75 in the run-in of a 1st v 2nd, and another × 1.5 if it is a derby |
| `stadium_full_seasons` | 2 | 1 | consecutive seasons above 92% to become eligible to expand |
| `stadium_occupancy` | `all` | `league` | occupancy counts every home match (league, cup, continental) |
| `stadium_size` | `demand` | `original` | the build matches the measured demand, up to the ceiling |
| `stadium_seat_cost` | `auto` | `original` | the cost per new place, in the database's currency; `auto` = 1,500 in the 2001 database, in proportion to the median cash of the reputation 8,000+ clubs in the loaded database (15.4 million in 2001, 56 million in CLZ 6.1: 5,450 per place); a number fixes the value |
| `stadium_ceiling_zero` | `demand` | `original` | for a stadium with no ceiling in the database, the ceiling becomes capacity plus the measured demand |
| `stadium_max` | 125000 | 125000 | the largest capacity that still expands |

With `log = 1`, three tables next to the exe: `publico_jogos.tsv` (each home match: demand, ceiling, crowd, record, title race), `publico_temporada.tsv` (each club at its league's season end: predicted, final, real attendance, average before and after) and `publico_estadio.tsv` (each build evaluation: occupancy, eligibility, demand, balance, places, cost and the reason).

**Compatibility** [observed, 08/11]: no NickPatcher option or MiscPatches patch touches the three replaced calls (0x6b86bc, 0x5d5cfb, 0x59cee4); `protect_against_virtual_staff_195_error` changes 13 bytes at the end of the match attendance routine, and it keeps working (the tweak calls the whole routine). In the CLZ 6.1, CLZ 5.4 and NickPatcher demo exes, the calls are as in 3.9.68. Coexistence with the NickDLL at runtime is still to be tested in the game.

The proof: 10-season careers with everything on (H), with the big game (I) and the control (the original only recording), compared season by season.
