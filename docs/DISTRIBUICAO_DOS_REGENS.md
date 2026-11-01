# A distribuição dos regens

*English version: [DISTRIBUICAO_DOS_REGENS.en.md](DISTRIBUICAO_DOS_REGENS.en.md)*

Como o CM 01/02 3.9.68 cria os regens jogadores e escolhe o clube de cada um. Feito a pedido da thread "[RELEASED] 4 Playable Polish Divisions" no champman0102.co.uk (perguntas do Here2Win79 e do alex91). Fontes: as rotinas já reescritas em `rust/src/motor.rs` (comparadas com a original sem divergência), a desmontagem do `cm0102.exe` e três saves. Marcação: **[observado]** = lido no código ou contado no save; **[inferência]** = conclusão minha.

Complementa [MARCA_DO_REGEN.md](MARCA_DO_REGEN.md) (§4 e §8: a fila, a reserva de fichas, o que o regen herda).

## 1. Resumo

- **Não existe "safra de juniores" por clube.** Um regen jogador nasce para cada ficha de jogador que se aposenta, e então os clubes da nação dele disputam o jovem [observado].
- **Quantos por temporada:** o mesmo número de jogadores que se aposentam (ou viram técnicos) no banco de dados inteiro. O potencial (PA) é o do jogador aposentado. Então o número de jovens de PA alto numa nação, por temporada, é o número de jogadores de PA alto daquela nação que se aposentam [observado].
- **Como o clube é escolhido:** cada clube elegível ganha uma nota, o jogo sorteia um número entre 0 e a nota de cada um e o maior sorteio leva o jovem [observado].
- **Por que os grandes da IA levam tantos:** três termos da nota. Training 20 soma 7000 + reputação/2 com certeza; com Training 19, só 45% das vezes. O clube da IA com técnico bom e Training alto tem a nota levantada até a própria reputação. E o elenco abaixo de 40 soma mais 1000. Nada limita o número de jovens por clube além do elenco de 50 [observado].
- **O clube humano** não usa o técnico (o humano não tem atributos). Recebe um bônus fixo pequeno e, com Training 20, o mesmo piso da reputação, só para PA acima de 120 [observado].

## 2. Quando o jogo cria regens

| Passo | Rotina | O que faz |
|---|---|---|
| Busca de aposentadorias | `find_new_retirements` 0x7a5790, nos dias 0, 80, 173 e 250 do ano (`is_player_regen_day` 0x5d6250) | Cada pessoa passa pela regra de idade (`player_regen_check_retirement` 0x7a7310). Ficam de fora quem tem menos de 30 anos e joga num clube de competição que roda no jogo, os 16 técnicos humanos e quem já está na lista [observado] |
| Aposentadoria | `process_retirements` 0x7a59e0, todo dia | O jogador que se aposenta entra na **fila** (`queue_staff_for_regen` 0x7a87d0) e a ficha dele (`player.dat`) entra na **reserva de fichas**. Quem vira técnico só deixa a ficha. A data na fila é hoje (menos de 30 anos) ou hoje + 1 a 65 dias [observado] |
| Nascimento | `process_regen_queue` 0x7a65d0, todo dia | Enquanto a cabeça da fila estiver vencida **e** houver ficha na reserva: `regenerate_as_youth_player` 0x7abac0 na vaga da cabeça, com a primeira ficha da reserva [observado] |

A reserva de fichas é o gargalo. Os não jogadores também entram na fila, mas sem ficha. Logo, **regens jogadores por temporada = fichas de jogador liberadas por temporada** [observado; ver MARCA_DO_REGEN §8].

Contagens nos saves [observado]:

| Save | Duração | Regens jogadores vivos |
|---|---|---|
| `TesteTelas.sav` (banco de fábrica, ligas do Lucas) | ~7 meses (jul/2001 a fev/2002) | 793 |
| `Valencia.sav` (CLZ) | 2025 a 2028 | 3.857 |
| `All Leagues - 3.9.68 - Compress.sav` (todas as ligas) | 2001 a 2063 (62 temporadas) | 127.863 de 158.790 pessoas |

No save de todas as ligas, isso dá pelo menos ~2.060 regens por temporada que continuam vivos no fim. É um piso, porque regens também se aposentam e viram regens de novo [cálculo].

## 3. O jovem: idade, habilidade, nação

Pela `regenerate_as_youth_player` 0x7abac0 [observado]:

- **PA (+7 da ficha):** não muda. É o do jogador aposentado (B, o dono da ficha). As posições, os pés e boa parte dos atributos também vêm dele.
- **Nação:** a de B. Se a reputação da nação (+0x88) for menor que 100, há 1 chance em 4 de trocar por outra nação sorteada, também de reputação menor que 100 (0x7abcd7).
- **Idade** (nação no jogo ou reputação ≥ 350; 0x7ac58e): sorteio(10000):
  - = 0: 15 a 18 anos;
  - < 5000: 16 a 18;
  - < 9976: 17 a 20;
  - senão: 21 a 26.
  - Resultado: **~75% dos regens têm menos de 19 anos**.
  - Numa nação fora do jogo de reputação < 350, a idade vai de 17 a 27.
- **CA (+5)**, pela posição (0x7ac3dd):

| Posição | CA |
|---|---|
| atacante | 0,3·PA + sorteio(0,6·PA) |
| defesa ou líbero | 0,25·PA + sorteio(0,55·PA) |
| goleiro | 0,25·PA + sorteio(0,5·PA) |
| outras | 0,25·PA + sorteio(0,6·PA) |

  Depois vem o teto pela idade, em 249 de 250 vezes (ou 149 de 150): 80 até 17 anos; 100 aos 18; 90/110 no ramo de 17 a 20; nenhum de 21 em diante.
- **Reputação:**
  - local (+9) = 250 + sorteio(750) + 20·CA;
  - mundial (+0xd) = 1 + sorteio(20) + 5·CA;
  - atual (+0xb) = local + sorteio(50).

## 4. Qual caminho até um clube

Fim da 0x7abac0, 0x7ace65..0x7acf5c [observado]:

| Situação | Caminho |
|---|---|
| Menos de 19 anos, nação no jogo (+0x11c), 3 vezes em 4 | **A disputa da nação**: `pick_best_club_for_player` 0x7ad0e0 |
| Nação no jogo, nos outros casos | 1 vez em 10, a lista da nação (0x7ad590); 9 vezes em 10, fica **sem clube** |
| Nação fora do jogo | 14 em 15, a lista da nação; senão 1 em 10 |

- A lista da nação (`build_nation_club_lists` 0x7acfc0) só aceita clube de nação no jogo com reputação ≤ 2 (0x7ad075). Na prática, **nas nações jogáveis ela fica vazia**, e o jovem desse caminho também fica sem clube [observado; que ninguém tenha reputação ≤ 2 é inferência pelos bancos usuais].
- Nas nações jogáveis, ~56% dos regens (0,75 × 0,75) passam pela disputa; os outros começam sem clube e dependem do mercado [cálculo].
- Sem clube e com menos de 20 anos: as reputações atual e local ficam limitadas a 1250, e a mundial vai a 1 (0x7acf6b).
- Saves [observado]:
  - `TesteTelas`: 280 de 793 sem clube;
  - `Valencia`: 214 de 3.857 (os outros já assinaram depois).

## 5. A disputa da nação: `pick_best_club_for_player` 0x7ad0e0

Entra todo clube da tabela (sem as seleções), mesmo de liga fora do jogo, se [observado]:

1. o jovem **não é estrangeiro** nele (0x53d980 com grupos: a nação dele, a segunda nação, e os grupos das Ilhas Britânicas e da Irlanda, de Austrália e [0x9cf3f4] e outros). Na prática: **só clubes da nação dele**;
2. não é o time B de um par (0x540a50);
3. não é vetado pela regra basca;
4. pode chegar ao jogador (`can_club_bid_for_player` 0x8ce990);
5. tem **menos de 50 no elenco** (`database_count_club_squad` 0x53c590; o `cmp bl, 0x32` em **0x7ad1bd**).

Cada um recebe `sorteio(nota)` (0x7ad220), número de 0 a nota − 1. **Ganha o maior** (começa em −25000; empate fica com o primeiro). Nenhum clube elegível: o jovem fica sem clube.

## 6. A nota do clube: 0x7ad220

Notação: R = reputação do clube (+0x80, 0..10000). Tr = Training (+0x7f, 1..20; o que os jogadores chamam de "instalações"). PA, CA = do jovem. rep = reputação atual do jovem (+0xb).

**1. Base** [observado]:

    nota = 5000 + R − |rep − R| − |50·PA − R| − |50·CA − R|

- É uma linha quebrada em R. Sobe 4 por ponto abaixo do menor dos três pontos (rep, 50·CA, 50·PA). Sobe 2 entre o menor e o do meio. É **plana entre o do meio e o maior**. **Cai 2 por ponto acima do maior.**
- O topo vale 5000 + menor + meio − maior [cálculo].
- Exemplo, PA 170, CA 80, rep 2250 (pontos 2250, 4000, 8500): topo de 2750 para R entre 4000 e 8500; R = 9500 dá 750.
- Sozinha, a base **prefere clubes de reputação próxima do nível do jovem** e castiga os grandes demais.

**2. Training** [observado]. Vale se PA > sorteio(20) + 100: sempre com PA ≥ 120, nunca com PA ≤ 100.

| Tr | Chance | Bônus |
|---|---|---|
| 20 | sempre | R/2 + 7000 |
| 1 a 19 | sorteio(Tr) > sorteio(20), ou seja (Tr − 1)/40: 45% com 19, 22,5% com 10, 0 com 1 | R/2 + 5000 + 50·Tr |

O degrau entre 19 e 20 é enorme [cálculo].

**3. Competição ativa e PA > 50** [observado]. O clube joga uma competição que roda no jogo (`[[0xadadfc] + 4·id]`).
- Só o clube humano, com PA > 120: +500 ou +250 pelo byte +4 do estado do clube na temporada (`[0xae2d14]` + 6·id), negativo ou zero [significado do byte não confirmado].
- Todos, pelo elenco:

| Reputação | Elenco < 35 | < 40 | < 45 | resto |
|---|---|---|---|---|
| R > 5750 | +1000 | +1000 | +750 | +250 |
| 4250 < R ≤ 5750 | +750 | +500 | +250 | +250 |
| R ≤ 4250 | +250 | +250 | +250 | +250 |

**4. O técnico principal** (+0xcf → ficha de não jogador +0x69). Não é o treinador de juniores [observado].
- **Clube da IA:**
  - + (Youngsters − 10) × 30 (Youngsters = +0x22);
  - com PA > sorteio(75) + 100: + (v − 10) × 60, em que v = f(CA do técnico + 2·JudgingPotential) é a `chance_to_know` 0x544410: x = (CA + 2·JP)·0,05 + 10, v = (x + 0,1x²)/3 + 0,5, de 1 a 20;
  - **o piso:** se v + Youngsters + Tr > 40, R > 4250 e PA > 80, a nota sobe até R (se já passou de R, +250).
- **Clube humano:** +200. Com PA > 120: +25·Tr, e com Tr = 20 o mesmo piso de R.

**5. Nação** [observado]:
- mesma nação do clube: +1000;
- clubes galeses ([0x9d0430], [0x9d0434], [0x9d043c]): jovem galês +1000, inglês −750;
- clube escocês: jovem galês ou inglês −750, escocês +1000.

**6. 16 bits.** Só a palavra baixa da nota vai ao sorteio. Com R ≤ 10000, a soma de todos os termos fica abaixo de ~31.700 (base ≤ 15.000, Training ≤ 12.000, competição ≤ 1.500, técnico ≤ 1.150, nação ≤ 2.000). Então **o corte em 16 bits não inverte a escolha** nos bancos normais. Só com reputações acima da escala (não validadas pelo editor) a nota passaria de 32.767 e viraria negativa, e aquele clube nunca ganharia [cálculo].

## 7. Por que os grandes da IA levam tantos

Simulação da fórmula acima ([inferência]: clubes inventados, 20.000 sorteios; script em `regens/tools/simula_nota.py`):
- grande da IA: R 9000, Tr 20, técnico bom;
- médio da IA: R 6500, Tr 14, técnico médio;
- humano: R 6500, Tr 20;
- 2ª divisão da IA: R 4000, Tr 10;
- 40 clubes pequenos: R 1500, Tr 3.

| Jovem | Grande da IA (9000) | Humano (6500) | Médio da IA (6500) | 2ª divisão (4000) | 40 pequenos |
|---|---|---|---|---|---|
| PA 170, CA 80 | 44% | 42% | 9% | 4% | 1% |
| PA 130, CA 65 | 27% | 51% | 11% | 5% | 5% |
| PA 90, CA 45 | 59% | 0% | 0% | 15% | 26% |
| PA 170, sem o humano | 68% | n/d | 18% | 10% | 3% |

O que explica [observado na fórmula]:
1. **O piso da IA.** Com técnico razoável e Training alto, a nota do grande fica pelo menos igual à reputação dele (9000 no exemplo). Isso apaga a penalidade da base para clube "grande demais" e vale **para qualquer PA acima de 80**. Por isso o grande da IA leva também os jovens medianos (a linha PA 90), que o humano e o clube médio não alcançam.
2. **Training 20** soma 11.500 ao clube de R 9000, sempre. Com 19, só 45% das vezes.
3. **Nenhum limite por clube.** A nota é recalculada para cada jovem, e o mesmo clube ganha de novo até o elenco chegar a 50. O único freio é o bônus de elenco, que cai de +1000 para +250 a partir de 45.

Nos saves [observado; regens no clube **atual**, que pode já não ser o da criação; reputação e Training andam juntos]:

| Reputação do clube | `TesteTelas` (7 meses) | `Valencia` (3 temporadas) | todas as ligas (62 temporadas), PA ≥ 150 por clube |
|---|---|---|---|
| < 1000 | 0,00 | 0,11 | 0,007 |
| 2000–2999 | 0,11 | 0,75 | 0,11 |
| 4000–4999 | 0,16 | 1,69 | 1,15 |
| 6000–6999 | 0,22 | 3,26 | 3,29 |
| 8000–8999 | **1,68** | 3,75 | 6,19 |
| 9000+ | 0,11 | 3,80 | 5,54 |

Regens por clube pelo Training, no `Valencia`: Tr 1 = 0,09, Tr 10 = 0,96, Tr 16 = 2,62, **Tr 19 = 2,40, Tr 20 = 5,81**. O salto de 19 para 20 é o degrau do bônus garantido [observado no save; a causa é inferência pela fórmula].

No `TesteTelas`, Palmeiras, São Paulo, Vasco (50 no elenco), Santos (46) e Cruzeiro (48) receberam de 5 a 8 regens cada em 7 meses. São os elencos cheios que o alex91 descreve. Os 9000+ quase não receberam: são europeus, de nações que liberaram poucas fichas tão cedo [inferência].

## 8. O clube humano

Diferenças [observado]:
- O técnico humano não entra na nota. No lugar: +200; com PA > 120, +25·Tr (até +500) e o piso da reputação **só com Tr = 20**.
- Com PA > 120 e competição ativa: +250/+500 pelo byte +4 do estado do clube.
- O resto é igual à IA (base, Training, elenco, nação).
- Quando assina: os reservas (0x4cc0f0), os bits de contrato +0x4f/+0x35 e a notícia de promoção de juvenil (0x7a8ba0). Na IA, o contrato fica limitado a hoje + 5 anos.

Para quem espera "10 juniores por ano com instalações 20": **o clube não gera jovens**. O número esperado é a soma, sobre os jovens da nação que entram na disputa, da chance de ganhar cada um. Essa chance depende dos outros clubes da nação, e com Training 20 um clube humano de reputação média fica competitivo pelos de PA > 120. Para os de PA ≤ 120, o humano não tem o piso, e o grande da IA com técnico bom leva [inferência pelos números da §7].

## 9. O ajuste do xeno / Saturn ("Regen Tweak")

- **0x7ad1bd** `cmp bl, 0x32` → `0x2d`: na disputa da nação (§5), só entra clube com elenco abaixo de **45** (o original é 50). Mexe no único freio que existe. **O crédito é do xeno e do Saturn.**
- **0x7a6f40** `cmp eax, 0x32`, **0x7a6f45** `cmp eax, 0x32` e **0x7a6f48** `jne` ficam em `player_regen_sign_for_club` 0x7a6f10: a busca da primeira vaga livre nas 50 vagas do elenco (+0xd7). A versão completa do Saturn põe 45 nos dois `cmp` e troca o `jne` por `jl`: só assina se houver vaga livre entre as 45 primeiras. O patch do petermf23 voltou esses três ao original e manteve só a 0x7ad1bd [observado nos bytes].
- Efeito prático da diferença [inferência]:
  - A disputa já descarta quem tem 45 ou mais. A trava na assinatura só pesa onde a contagem e as vagas divergem: time A/B (a contagem 0x53c590 soma o parceiro), ou vagas ocupadas por quem não conta.
  - E no outro caminho, a lista da nação (0x7ad590), que tem o próprio limite de 50 em **0x7ad6aa** (`cmp al, 0x32`) e só funciona nas nações fora do jogo. Com o ajuste completo, esses clubes param em 45; com o do petermf23, vão até 50.

## 10. Endereços

| Endereço | Nome (mapa da comunidade) | Papel |
|---|---|---|
| 0x5d6250 | `is_player_regen_day` | dias 0, 80, 173, 250 |
| 0x7a5790 | `find_new_retirements` | busca de aposentadorias |
| 0x7a7310 | `player_regen_check_retirement` | regra de idade |
| 0x7a59e0 | `process_retirements` | fila e reserva de fichas |
| 0x7a87d0 | `queue_staff_for_regen` | entra na fila |
| 0x7a65d0 | `process_regen_queue` | nascimento diário |
| 0x7abac0 | `regenerate_as_youth_player` | idade, CA, reputação, caminho (0x7ace65..0x7acf5c) |
| 0x7ad0e0 | `pick_best_club_for_player` | a disputa da nação; elenco < 50 em 0x7ad1bd |
| 0x7ad220 | (nota do clube) | a fórmula da §6 |
| 0x7ad590 | `pick_club_from_nation_list` | lista da nação; elenco < 50 em 0x7ad6aa |
| 0x7acfc0 | `build_nation_club_lists` | reputação ≤ 2 nas nações jogáveis (0x7ad075) |
| 0x7a6f10 | `player_regen_sign_for_club` | vaga livre (0x7a6f40..0x7a6f48), contrato |
| 0x544410 | `chance_to_know` | v do técnico |
| 0x53d980 | `database_is_staff_foreign` | filtro da nação |
| 0x90cfc0 | `aleatorio` | sorteio(n) = 0..n−1 |

Campos do clube: +0x7f Training, +0x80 Reputation, +0xcf Manager, +0xd7 Squad[50]. Ficha do técnico: +4 CA, +0x18 JudgingPotential, +0x22 Youngsters. Ficha do jogador: +5 CA, +7 PA, +9/+0xb/+0xd reputações.
