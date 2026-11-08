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

## 4. What the career measured

*(to be filled in with the 07/11 career)*

## 5. Proposed keys

*(to be filled in)*
