# Assumir qualquer seleção (`manage_any_team`)

[English](SELECOES.en.md)

Pedido do fórum (17/11/2026): com uma seleção pequena (o exemplo era Porto Rico), não dá para fechar a convocação, porque a nação tem menos jogadores que o mínimo. A chave `manage_any_team = 1` resolve as duas pontas: deixa assumir qualquer seleção principal e dá cinzentos à seleção do técnico humano. Com `grey_signing = 1`, os cinzentos da seleção podem ser efetivados.

## 1. O que o original faz

- **Quem pode ser assumido** (`club_can_be_managed`, 0x544820, cdecl(time) → al). Usada no botão de candidatura, na contratação de técnicos, nas regens e na escolha do time no começo da carreira. Uma seleção (id entre total − 2 × nações e total − nações − 1) só passa se a nação tem mais de 150 pessoas no banco (+0x88, `NumberStaff`; o teste em 0x544859). No banco do 3.9.68, Porto Rico tem 9 pessoas e São Marino 12 [medido no `nation.dat`].
- **A escolha do time** (Setup.cpp, 0x820d40) tirava as seleções da lista (0x53b900 em 0x8226b1), embora saiba desenhá-las.
- **Os cinzentos** (`virtual_staff.cpp`): o jogo os cria antes de cada partida, para os dois times (0x90d2f0, chamada de `match_day.cpp`), e ao abrir a tela de um time (0x90d4f0, chamada de `club_screens.cpp`; cada técnico humano tem um time "aberto" por vez, e os cinzentos saem do anterior). A criação de um time (0x90dae0), quando o time é uma seleção, pergunta se ele tem técnico humano (`0x600a50(time, 1, 1, 0, 0)` em 0x90dc6d) e, se tem, sai sem criar nenhum. Por isso a seleção do humano só tinha os jogadores reais [observado: São Marino com 5 convocados e nenhum cinzento depois da carga].

## 2. O que a chave muda

- Todas as seleções principais podem ser assumidas (o desvio da entrada de 0x544820 chama a original e, quando ela recusa uma seleção do primeiro conjunto, aceita). As do segundo conjunto (a sub-21) seguem o original.
- Na escolha do time, um item **Seleções** (National teams) no fim do seletor de nação, que aparece também com uma nação só; nele, a lista mostra só as seleções. A linha de uma seleção de nação sem continente cadastrado (+0x71) caía; agora o texto fica vazio.
- A carreira automática (`auto_setup.cfg`, a chamada de 0x827d0b) segue a regra original.
- A seleção principal do técnico humano recebe cinzentos como um clube: a chamada de 0x90dc6d responde "sem técnico humano" para as seleções principais.
- **A candidatura** (1.8). O botão de candidatura passou a aparecer para as seleções pequenas, mas a vaga é outra regra: `add_job_vacancy` (0x69ba50) não abre vaga numa seleção cuja nação tem até 1.000 pessoas no banco (o `jle` em 0x69bae5), e `apply_for_job` (0x69ca00), sem achar a vaga, dava a caixa de erro de manager_manager.cpp, linha 12456 (relato do fórum de 18/11: Porto Rico). Agora o limite cai só durante a chamada da candidatura (0x69cd41): a vaga daquela seleção abre e o resto segue o original.

## 2.1. Vagas em todas as seleções (`national_team_jobs`)

O jogo chama `add_job_vacancy` de 20 lugares: o começo da carreira (`init_club_board_records`), a rotina diária dos técnicos (seis chamadas), `fix_missing_jobs`, as demissões, as aposentadorias, a troca de divisão. Em todos, a seleção de uma nação com até 1.000 pessoas fica sem vaga, e o lugar do técnico fica como está. Com `national_team_jobs = 1`, o limite cai em todas as chamadas: o jogo abre essas vagas desde o começo da carreira e a IA contrata técnicos para elas, tirando-os de clubes e de outras seleções, o que abre vagas em cadeia. Muda o mundo inteiro, com ou sem o humano numa seleção, por isso fica desligada por padrão e não entra na candidatura (que só precisa da vaga de uma seleção). Vale também para o segundo conjunto (a sub-21), que passa pela mesma regra.

## 3. Efetivar na seleção

Com `grey_signing = 1`, o item Efetivar aparece também no perfil de um cinzento da seleção do técnico humano (+0x24 do técnico). A efetivação é a mesma do clube (a pessoa, a identidade do cinzento, o CA e o PA, a personalidade e a marca de regen), com uma diferença: **ele fica sem clube e convocado**, no lugar do cinzento na lista (+0xd7), sem contrato e sem salário. O vínculo com a seleção (+0x24 a seleção, +0x28 o cargo, 0x0b para jogador, e as datas até +0x38) é copiado de um convocado real, e a convocação é marcada no +0x5e (bits 2-3: 4 principal, 8 B; `staff_set_squad_flags` 0x540980), que o jogo confere para tirar o jogador da lista (`national_teams_player_in_squad` 0x76da80). Até a 1.7.1 o efetivado ficava sem essa marca e só saía da lista depois de convocado de novo (relato do fórum de 18/11). Qualquer clube pode contratá-lo; a aposentadoria de quem fica um ano sem clube e sem seleção (0x7a8050) não o pega enquanto estiver convocado.

## 4. Os pontos no exe

| Ponto | O que faz | Troca |
|---|---|---|
| 0x544820 (`56 8B 74 24 08`) | quem pode ser assumido | desvio na entrada |
| 0x827d0b | a carreira automática | chama a original |
| 0x8226b9 | a lista tirava as seleções | `jne` some |
| 0x8227b1 | a linha da seleção sem +0x71 | trecho novo |
| 0x822698 | o filtro da lista por nação | trecho novo (o item Seleções) |
| 0x82257a | o fim do seletor de nação | o item Seleções |
| 0x822306, 0x822693 | o seletor só com mais de uma nação | com uma também |
| 0x90dc6d | a seleção do humano sem cinzentos | `humano_na_selecao` |
| 0x69bae5 (`0F 8E 3A 0A 00 00`) | `add_job_vacancy`: sem vaga numa seleção de até 1.000 pessoas | trecho novo: segue com a marca |
| 0x69cd41 | a candidatura pede a vaga | `vaga_da_candidatura` (a marca durante a chamada; com `national_team_jobs = 1`, a marca fica sempre e este ponto não muda) |

Trecho diferente do 3.9.68 em qualquer ponto da tela de escolha: a tela fica como está (a regra de quem pode ser assumido ainda vale). A entrada de 0x544820 diferente: a chave fica desligada e o log diz por quê. O `jle` de 0x822696 já apagado (o NickPatcher atual faz isso) conta como compatível: o ponto 0x822693 fica como está e o resto da tela liga ([NICKPATCHER.md](NICKPATCHER.md)).

**NickPatcher:** nenhuma opção nem patch do MiscPatches escreve nesses pontos (`tools/exe/nickpatcher_vs_tweaks.py`). A opção `manageanyteam` dele muda outro byte da mesma rotina (0x5448aa, a nação ativa de um clube com divisão) e não libera as seleções pequenas; as duas convivem, porque o desvio chama a original antes.

## 5. As provas

[TESTES.md](TESTES.md) §7: São Marino assumido na criação da carreira, os cinzentos na seleção, 19 efetivados, gravado e carregado. Falta: um jogo marcado no calendário, para ver a convocação de uma partida com os efetivados.
