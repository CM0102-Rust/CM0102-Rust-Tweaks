# Empréstimos: como o jogo decide (mapa para os ajustes)

*English version: [EMPRESTIMOS.en.md](EMPRESTIMOS.en.md)*

A reclamação mais repetida do mercado no fórum ([OPORTUNIDADES.md](OPORTUNIDADES.md) §2): "a oferta de empréstimo é recusada qualquer que seja o salário ou o período, ou o jogador não tem intenção de ir" (JLa, t=1593). Os empréstimos ficam presos no modo férias (lista do xeno, item 88), e o humano não consegue pegar emprestado jogadores em fim de contrato (58.2). Este documento mapeia quem decide, a partir das rotinas já reescritas em Rust no projeto CM0102 e do pseudocódigo das que ainda estão no original. **[observado]** = lido no código; **[inferência]** = conclusão a provar.

## As peças

| Rotina | Nome (mapa da comunidade) | O que decide | Estado no projeto |
|---|---|---|---|
| 0x8c8090 | `TRANSFER_MANAGER::ai_respond_to_bid` | a resposta de um clube da IA a um lance | original |
| 0x84af00 | `shortlist_manager_predict_selling_club_response` | a previsão: aceita, negocia ou recusa um lance ou um pedido de empréstimo | Rust |
| 0x846920 | `Shortlist_Manager::staff_interest_in_joining_club` | a nota de vontade de uma pessoa de ir (ou ficar) num clube, com um motivo marcado por fator | original (13 KB, 3.881 instruções) |
| 0x842740 | `Shortlist_Manager::try_loan_offer_for_player` | a IA oferecer um empréstimo por um jogador da lista dela | Rust |
| 0x84a020 | `Shortlist_Manager::make_club_bids` | a IA percorrer as 40 vagas da lista e fazer lances ou pedidos de empréstimo | original |
| 0x8bed60 | `TRANSFER_MANAGER::make_loan_offer` | montar a oferta de empréstimo | Rust |
| 0x8bfa30 | `TRANSFER_MANAGER::reject_loan_offer` | recusar e mandar a notícia | Rust |
| 0x4d01e0 | `CONTRACT_MANAGER::sign_loan_contract` | assinar: o jogador vai para o elenco do clube que pegou | Rust |
| 0x8d0590, 0x8d0530 | `get_loan_period`, `can_end_loan` | o período e se dá para encerrar antes | Rust |
| `*_RULES::can_loan_player` | regras por país | janelas, limites (Inglaterra: 5 por temporada, 2 ao mesmo tempo, nada dentro da mesma liga; Itália, Irlanda, Espanha, Escócia...) | original, por país |

## O que o código diz

1. **O clube da IA recusa todo lance de empréstimo que chega pela resposta a lances.** Logo no começo da 0x8c8090, uma oferta com `+0x1f = 1` (empréstimo) vai direto para `reject_loan_offer`, sem olhar o salário, o período ou o jogador [observado: o pseudocódigo da 0x8c8090, linhas 72–75].
2. **Os empréstimos que acontecem passam pela previsão (0x84af00), que só aceita em poucos casos** [observado]:
   - o clube que empresta é humano;
   - o jogador está **na lista de empréstimos** do dono e a reputação de quem pede, vezes 1,25 (0x968928), passa a reputação atual do jogador (+0xb da ficha);
   - um caso estreito: estatuto no elenco 6 ou 7 (+0x4f), ou goleiro, com a reputação de quem pede acima de uma fração da do jogador, +0x3d do contrato abaixo de −50 e uma condição do clube (+0x1db);
   - fora disso: **recusa**. O salário oferecido não entra na conta.
3. **A IA só pede emprestado quando o jogador quer** (0x842740): a nota de interesse (0x846920) tem de ser ao menos 3,5 (0x968e38), o período tem de caber no contrato e na janela, e o salário pago é 100%, 50% ou 0% pela diferença de reputação entre os clubes [observado].
4. **"Sem intenção de ir":** a mensagem existe no exe ("... has no intention of going on loan to ..."). A inferência é que ela sai da mesma nota de interesse (0x846920) abaixo do mínimo, mas o ponto exato da tela do humano ainda não foi lido [inferência].

**O que isso explica** [inferência pelo código acima]:
- "Recusado qualquer que seja o salário": para um jogador fora da lista de empréstimos, a resposta é sempre não, e o salário não é um fator.
- "O clube grande não consegue emprestar os jovens": quem pede precisa de reputação × 1,25 acima da do jovem, e os jogadores do clube grande só saem por empréstimo se o dono os listou.

## A abordagem proposta

1. **Medir antes de mexer.** Um modo de registro na DLL, como o dos regens: cada pedido e cada previsão de empréstimo (clube, jogador, reputações, listado ou não, estatuto, resultado), cada oferta, recusa e assinatura. Uma carreira automática na bancada responde quantos pedidos são feitos, quantos são recusados e por qual regra.
2. **Regras configuráveis**, a decidir com o Lucas depois da medição. Ideias:
   - o clube da IA **considera** o pedido de um jogador fora da lista quando ele não está nos planos (estatuto de reserva ou dispensável, poucos jogos), com uma chance, em vez de recusar sempre;
   - a regra da reputação vira **gradual** (uma chance que cresce com a diferença), em vez de um corte de 1,25;
   - **jovens para desenvolver:** o clube da IA põe na lista de empréstimos os jovens com pouco tempo de jogo e PA bem acima do CA;
   - os empréstimos presos no modo férias (item 88 do xeno): achar o estado em que a oferta fica parada.
3. **As regras de cada país ficam** (janelas, limites por temporada).
4. **Como entrar na rotina:** a 0x8c8090 trata todos os lances. O ajuste desvia a entrada, decide os lances de empréstimo pela regra nova e devolve os outros à rotina do exe, como o modo de observação da disputa faz hoje.
