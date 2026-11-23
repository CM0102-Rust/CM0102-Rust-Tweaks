# Propostas de emprego ao técnico humano

*English version: [OFERTAS_AO_TECNICO.en.md](OFERTAS_AO_TECNICO.en.md)*

Pedido do fórum (22/11/2026): "se você vai longe numa competição continental ou ganha títulos, a sua reputação sobe e clubes maiores, também do exterior, deveriam se interessar; subi de divisão várias vezes e nunca recebi proposta". Este documento descreve como o jogo escolhe técnico para uma vaga, o que barra o técnico humano e as medições feitas. **Estudo; nenhuma chave ainda.**

## 1. Como o jogo oferece emprego

Só há um caminho para uma proposta chegar ao humano:

1. Uma vaga abre (`add_job_vacancy` 0x69ba50): o jogo percorre todas as pessoas, inclusive os 16 lugares de humano no fim da tabela, aplica filtros e fica com a maior nota de `score_staff_for_job` 0x69d1b0.
2. Na data marcada, o `daily_update` 0x68f9c0 nomeia o escolhido; se ele é humano, em vez de nomear, manda a proposta (`post_job_offer_to_human_news` 0x6a4f80, chamada só em 0x68fc68).

Não há regra de "assediar o humano": ele recebe proposta só quando vence a disputa da vaga contra todos os técnicos da IA. A candidatura (`apply_for_job` 0x69ca00) passa pela mesma nota.

## 2. O que barra o humano

Na ordem em que o laço de 0x69ba50 testa:

| Regra | Onde | O que exige |
|---|---|---|
| Liga em andamento | `club_can_be_managed` 0x544820 (só para os humanos) | o clube estar numa liga carregada; sem as ligas do exterior ligadas, nunca há proposta de fora |
| Estrangeiro | 0x69ba50 | clube de outra nação: reputação em casa do técnico (+0xa do registro +0x69) acima de 3.749 |
| Outra região | 0x69ba50 (o byte +0x75 da nação) | região diferente: reputação mundial (+0xc) acima de 4.249 **e** clube acima de 4.749 |
| Adequação | `is_staff_suitable_for_job` 0x69f810 | para quem está empregado: o clube novo numa divisão de reputação maior, ou na mesma com 1.250 a mais de reputação, e o interesse (abaixo) de pelo menos 3,5 |
| Interesse | `staff_interest_in_job_at_club` 0x844780 | **0 nos primeiros `(+0x59 + 8) × 15` dias no cargo** (420 dias com 20), para qualquer técnico; 0 em clube abaixo de 2.750 da mesma nação |
| Nota | `score_staff_for_job` 0x69d1b0 | a maior entre os candidatos; **para o humano a parte de atributos e personalidade é pulada** (`id ≥ N − 16`): sobra a reputação vista pelo clube, mais 100 se ele já for do clube |

A reputação do técnico muda só pela nota média das partidas (`update_manager_reputation_after_match` 0x6d3610), com teto pela reputação da competição: nas competições nacionais o teto da mundial cai à metade (no mínimo 3.000); só as continentais a deixam passar disso. Título e acesso não entram na conta.

## 3. Medições

Estudo `TWEAKS_ESTUDO=1` (`src/estudo.rs`, `estudo_vagas.tsv`): a cada vaga de clube, uma linha por humano com os filtros, a adequação, o interesse (com os bits de motivo), a nota dele e a do vencedor. `TWEAKS_REP_TECNICO=a,c,m` força as três reputações do humano; `TWEAKS_BONUS_TECNICO=N` soma N à nota dele no laço. Carreiras automáticas escondidas no 3.9.68, ligas ENG BRA ITA, com o `tweaks.cfg` do pacote; o técnico de testes do jogo muda de nação e clube a cada carreira.

| Carreira | Vagas | Barrado (estrangeiro / região / adequação / nota) | Vence | Nota do humano × vencedor (mediana) |
|---|---|---|---|---|
| Reputação real, 1 temporada (América-RJ, Série C) | 82 | 36 / 0 / 43 / 3 | 0 | 3.389 × 7.005 |
| Reputação 8.000, 1 temporada (Cagliari) | 82 | 0 / 38 / 43 / 1 | 0 | 4.356 × 7.547 |
| Reputação real, 2 temporadas (Catania, subiu da C à B) | 158 | 117 / 0 / 21 / 20 | 0 | 641 × 7.678 |
| Reputação 8.000, 2 temporadas (Botafogo, depois Guarani) | 177 | 0 / 64 / 102 / 9 | **2** | 3.641 × 8.730 |

- **No primeiro ano no cargo o interesse é 0** e nenhuma vaga é possível, por regra do jogo que vale para todos.
- **Depois disso, o humano passa os filtros da própria nação e perde na nota.** Com a reputação real (Catania, 2.400 a 3.200), as notas dele ficam entre 2.000 e 3.500 contra 5.000 a 14.000 dos vencedores (Palermo, Sampdoria, Gênova, Ancona).
- **Com reputação 8.000 a nota chega perto** (10.000 a 10.800 contra 11.400 a 12.000) e, desempregado, ele vence duas vagas (Guarani e Botafogo, 2003); a do Guarani virou proposta e emprego.
- **A reputação do humano não sobe com o sucesso:** no Catania, com o acesso da Série C para a B, ela caiu de 3.177 para 2.709 (a em casa, de 3.177 para 2.459).

## 4. Possibilidades

| | O quê | Efeito no mundo |
|---|---|---|
| A | Este estudo (feito) | nenhum |
| B | Bônus na nota do humano, compensando a parte de atributos pulada | só nas vagas em que o humano já passa os filtros |
| C | Afrouxar os filtros de estrangeiro e região só para os humanos | idem |
| D | A reputação do técnico crescer com o sucesso (acesso, título, campanha continental) | maior: a reputação entra em tudo |
| E | Proposta em paralelo quando o humano estiver entre os melhores | médio: a vaga demora a fechar |

### 4.1 O experimento do bônus (opção B)

`TWEAKS_BONUS_TECNICO` soma o bônus à nota do humano no laço da vaga; duas carreiras de duas temporadas com a reputação real:

| Bônus | Vagas | Barrado (estrangeiro / região / adequação / nota) | Vence | O que aconteceu |
|---|---|---|---|---|
| 3.000 | 155 | 84 / 0 / 54 / 11 | **6** | demitido duas vezes (Volta Redonda, Mirassol); desempregado, venceu 6 vagas de clubes pequenos e foi contratado pelo Mirassol e pelo Sorocaba |
| 6.000 | 160 | 0 / 52 / 108 / 0 | 0 | no Palmeiras a carreira inteira: nada maior para onde subir, e as vagas de fora barradas pela região |

O bônus funciona onde o humano já passa os filtros: na prática, quando está desempregado. **Empregado, a trava é a adequação**, que vem antes da nota: o primeiro ano no cargo (interesse 0) e a exigência de subir de divisão ou de clube.

### 4.2 Um detalhe do humano

Os humanos têm o +0x59 em 20 (o máximo): a regra do tempo no cargo pede `(20 + 8) × 15 = 420` dias, o mais longo possível. Para um técnico da IA com 10, são 270. O jogo trata o humano como o técnico mais leal que existe.

O experimento `TWEAKS_LEALDADE_TECNICO=10` (o +0x59 dos humanos em 10), duas carreiras de duas temporadas com a reputação real:

| Lealdade | Bônus | Vagas | Barrado (estrangeiro / região / adequação / nota) | Vence | O que aconteceu |
|---|---|---|---|---|---|
| 10 | 0 | 169 | 0 / 128 / 34 / 7 | 0 | no Stoke City (Championship), passou a adequação de Newcastle, Wolverhampton, Crystal Palace, Chelsea e West Bromwich (todos da Premier) e perdeu na nota: 5.400 a 6.900 contra 10.500 a 15.600 |
| 10 | 3.000 | 153 | 114 / 0 / 29 / 9 | 1 | no Prato (Série C), passou para Milan, Bolonha e Reggiana e perdeu na nota (6.000 contra 6.700 a 13.000); desempregado, venceu a vaga da Ternana e foi contratado |

**Com a lealdade em 10 a janela abre**: o humano empregado passa a ser considerado pelos clubes maiores da própria nação. A trava seguinte é a nota, com uma diferença de 4.000 a 9.000 pontos para os vencedores nos clubes grandes; um bônus de 3.000 não chega lá.

### 4.3 Recomendação

O relato pede três coisas que hoje o jogo não faz, e cada uma tem uma causa:

1. **"Ganhei e ninguém me quis"**: a reputação do técnico não sobe com acesso nem com título (§2), e a do humano chega a cair. A opção D ataca a causa: uma chave para a reputação do técnico humano crescer com o sucesso (acesso, título, posição acima da expectativa da diretoria, fase continental), pelo mesmo desvio de `update_manager_reputation_after_match` 0x6d3610. Com reputação maior, os filtros de estrangeiro e região passam e a nota sobe, sem mexer nas regras de escolha.
2. **"Nunca recebi proposta enquanto empregado"**: a adequação. Uma chave que trate o humano como um técnico de lealdade média (o +0x59 do humano em 10: 270 dias no cargo) é a menor mudança que abre a janela, e mexe só nos 16 lugares de humano.
3. **A nota do humano**, que não tem a parte de atributos: o bônus (B) completa as duas acima; sozinho, só ajuda o desempregado.

Antes da 2, falta conferir onde mais o jogo lê o +0x59 de um técnico (a adequação de 0x69f810 também o usa, `(+0x59 + 21) × 10` dias), para não mudar outra coisa do humano junto; a alternativa é trocar só a conta do tempo no cargo dentro da 0x844780 quando a pessoa é humana.

Ordem sugerida: 2 (efeito só no humano), depois 1 (a mudança grande, com teste longo), e o bônus só se as duas não bastarem. A IA continua escolhendo pelas mesmas regras; o humano passa a ser candidato de verdade.

Pelas medições, a 2 sozinha abre a porta mas não dá proposta: a diferença de nota nos clubes grandes (4.000 a 9.000) pede a 1, a reputação que cresce com o sucesso, ou um bônus da ordem de 5.000 a 8.000, que seria um número arbitrário. A 1 é a resposta que o relato pede.
