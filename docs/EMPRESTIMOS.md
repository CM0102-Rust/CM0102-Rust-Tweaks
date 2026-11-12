# Empréstimos: como o jogo decide e as nossas regras

*English version: [EMPRESTIMOS.en.md](EMPRESTIMOS.en.md)*

A reclamação mais repetida do mercado no fórum ([OPORTUNIDADES.md](OPORTUNIDADES.md) §2): "a oferta de empréstimo é recusada qualquer que seja o salário ou o período, ou o jogador não tem intenção de ir" (JLa, t=1593). Este documento explica quem decide no jogo original, as regras que esta DLL propõe (chave `loans`), o que as carreiras de teste mediram e cada chave. **As regras são uma proposta**: a ideia é que a comunidade jogue, olhe os números e proponha mudanças (§7).

**[observado]** = lido no código; **[medido]** = carreira automática; **[inferência]** = conclusão a provar.

## 1. Como o jogo original decide

| Rotina | Nome (mapa da comunidade) | O que decide |
|---|---|---|
| 0x84af00 | `predict_selling_club_response` | **o dono**: aceita, negocia ou recusa um lance ou um pedido de empréstimo |
| 0x846920 | `staff_interest_in_joining_club` | **a vontade** do jogador de ir para um clube (uma nota de 0 a 10) |
| 0x83cb40 | `assess_club_squad` | a avaliação do elenco: quem vai para as listas de transferência e de empréstimo |
| 0x83f3a0 | `fill_shortlist_slot` | **quem o clube procura** para cada vaga da lista dele |
| 0x84a020 | `make_club_bids` | a IA percorre a lista e faz lances e pedidos de empréstimo |
| 0x842740 | `try_loan_offer_for_player` | o pedido de empréstimo da IA |
| 0x598800 | a verba de transferências | quanto o clube pode gastar |

1. **O dono quase sempre recusa** [observado]. Um clube da IA só empresta:
   - o jogador que ele mesmo pôs na lista de empréstimos, com a reputação de quem pede × 1,25 acima da do jogador; ou,
   - num caso estreito, o reserva infeliz.

   Fora disso, recusa. **O salário e o período não entram na conta.**
2. **A lista de empréstimos quase não tem jovens** [observado]. A IA só lista o jogador do fim do elenco em má fase ou cansado, então o jovem promissor que não joga fica parado.
3. **A IA só pede quando o jogador quer** [observado]. A nota de vontade precisa ser 3,5 ou mais, e o pedido passa por um sorteio contra a reputação do clube / 10.
4. **"Sem intenção de ir"** sai quando a nota de vontade fica abaixo de 1,0 [observado].
5. **A verba é comprimida em algumas nações** [observado]:
   - Alemanha, França, Escócia e Holanda só usam 25% do que passa de 17,5 milhões;
   - Brasil, os nórdicos e outros, só uma parte acima de 2,5 milhões;
   - Inglaterra, Espanha e Itália não têm freio.

## 2. A ideia das nossas regras

O empréstimo existe para três coisas:

- **Rodar o jovem**: quem tem até 21 anos e não é titular sai para jogar, para um clube onde seria um dos dois primeiros da posição e que não treine muito pior.
- **Dar minutos a quem sobra**: o jogador do elenco que tem quem o cubra vai para onde jogaria (o Flamengo com três volantes bons empresta um).
- **Voltar para casa**: o jogador que está fora do país vai com mais vontade para um clube da nação dele, e o dono pede menos cobertura para liberá-lo.

E um limite: **o titular absoluto (Key) nunca sai emprestado.** Mas o titular que um clube bem maior quer, e o dono não libera, fica chateado, e isso também mexe o mercado. As decisões do clube humano continuam do jogador.

## 3. As regras

**A. A decisão do dono** (0x84af00, para todo pedido de empréstimo a um clube da IA):
1. Estatuto Key: recusa.
2. **Minutos.** O jogador precisa ficar entre os melhores da linha dele **no clube que pede**, pelo CA:
   - jovem (até `loan_youth_age` anos): entre os `loan_youth_top` primeiros;
   - os outros: entre os `loan_others_top` primeiros.

   A linha é a da melhor nota de posição: goleiro, defesa (líbero, zagueiro, ala), meio (volante, meia, meia ofensivo) ou ataque. Ninguém vai emprestado para o banco.
3. **Cobertura.** O dono precisa ficar com jogadores da mesma linha com CA de ao menos `loan_cover_ratio`% do do emprestado:
   - para um titular (Regular starter): `loan_cover_regular` jogadores;
   - para rotação e reserva: `loan_cover_backup`;
   - para o jovem: nenhum.
4. **Treino.** O jovem só vai para um clube com Treino de ao menos o do dono − `loan_training_margin`.
5. **Volta para casa** (o destino é da nação do jogador e o dono não): a cobertura pede um a menos, e o treino não conta.
6. **Piso do elenco.** Com `loan_squad_min` jogadores ou menos no clube, o dono não empresta ninguém.

**B. A lista de empréstimos**: depois de cada avaliação do elenco, o dono põe na lista quem a regra A emprestaria (sem olhar o destino). Assim o jogador aparece nas listas de procura dos outros clubes.

**C. Quem pede**:
- o pedido da IA precisa de vontade `loan_request_willingness` (o original pede 3,5);
- a IA tenta `loan_requests` vezes mais;
- a vontade do jogador de ir emprestado é multiplicada:
  - por `loan_home`, para um clube da nação dele, estando fora dela;
  - por `loan_playing_time`, para onde ele seria o 1º ou o 2º da linha.

**D. A procura por jovens** (0x83f3a0): quando um clube da IA escolhe quem pedir numa vaga de empréstimo, o jovem que seria um dos `loan_youth_top` da linha ali tem a nota multiplicada por `loan_youth_demand`. Ele também precisa ir para casa ou para um treino dentro da margem. Sem isso, os adultos que agora sobram ganham as vagas e os jovens emprestados caem pela metade [medido].

**E. O titular barrado**: numa decisão de verdade (não nas previsões das listas de procura), quando o dono recusa o empréstimo de um titular (Key ou Regular starter) para um clube com reputação ao menos `loan_unhappiness_gap` acima da dele, o jogador perde `loan_unhappiness` de felicidade no contrato. Infeliz, ele fica mais perto de pedir para sair.

**F. A trava da verba**: com `budget_caps = off`, nenhuma nação tem a verba comprimida (todas como a Inglaterra).

## 4. O que as carreiras mediram

Carreiras automáticas com todas as ligas do banco, de julho de 2001 a julho de 2003, sem clube humano. Cada número vem de uma carreira só; o ruído entre carreiras iguais é de uns ±5% e ±0,1 a 0,3 de CA [medido].

| Por temporada (2001/02 · 2002/03) | Original | Estas regras |
|---|---|---|
| Empréstimos | 3.255 · 4.042 | 5.467 · 5.469 |
| Jovens promissores emprestados (até 21, PA − CA ≥ 30) | 680 · 698 | 726 · 692 |
| Bons jogadores subindo (CA 120+ para um clube maior) | 259 · 387 | 456 · 920 |
| Titulares saindo dos grandes (CA 140+) | 70 · 104 | 136 · 182 |
| Valor das transferências (milhões) | 1.253 · 1.464 | 1.630 · 1.978 |
| Evolução dos promissores (CA por ano) | 11,56 · 10,55 | 11,78 · 10,70 |
| Clubes no vermelho no fim da temporada | 48,7% · 47,6% | 48,1% · 48,8% |

O que cada parte fez, uma de cada vez [medido]:

- **Só a verba sem freio**: os clubes ricos que tinham teto ganham +27% a +43% de verba e gastam mais (de 104 para 143 milhões e de 133 para 166 milhões). As finanças não mudam.
- **A e B (o dono e a lista)**: os empréstimos sobem 70%, e os bons jogadores que sobem para um clube maior dobram. **Mas os jovens emprestados caem pela metade**: a IA pedia o jovem para o banco, e a regra A recusa.
- **C (casa e minutos)**: mais uns 500 a 850 empréstimos por temporada.
- **D (a procura por jovens)**: os jovens emprestados voltam ao nível do original: 681 · 653 com ×2, 726 · 692 com ×3.
- **E (o titular barrado)**, com 30 de felicidade e 1.000 de diferença: 6 casos em duas temporadas. É raro porque o clube maior quase nunca pede um titular emprestado; a chave fica para quando as transferências mexerem nisso.
- **O piso do elenco**: sem ele, três de quatro carreiras caíram:
  - numa, um clube emprestou até não ter time, e o jogo parou com "Only 10 away players";
  - noutra, um erro de copa;
  - na terceira, uma falha de memória.

  Com o piso em 20, nenhuma caiu.

## 5. As chaves

No `tweaks.cfg`, ou na aba **Empréstimos e trava** do configurador. Sem `tweaks.cfg` nada muda; com ele, a chave ausente usa o valor da coluna "padrão". **O padrão é o que foi testado** nas carreiras do §4. Na coluna "original", "sem a regra" quer dizer que o jogo original não tem aquela conta: o jovem, por exemplo, vai para qualquer treino.

| Chave | Padrão | Original | O que faz |
|---|---|---|---|
| `loans` | on | off | liga as regras A a E; off = os empréstimos do jogo original |
| `loan_youth_age` | 21 | sem a regra | até esta idade, quem não é Key conta como jovem |
| `loan_youth_top` | 2 | sem a regra | o jovem só vai para onde seria do 1º ao Nº da linha |
| `loan_others_top` | 2 | sem a regra | o mesmo para os outros |
| `loan_training_margin` | 3 | sem a regra | o jovem só vai para um Treino ≥ o do dono − isto (0 = igual ou melhor) |
| `loan_cover_regular` | 2 | sem a regra | cobertura para emprestar um titular |
| `loan_cover_backup` | 1 | sem a regra | cobertura para emprestar rotação e reserva |
| `loan_cover_ratio` | 90 | sem a regra | quem cobre: CA de ao menos este % do dele |
| `loan_squad_min` | 20 | 0 | com tantos jogadores ou menos, o dono não empresta |
| `loan_request_willingness` | 2.75 | 3.5 | a vontade mínima para a IA pedir |
| `loan_requests` | 2 | 1 | vezes os pedidos do original (1, 2, 4, 8...) |
| `loan_home` | 2 | 1 | a vontade de voltar para casa, vezes |
| `loan_playing_time` | 1.3 | 1 | a vontade de ir para onde seria 1º ou 2º da linha, vezes |
| `loan_youth_demand` | 3 | 1 | a nota do jovem que jogaria ali, na escolha do alvo da vaga de empréstimo |
| `loan_unhappiness` | 30 | 0 | a felicidade que o titular barrado perde |
| `loan_unhappiness_gap` | 1000 | sem a regra | quanto maior (reputação) o clube que pede precisa ser |
| `budget_caps` | off | original | off = nenhuma nação com a verba comprimida |

Com `log = 1`, a linha `tick` do `tweaks.log` traz, a cada mês do jogo:
- `loan_listed`: as marcações na lista de empréstimos;
- `loan_real_yes` e `loan_real_no`: as decisões do dono, fora as previsões das listas de procura;
- `young_sought`: as vezes em que um jovem levou vantagem numa vaga de empréstimo;
- `unhappy`: os titulares barrados que ficaram chateados.

## 6. O que não entrou, e o que ainda não se sabe

- **O titular barrado é raro** (6 casos em duas temporadas): o clube maior quase nunca pede um titular emprestado. Para que ele aconteça mais, o caminho são as transferências (o próximo estudo) ou valores maiores nas chaves.
- **Os minutos quase não mudam a evolução do jovem** [medido]:
  - no original, o promissor emprestado ganha +9,9 de CA com 0 jogos de liga e +10,8 com 10 a 19;
  - em todas as carreiras, inclusive no original, **o jovem emprestado evolui uns 2,5 de CA por ano menos que o que fica**, e quase todos vão para clubes menores, que treinam pior;
  - o motor do jogo forma pelo treino, pela idade e pelo potencial.

  Por isso existe a margem do treino. Com `loan_training_margin = 0`, cada jovem vai para um lugar melhor, mas um terço deles deixa de ser emprestado.
- **Não medido ainda**: carreiras longas (10 temporadas), carreiras com o clube humano e os bancos da comunidade (como o CLZ).
- **O empréstimo do clube humano** continua como no jogo: o clube humano decide o que empresta, e um pedido do humano a um clube da IA passa pela regra A.

## 7. Como propor uma mudança

Cada regra é uma chave, e o original é sempre um valor possível: dá para desligar uma parte só. Para propor:

1. Mude as chaves no configurador e jogue (ou rode uma carreira de férias).
2. Com `log = 1`, mande o `tweaks.log` junto da proposta: os contadores da linha `tick` mostram o que as regras fizeram.
3. Diga o que no futebol a regra deveria refletir. É assim que estas nasceram: o jovem roda, quem sobra joga, o jogador volta para casa, o titular não sai, e o titular barrado se chateia.

## Notas técnicas

Os pontos onde cada regra entra no exe:

| Regra | Ponto no exe |
|---|---|
| A decisão do dono | embrulha a 0x84af00 (o trampolim confere os 6 primeiros bytes) |
| A avaliação do elenco | a chamada em 0x83a763 |
| A vontade | embrulha a 0x846920 |
| O limiar do pedido | o `fcomp` em 0x8427f0 |
| O sorteio do pedido | o `sar edx, 2` em 0x84aaf9 |
| A procura por jovens | o sorteio da nota final em 0x840610 (`push 0x11; call 0x90cfc0`) |
| O titular barrado | a felicidade do contrato (+0x3d), na decisão de verdade da 0x84af00 |
| A verba sem freio | o salto em 0x599307 |

Um exe diferente do 3.9.68 em qualquer desses pontos deixa aquela parte desligada, e o `tweaks.log` diz qual.

O mapa completo do mercado (as rotinas, os sorteios, os defeitos do original) está no projeto CM0102, `docs/MAPA_MERCADO.md`.
