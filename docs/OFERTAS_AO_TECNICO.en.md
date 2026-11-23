# Job offers to the human manager

*Versão em português: [OFERTAS_AO_TECNICO.md](OFERTAS_AO_TECNICO.md)*

Forum request (11/22/2026): "if you go far in a continental competition or win titles, your reputation goes up and bigger clubs, abroad too, should take an interest; I went up divisions several times and never got an offer". This document describes how the game picks a manager for a vacancy, what blocks the human manager, and the measurements made. **Study; no key yet.**

## 1. How the game offers a job

There is only one way for an offer to reach the human:

1. A vacancy opens (`add_job_vacancy` 0x69ba50): the game goes through every person, including the 16 human places at the end of the table, applies filters and keeps the highest score from `score_staff_for_job` 0x69d1b0.
2. On the set date, `daily_update` 0x68f9c0 appoints the chosen one; if he is human, instead of appointing him it sends the offer (`post_job_offer_to_human_news` 0x6a4f80, called only at 0x68fc68).

There's no "court the human" rule: he gets an offer only when he wins the vacancy against every AI manager. Applying (`apply_for_job` 0x69ca00) goes through the same score.

## 2. What blocks the human

In the order the loop at 0x69ba50 tests them:

| Rule | Where | What it requires |
|---|---|---|
| League running | `club_can_be_managed` 0x544820 (humans only) | the club in a loaded league; without the foreign leagues on, there's never an offer from abroad |
| Foreign | 0x69ba50 | a club of another nation: the manager's home reputation (+0xa of the +0x69 record) above 3,749 |
| Other region | 0x69ba50 (the nation's +0x75 byte) | a different region: world reputation (+0xc) above 4,249 **and** the club above 4,749 |
| Suitability | `is_staff_suitable_for_job` 0x69f810 | for someone employed: the new club in a division of higher reputation, or in the same one with 1,250 more reputation, and the interest (below) of at least 3.5 |
| Interest | `staff_interest_in_job_at_club` 0x844780 | **0 in the first `(+0x59 + 8) × 15` days in the job** (420 days with 20), for any manager; 0 at a club below 2,750 of the same nation |
| Score | `score_staff_for_job` 0x69d1b0 | the highest among the candidates; **for the human the attributes and personality part is skipped** (`id ≥ N − 16`): what's left is the reputation as the club sees it, plus 100 if he's already at the club |

The manager's reputation changes after each match with the average rating (`update_manager_reputation_after_match` 0x6d3610), capped by the competition's reputation: in national competitions the world one's cap is halved (at least 3,000); only continental ones let it go beyond that. Titles, promotions and cups raise it too (`Staff_Records`: `add_league_champions_record` 0x868c50, `add_promotion_record` 0x869480, `add_cup_winners_record` 0x86a800), but with the same kind of cap: the league's reputation × 400 for the current and home ones. In a division of reputation 9, that's 3,600: winning in the lower divisions barely moves the reputation.

The same `Staff_Records` (`[0xdd7f00]`) keeps, for each manager, a 210-byte record (`get_manager_record` 0x86f5b0) with the score the game itself works out: cups (+0x8a), league titles (+0x8c), games, wins, draws and losses (+0x8e to +0x94), and weighted achievement points (title, cup, rounds, by the competition's reputation) summed over the year (+0xb6), the season (+0xba) and, from what we see, the career (+0xca). It's the natural basis for a merit score.

## 3. Measurements

Study `TWEAKS_ESTUDO=1` (`src/estudo.rs`, `estudo_vagas.tsv`): at each club vacancy, one line per human with the filters, the suitability, the interest (with its reason bits), his score and the winner's. `TWEAKS_REP_TECNICO=c,h,w` forces the human's three reputations; `TWEAKS_BONUS_TECNICO=N` adds N to his score in the loop. Hidden automatic careers on 3.9.68, leagues ENG BRA ITA, with the package's `tweaks.cfg`; the game's test manager changes nation and club in each career.

| Career | Vacancies | Blocked (foreign / region / suitability / score) | Wins | Human × winner score (median) |
|---|---|---|---|---|
| Real reputation, 1 season (América-RJ, Série C) | 82 | 36 / 0 / 43 / 3 | 0 | 3,389 × 7,005 |
| Reputation 8,000, 1 season (Cagliari) | 82 | 0 / 38 / 43 / 1 | 0 | 4,356 × 7,547 |
| Real reputation, 2 seasons (Catania, up from C to B) | 158 | 117 / 0 / 21 / 20 | 0 | 641 × 7,678 |
| Reputation 8,000, 2 seasons (Botafogo, then Guarani) | 177 | 0 / 64 / 102 / 9 | **2** | 3,641 × 8,730 |

- **In the first year in the job the interest is 0** and no vacancy is possible, by a game rule that holds for everyone.
- **After that, the human passes his own nation's filters and loses on score.** With the real reputation (Catania, 2,400 to 3,200), his scores are 2,000 to 3,500 against the winners' 5,000 to 14,000 (Palermo, Sampdoria, Genoa, Ancona).
- **With reputation 8,000 the score gets close** (10,000 to 10,800 against 11,400 to 12,000) and, unemployed, he wins two vacancies (Guarani and Botafogo, 2003); Guarani's became an offer and a job.
- **The human's reputation doesn't rise with success:** at Catania, with promotion from Série C to B, it fell from 3,177 to 2,709 (the home one, from 3,177 to 2,459).

## 4. Possibilities

| | What | Effect on the world |
|---|---|---|
| A | This study (done) | none |
| B | A bonus to the human's score, making up for the skipped attributes part | only at vacancies where the human already passes the filters |
| C | Loosen the foreign and region filters for humans only | same |
| D | The manager's reputation growing with success (promotion, title, continental run) | bigger: reputation goes into everything |
| E | A parallel offer when the human is among the best | medium: the vacancy takes longer to fill |

### 4.1 The bonus experiment (option B)

`TWEAKS_BONUS_TECNICO` adds the bonus to the human's score in the vacancy loop; two two-season careers with the real reputation:

| Bonus | Vacancies | Blocked (foreign / region / suitability / score) | Wins | What happened |
|---|---|---|---|---|
| 3,000 | 155 | 84 / 0 / 54 / 11 | **6** | sacked twice (Volta Redonda, Mirassol); unemployed, he won 6 small clubs' vacancies and was hired by Mirassol and Sorocaba |
| 6,000 | 160 | 0 / 52 / 108 / 0 | 0 | at Palmeiras the whole career: nothing bigger to move up to, and the vacancies abroad blocked by the region |

The bonus works where the human already passes the filters: in practice, when he's unemployed. **Employed, the block is suitability**, which comes before the score: the first year in the job (interest 0) and the requirement to move up a division or to a bigger club.

### 4.2 A detail about the human

Humans have +0x59 at 20 (the maximum): the time-in-job rule asks for `(20 + 8) × 15 = 420` days, the longest possible. For an AI manager with 10, it's 270. The game treats the human as the most loyal manager there is.

The `TWEAKS_LEALDADE_TECNICO=10` experiment (the humans' +0x59 at 10), two two-season careers with the real reputation:

| Loyalty | Bonus | Vacancies | Blocked (foreign / region / suitability / score) | Wins | What happened |
|---|---|---|---|---|---|
| 10 | 0 | 169 | 0 / 128 / 34 / 7 | 0 | at Stoke City (Championship), passed suitability for Newcastle, Wolverhampton, Crystal Palace, Chelsea and West Bromwich (all Premier League) and lost on score: 5,400 to 6,900 against 10,500 to 15,600 |
| 10 | 3,000 | 153 | 114 / 0 / 29 / 9 | 1 | at Prato (Serie C), passed for Milan, Bologna and Reggiana and lost on score (6,000 against 6,700 to 13,000); unemployed, he won Ternana's vacancy and was hired |

**With loyalty at 10 the window opens**: the employed human starts being considered by the bigger clubs of his own nation. The next block is the score, 4,000 to 9,000 points behind the winners at the big clubs; a 3,000 bonus doesn't get there.

### 4.3 Recommendation

The report asks for three things the game doesn't do today, and each has a cause:

1. **"I won and nobody wanted me"**: the manager's reputation barely rises with promotions and titles, because of the league's cap (§2), and the human's even falls. Option D goes at the cause: a key for the human manager's reputation to grow with success (promotion, title, finishing above the board's expectation, continental run), through the same hook on `update_manager_reputation_after_match` 0x6d3610. With a higher reputation, the foreign and region filters pass and the score rises, without touching the selection rules.
2. **"I never got an offer while employed"**: suitability. A key that treats the human as a manager of average loyalty (the human's +0x59 at 10: 270 days in the job) is the smallest change that opens the window, and touches only the 16 human places.
3. **The human's score**, which lacks the attributes part: the bonus (B) completes the two above; on its own, it only helps the unemployed.

Before 2, it remains to check where else the game reads a manager's +0x59 (the suitability at 0x69f810 uses it too, `(+0x59 + 21) × 10` days), so as not to change something else about the human along with it; the alternative is to change only the time-in-job count inside 0x844780 when the person is human.

Suggested order: 2 (effect on the human only), then 1 (the big change, with a long test), and the bonus only if the two aren't enough. The AI keeps choosing by the same rules; the human becomes a real candidate.

By the measurements, 2 alone opens the door but gives no offer: the score gap at the big clubs (4,000 to 9,000) calls for 1, reputation growing with success, or a bonus of around 5,000 to 8,000, which would be an arbitrary number. 1 is the answer the report asks for.

## 5. The new logic: `manager_offers` (prototype, off)

Lucas's request (11/23): change the logic instead of inflating values. The vacancy contest mixes two questions, and the prototype (`src/jogo/ofertas.rs`) separates them for human managers:

- **"Would he come?"** (the manager's side: the interest 0x844780, the first year in the job, the requirement to move up in 0x69f810). For the human, the player answers when the offer arrives. With the key, the human's suitability in the two candidate loops of 0x69ba50 (the calls at 0x69bdf7 and 0x69bf77) is only a brake: being unemployed, or the vacancy being at a club with a higher reputation than his current one.
- **"Does the club want him?"** (the score 0x69d1b0, called at 0x69be55). For the human, the attributes part the game skips becomes a merit score, from 0 to 6,000, from what the game itself records: 50% the current club's board confidence (the 0x49-byte record of MANAGER_MANAGER, +6 to +0xc, from 2,000 to 10,000), 30% the win rate (from 20% to 60% wins, with at least 10 games) and 20% the titles and cups (the manager's 210-byte record, +0x8c and +0x8a).

The club's filters (foreign, region, running leagues) and national team vacancies stay as in the original. The key is `manager_offers = 1` in `tweaks.cfg`; without it, nothing changes. In 1.9 it's in the configurator, off (experimental).

**Measured** (two hidden two-season careers, 11/23, managers of Italian Serie C clubs with weak campaigns): employed, the human became a candidate for the vacancies at Napoli, Vicenza, Genoa, Triestina and Pistoiese (blocked by suitability before); his score rose from ~3,500 to 6,200 to 6,500; he won the Catania and Brescello vacancies and lost the big clubs' to managers with reputation 6,000 to 8,000, consistent with the campaign. **Still to do:** a career with a successful human manager (the automatic career wins nothing) and Lucas's decision on the scale and weights.
