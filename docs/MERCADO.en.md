# Market: transfers and loans (study)

*Versão em português: [MERCADO.md](MERCADO.md)*

Study of 07/11/2026, on the roadmap after the intake. Lucas's request: a player who stands out doesn't gain enough reputation to become wanted by the market, and a successful club takes too long to become attractive. Also the forum's old loan complaints ([EMPRESTIMOS.en.md](EMPRESTIMOS.en.md), [OPORTUNIDADES.en.md](OPORTUNIDADES.en.md) §2) and the rich club sitting on hundreds of millions.

Method, the same as for the regens: map it in the code (the routines already rewritten in Rust in the CM0102 project and the disassembly of the others), measure it in an automatic career with every league, and only then propose keys whose default equals the original. Reputation, which is what drives the market, is in [REPUTACAO_E_PUBLICO.en.md](REPUTACAO_E_PUBLICO.en.md).

**[observed]** = read in the code; **[measured]** = automatic career; **[inference]** = conclusion still to prove.

## 1. How the AI picks whom to sign

- **The candidates** come from the position lists, rebuilt **every 30 days** and sorted by the player's **value** (+0x52, routine 0x4d0c60: the ability seen by the coach × 50, reputation, age, position and nation) [observed].
- **The shop window:** 0x842c00 offers an available player to up to 5 clubs, but only if his current reputation is **5,500 or more** [observed].
- **The choice** (0x83f3a0 and 0x840d80) uses the selection score (position, condition, form, contract, reputation), seen by the coach or the scouts, and reputation draws for foreigners. **The average match rating doesn't enter**: performance only counts through form and reputation [observed].
- **The club** weighs by its reputation (+0x80) in almost everything: the budget cap (reputation below 3,500 caps it at reputation × 500), the bid draw (2 × reputation + 1,000), the player's willingness to go and the reputation the club looks for (0x83bf40) [observed].
- **The pace:** the club's watch list is only rebuilt every 30 to 100 days; a rejection blocks the slot for 120 days [observed].

So, for the market, **standing out means gaining reputation**. That is where ([REPUTACAO_E_PUBLICO.en.md](REPUTACAO_E_PUBLICO.en.md) §3) the game stalls: player reputation moves slowly and has a league cap.

## 2. Loans

Corrections to [EMPRESTIMOS.en.md](EMPRESTIMOS.en.md): 0x8c8090, 0x846920 and 0x84a020 are already in Rust in the CM0102 project, and **what decides the loan is 0x84af00**, not 0x8c8090. A due loan offer goes, through `process_due_offer` (0x8c63a0) and the holiday auto-response (0x784100), straight to 0x84af00; the rejection at the top of 0x8c8090 is only reachable from exchange callers, with no loan (practically dead code) [observed].

**The rule of 0x84af00** for a loan [observed]. An AI club only lends:
1. a player **listed for loan** (bit 2 of contract+0x4e), with the asking club's reputation × 1.25 above the player's current reputation; or
2. a player of squad status 6 or 7 (or a goalkeeper), with the asking club's reputation above 0.75 × the player's, **morale below −50** and the owner out of the Champions League.

Otherwise, it refuses. **Wage and period don't enter.** And the AI only loan-lists the player at the bottom of the squad in bad form or tired (0x83cb40) [observed]: the promising youngster who doesn't play almost never gets in.

**"Has no intention of going"** (0x8d8310 → 0x8d84e0): it comes out when the willingness score (0x846920) drops below 1.0 [observed]. For a loan it starts at 2.75, and the clubs' reputation bonus only counts when the player has a grievance (bit 0x10 of contract+0x45). The AI only asks for a loan with a score of 3.5 or more [observed].

**Stuck in holiday mode:** the offer dispatcher (0x8c63a0) has no branch for states 8, 9, 0xb, 0xc, 0xd and 0x13. An offer that was waiting for the human (state 9) when he went on holiday sits there forever [observed: the missing branch; inference: the forum case].

**End of contract:** `get_loan_period` (0x8d0590) pulls the loan end back until it fits the contract, and `can_loan_player` (0x8d0040) then finds no valid month [observed].

## 3. The idle rich club

The budget exists, but it goes through funnels [observed]:
1. **The budget** (0x598800): the balance minus reserves, half of the rest, a 40 million cap in some nations and a per-nation compression (Germany, France, Scotland and the Netherlands above 17.5 million only 25% of the excess; Brazil above 2.5 million; the Nordic countries and others above 2.5 million only 10%). England, Spain and Italy have no compression.
2. **The split** (0x83b6e0): only the first 8 slots get budget, divided by the number of open slots × 2 to 10.
3. **The candidates** (0x83f3a0): price × 1.3 within the slot's budget, willingness 3.15 or more, and the wage asked within 1.2 × the wage cap.
4. **The bid** (0x84a020): draw(value) > draw(2 × reputation + 1,000).
5. **The pace:** list rebuilt every 30 to 100 days; a 120-day block after a rejection.

## 4. What the career measured

Loans and the budget were measured and became keys: see [EMPRESTIMOS.en.md](EMPRESTIMOS.en.md) §4. The rest of the study from the 11 and 12/11/2026 careers, each item tested on its own:

- **The easier bid** (the draw against reputation + 1,000, not 2 × reputation + 1,000): +4% to 7% activity.
- **The standout** (a starter's willingness to go to a much bigger club, × 1.6): +22% players moving up in the 2nd season.
- **The Key player's price** (× the asking price) and the **chairman's Business** in the budget: within the noise, or they only change the budget.
- **The fixes to the release clause (0x86165b) and to the reserve-list loan (0x83e482)**: within the noise.

## 5. Keys

- **Loans and the budget brake** (1.5): `loans`, the `loan_*` keys and `budget_caps`, in [EMPRESTIMOS.en.md](EMPRESTIMOS.en.md) §5.
- **Transfers** (the standout, the bid, the market's interest): the next study, with the same method.
