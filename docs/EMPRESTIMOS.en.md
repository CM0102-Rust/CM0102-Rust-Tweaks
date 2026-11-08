# Loans: how the game decides (a map for the tweaks)

*Versão em português: [EMPRESTIMOS.md](EMPRESTIMOS.md)*

The market's most repeated complaint on the forum ([OPORTUNIDADES.en.md](OPORTUNIDADES.en.md) §2): "the loan offer is rejected, no matter what wages or period or the player has no intention of going on loan" (JLa, t=1593). Loans get stuck in holiday mode (xeno's list, item 88), and the human can't sign on-loan players through Bosmans (58.2). This document maps who decides, from the routines already rewritten in Rust in the CM0102 project and the pseudocode of those still in the original. **[observed]** = read in the code; **[inference]** = a conclusion still to prove.

## The pieces

| Routine | Name (community map) | What it decides | Status in the project |
|---|---|---|---|
| 0x8c8090 | `TRANSFER_MANAGER::ai_respond_to_bid` | an AI club's answer to a bid | Rust |
| 0x84af00 | `shortlist_manager_predict_selling_club_response` | the prediction: accepts, haggles over or rejects a bid or loan request | Rust |
| 0x846920 | `Shortlist_Manager::staff_interest_in_joining_club` | a person's willingness score to join (or stay at) a club, with a reason marked per factor | Rust |
| 0x842740 | `Shortlist_Manager::try_loan_offer_for_player` | the AI making a loan offer for a player on its shortlist | Rust |
| 0x84a020 | `Shortlist_Manager::make_club_bids` | the AI walking its 40 shortlist slots and making bids or loan requests | Rust |
| 0x8bed60 | `TRANSFER_MANAGER::make_loan_offer` | building the loan offer | Rust |
| 0x8bfa30 | `TRANSFER_MANAGER::reject_loan_offer` | rejecting and posting the news | Rust |
| 0x4d01e0 | `CONTRACT_MANAGER::sign_loan_contract` | signing: the player joins the borrowing club's squad | Rust |
| 0x8d0590, 0x8d0530 | `get_loan_period`, `can_end_loan` | the period and whether it can end early | Rust |
| `*_RULES::can_loan_player` | per-country rules | windows, limits (England: 5 a season, 2 at once, none within the same league; Italy, Ireland, Spain, Scotland...) | original, per country |

## What the code says

1. **Correction (07/11/2026):** the rejection of every loan at the start of 0x8c8090 (`+0x1f = 1` → `reject_loan_offer`) is only reached by exchange callers, with no loan: it is practically dead code. A due loan offer goes, through `process_due_offer` (0x8c63a0) and the holiday auto-response (0x784100), straight to 0x84af00, which is **what decides** [observed]. See [MERCADO.en.md](MERCADO.en.md) §2.
2. **The loans that do happen go through the prediction (0x84af00), which accepts only in a few cases** [observed]:
   - the lending club is human;
   - the player is **on the owner's loan list** and the requesting club's reputation times 1.25 (0x968928) exceeds the player's current reputation (+0xb of the record);
   - a narrow case: squad status 6 or 7 (+0x4f), or a goalkeeper, with the requester's reputation above a fraction of the player's, contract +0x3d below −50 and a club condition (+0x1db);
   - otherwise: **reject**. The offered wage plays no part.
3. **The AI only asks for a loan when the player is willing** (0x842740): the interest score (0x846920) must be at least 3.5 (0x968e38), the period must fit the contract and the window, and the wage paid is 100%, 50% or 0% by the reputation gap between the clubs [observed].
4. **"No intention of going":** the message exists in the exe ("... has no intention of going on loan to ..."). The inference is that it comes from the same interest score (0x846920) below the minimum, but the exact spot on the human's screen hasn't been read yet [inference].

**What this explains** [inference from the code above]:
- "Rejected no matter the wage": for a player not on the loan list, the answer is always no, and wage isn't a factor.
- "The big club can't loan out its youngsters": the requester needs reputation × 1.25 above the youngster's, and the big club's players only leave on loan if the owner listed them.

## The proposed approach

1. **Measure before changing anything.** A logging mode in the DLL, like the regens one: every loan request and prediction (club, player, reputations, listed or not, squad status, result), every offer, rejection and signing. An automatic career on the test bench answers how many requests are made, how many are rejected and by which rule.
2. **Configurable rules**, to be decided with Lucas after the measurement. Ideas:
   - the AI club **considers** a request for a player not on the list when he isn't in its plans (backup or not-needed status, few games), with a chance, instead of always refusing;
   - the reputation rule becomes **gradual** (a chance that grows with the gap), instead of a 1.25 cut-off;
   - **youngsters for development:** the AI club loan-lists youngsters with little playing time and PA well above CA;
   - loans stuck in holiday mode (xeno's item 88): find the state in which the offer stalls.
3. **Each country's rules stay** (windows, per-season limits).
4. **How to hook in:** wrap the 7 calls to 0x84af00: call the original and, when it rejects a loan, apply the new rule. That is what the experimental key `loan_rule = squad` does (branch `estudos`, [MERCADO.en.md](MERCADO.en.md) §5).
