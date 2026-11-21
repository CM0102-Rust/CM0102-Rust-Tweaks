# Os jogadores cinzentos: efetivar e o apelido

*English version: [EFETIVAR_CINZENTOS.en.md](EFETIVAR_CINZENTOS.en.md)*

Pedido do Lucas (16/11/2026): com todos os clubes jogáveis (a CLZ.DLL libera os clubes de estadual, que vivem de cinzentos), o técnico humano precisa poder ficar com os cinzentos do próprio elenco. Em 17/11 entrou o apelido dos cinzentos, para evitar os nomes compostos longos. Este documento descreve como ficou; as provas estão em [TESTES.md](TESTES.md) §6.

**[observado]** = lido no código ou na memória do jogo; **[inferência]** = a provar.

## 1. As decisões do Lucas

1. Só os cinzentos do **próprio clube** do técnico humano. A IA continua usando cinzentos como hoje.
2. Pelo item **"Efetivar"** no menu Action do perfil do cinzento.
3. A personalidade é **sorteada** na efetivação (o jogo dá 10 em tudo ao cinzento).
4. O efetivado recebe a **marca de regen** (bit 0x40 do +0xb do registro de 79 bytes), para os editores o acharem no filtro "Só regens".
5. **Sem negociação** (17/11): a primeira versão abria a tela de contrato do jogo, e nenhum efetivado aceitava a proposta. O efetivado chega com um **contrato normal de 12 meses** e um salário na medida do elenco.
6. Ele **fica no time em que estava**: principal ou reservas.
7. Funciona com a fornada (`intake`) ligada ou desligada.
8. Os cinzentos ganham um **apelido** de uma palavra (`grey_players_names = nickname`), em todos os clubes.

## 2. O que acontece no jogo

1. No perfil de um cinzento do seu clube aparece o botão **Action** com um item só, **Efetivar** (Make permanent). O jogo não mostra o Action para cinzentos.
   - O cinzento precisa ser do seu clube (ou da sua seleção) pelo dono que o jogo grava nele, não só estar numa vaga do seu elenco. O original às vezes deixa a mesma linha de cinzento em dois elencos (semana com jogos continentais); até a 1.8 isso mostrava o item no perfil de cinzentos da IA, e a efetivação fazia uma cópia, deixando o cinzento lá (relato do fórum de 20/11, corrigido na 1.8.1).
2. No clique, a DLL transforma o cinzento numa pessoa real, com os mesmos nome (e apelido), idade, nação, posições e atributos.
   - O CA fica pelo menos no nível do clube (o dos cinzentos, `grey_players = squad`).
   - O PA ganha espaço pela idade: até 18 anos, +30 a +50; de 19 a 21, +15 a +35; de 22 a 24, +5 a +20; depois, 0 a +8.
   - A personalidade é sorteada e ele leva a marca de regen.
3. Ele assina com o seu clube um contrato **normal** (não o mensal) de `grey_signing_months` meses (12). O salário é a mediana dos salários dos jogadores do elenco que não são regens × `grey_signing_wage` % (100), com o mínimo do jogo (55).
4. Ele fica no time em que o cinzento estava: no principal, sobe pela rotina do próprio jogo; nas reservas, fica lá.
5. A vaga do cinzento é liberada. Na partida seguinte o jogo cria outro cinzento, e a DLL garante que ele não nasça com o nome do efetivado: o sorteio do jogo é fixo por clube e vaga.

## 3. De onde vem a pessoa

O cinzento não tem linha nas tabelas por pessoa (contratos, disciplina, lesões, estatísticas) e é refeito antes de cada partida [observado, estudo de 03/11]. A efetivação usa uma pessoa real que esteja sobrando, nesta ordem:

| Ordem | Fonte | Quando existe | Custo |
|---|---|---|---|
| 1 | as **adormecidas da fornada** (aposentadas sem clube, com ficha, fora da fila: `adormecidos` em `src/regens/fornada.rs`) | carreira criada com `intake = 1` | uma adormecida a menos (sobram milhares) |
| 2 | a **cabeça da fila de regens + uma ficha da reserva** da regen | qualquer save | um regen a menos; sem ficha livre, a DLL registra e não efetiva |

A pessoa vira regen pela rotina do próprio jogo (`regenerate_as_youth_player` 0x7abac0), sem a disputa dos clubes, e depois recebe a identidade do cinzento.

Ainda não feito: um estoque próprio (`grey_signing_stock`, umas 300 pessoas criadas com a carreira) para a fornada desligada, sem custar regens.

## 4. O apelido (`grey_players_names = nickname`)

O jogo sorteia 20 nomes da nação do clube e fica com o de maior peso no banco. No CLZ, os nomes brasileiros compostos têm peso alto ("Pedro Henrique", "da Silva Santos"): 7.668 dos 9.270 sobrenomes brasileiros são compostos.

Com o apelido, depois que o jogo cria o cinzento:

1. Nome e sobrenome passam a ser de uma palavra (de 2 a 12 letras, sem ponto; até 10 sorteios cada).
2. O apelido (+0xc, que no cinzento aponta para a linha vazia 0 da tabela de apelidos) recebe uma linha de apelido da nação, de uma palavra e ainda não usada por nenhuma pessoa real do banco. O sorteio é o mesmo dos nomes: 20 sorteios na faixa da nação, fica o de maior peso.
3. O jogo mostra o apelido no lugar do nome, como faz com os jogadores reais.

Nação com menos de 20 apelidos fica só com nome e sobrenome de uma palavra. O efetivado leva o apelido, e a linha passa a contar como usada. O sorteio usa o `rand()` que o jogo semeia por clube: o cinzento da mesma vaga mantém o apelido de uma partida para a outra.

## 5. Os pontos no exe

| O quê | Onde | Como |
|---|---|---|
| O item no menu | 0x874a58 (`33 F6 56 6A 04`), na montagem do perfil 0x870700, logo depois do menu Action | para cinzento (`id ≥ [0xAE2364]`) do clube humano: o botão Action (`add_button_item` 0x804030), a área (`Display::add_area` 0x55fba0) e o item (`Display::add_item` 0x55f990) com o id 0x50 |
| O clique | `staff_screens_handle_profile_action` 0x888260 (`6A FF 68 9B 2F 96 00`) | id 0x50: efetivar; o resto vai para a original |
| O humano e o clube dele | `[0xb74c26 + 0x300 × [0xb77c7a]]`, clube em +0x39 | |
| O dono do cinzento | +0x39 o clube (0x90e33d) ou +0x24 a seleção (0x90e330), gravados pela `create_virtual_player` | tem de ser o clube ou a seleção do humano: a `relocate_virtual_players` (0x90e9f6) pode deixar a mesma linha em dois elencos |
| A pessoa vira regen | as duas chamadas de clube da 0x7abac0 (0x7acea2, a disputa; 0x7acf4f, a lista da nação) | devolvem 0 durante a efetivação |
| Pôr no clube | `player_regen_sign_for_club` 0x7a6f10 | no clube humano: reservas, contrato mensal, salário padrão |
| O contrato | `CONTRACT_MANAGER::get_contract` 0x4cf5a0 | a DLL troca o tipo para normal (+0x35, 2), o fim (+0x2d dia, +0x2f ano) e o salário (+0xc) |
| Subir ao principal | `move_to_first_team` 0x4cc0b0 | quando o cinzento estava no principal (bit 0 do +0x5e) |
| O nome do cinzento | `create_virtual_player` 0x90e0a0 (`6A FF 68 A0 55 96 00`, thiscall, `ret 0x1c`) | depois da original: o apelido e o nome que não repete o de um jogador real do elenco |
| Os sorteios de nome | 0x539c60 (primeiro nome), 0x53a1b0 (sobrenome), índice por nação `[0xae2338]` + id × 36 | apelidos: primeira linha em +8, quantos em +0x20; tabela `[0xae23f8]`, linhas de 60 bytes, nação em +0x37, peso em +0x3b |

**Armadilha:** o gerente dos contratos é `[0xae1050]`, e o vetor dos contratos é `[[0xae1050]]`. Passar `0xae1050` como `this` grava em memória errada; foi a causa do contrato que ficava mensal (16/11).

## 6. As chaves

| Chave | Padrão | O que faz |
|---|---|---|
| `grey_signing` | 0 (sem a linha) | 1 = o item Efetivar |
| `grey_signing_months` | 12 | a duração do contrato do efetivado, em meses (1 a 60) |
| `grey_signing_wage` | 100 | o salário do efetivado, em % da mediana do elenco sem os regens (10 a 300) |
| `grey_players_names` | original | `nickname` = o apelido de uma palavra dos cinzentos |

A surpresa dos cinzentos (`grey_players_surprise`, 5%, +15 a +35 de CA acima do nível) fica no [GUIA_DAS_CHAVES.md](GUIA_DAS_CHAVES.md).

Na seleção do técnico humano (com `manage_any_team = 1`), o efetivado fica sem clube e convocado, sem contrato: ver [SELECOES.md](SELECOES.md) §3.

## 7. As provas e o que falta

Provado (TESTES §6):

- a efetivação na tela, no CLZ v6;
- gravado e carregado: um do principal e um das reservas no CLZ (fornada desligada, a pessoa da fila de regens) e um das reservas no 3.9.68 com a fornada ligada (a pessoa das adormecidas);
- o apelido numa carreira escondida no 3.9.68 e no CLZ.

Falta: o estoque próprio para a fornada desligada.
