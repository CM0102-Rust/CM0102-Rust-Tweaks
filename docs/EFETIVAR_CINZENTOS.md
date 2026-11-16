# Efetivar um jogador cinzento: o desenho

*English version: [EFETIVAR_CINZENTOS.en.md](EFETIVAR_CINZENTOS.en.md)*

Pedido do Lucas (16/11/2026): com todos os clubes jogáveis (a CLZ.DLL libera os clubes de estadual, que vivem de cinzentos), o técnico humano precisa poder ficar com os cinzentos do próprio elenco. Este documento é o desenho, antes do código. Parte do estudo de 03/11 do projeto CM0102 (`docs/EFETIVAR_CINZENTOS.md` de lá) e de dois estudos do exe de 16/11 (o menu de ações do perfil e a negociação de contrato).

**[observado]** = lido no código; **[inferência]** = a provar.

## 1. As decisões do Lucas

1. Só os cinzentos do **próprio clube** do técnico humano. A IA continua usando cinzentos como hoje.
2. Pelo item **"Efetivar"** no menu Action do perfil do cinzento.
3. A personalidade é **sorteada** na efetivação (o jogo dá 10 em tudo ao cinzento).
4. O efetivado recebe a **marca de regen** (bit 0x40 do +0xb do registro de 79 bytes), para os editores o acharem no filtro "Só regens".
5. O contrato é **negociado** na tela do próprio jogo, numa negociação **fácil**: o jogador fica seduzido pelo papel e nunca pede muito.
6. Funciona com a fornada (`intake`) ligada ou desligada.

## 2. O que acontece no jogo

1. No perfil de um cinzento do seu clube aparece o botão **Action** com um item só, **Efetivar** (Make permanent). Hoje o jogo não mostra o Action para cinzentos.
2. No clique, a DLL transforma o cinzento numa pessoa real: os mesmos nome, idade, nação, posições e atributos, a personalidade sorteada e a marca de regen. Ele entra no seu elenco com um **contrato provisório** curto e o perfil reabre já como jogador comum.
3. Logo em seguida abre a tela do jogo **Set Role At Club** e, depois, a de oferta de contrato. Você escolhe o papel e oferece.
4. Para o efetivado, o pedido é o **salário padrão do jogo para aquele papel** × `grey_signing_wage` (%), sem luvas, bônus nem cláusulas; ele nunca se recusa a conversar, e uma oferta que cubra o pedido é sempre aceita. A resposta vem no dia seguinte (no jogo, de 1 a 14 dias).
5. Se você não renovar, o contrato provisório acaba e ele sai do clube como qualquer jogador.

## 3. De onde vem a pessoa

O cinzento não tem linha nas tabelas por pessoa (contratos, disciplina, lesões, estatísticas) e é refeito antes de cada partida [observado, estudo de 03/11]. A efetivação usa uma pessoa real que esteja sobrando, nesta ordem:

| Ordem | Fonte | Quando existe | Custo |
|---|---|---|---|
| 1 | as **adormecidas da fornada** (aposentadas sem clube, com ficha, fora da fila: `adormecidos` em `src/regens/fornada.rs`) | carreira criada com `intake = 1` | uma adormecida a menos (sobram milhares) |
| 2 | o **estoque dos cinzentos**: `grey_signing_stock` pessoas criadas com a carreira, pelo mesmo mecanismo da fornada | carreira nova criada com `intake = 0` | nenhum |
| 3 | a **vaga da fila de regens + uma ficha da reserva** da regen (o caminho da regen) | qualquer save | um regen a menos; sem ficha livre, a DLL avisa e não efetiva |

## 4. Os pontos no exe

| O quê | Onde | Como |
|---|---|---|
| O item no menu | 0x874a58 (`33 F6 56 6A 04`), na montagem do perfil 0x870700, logo depois do menu Action | para cinzento (`id ≥ [0xAE2364]`) do clube humano: o botão Action (`add_button_item` 0x804030), a área (`Display::add_area` 0x55fba0) e o item (`Display::add_item` 0x55f990) com o id 0x50, livre [observado] |
| O clique | `staff_screens_handle_profile_action` 0x888260 (`6A FF 68 9B 2F 96 00`) | id 0x50: efetivar, reabrir o perfil (0x8702a0) e abrir o contrato; o resto vai para o original [observado] |
| O humano e o clube dele | `[0xb74c26 + 0x300 × [0xb77c7a]]`, clube em +0x39 | [observado] |
| Pôr no clube | `player_regen_sign_for_club` 0x7a6f10 | o contrato do regen (no clube humano: reservas, situação 6); a DLL encurta o fim (+0x2d/+0x2f) [observado] |
| Abrir o contrato | `contract_screens_offer_contract_to_staff` 0x4dd0f0 `(staff, 0, 0, 1)` | abre a tela Set Role e depois a oferta [observado] |
| O pedido | `STAFF_CONTRACT::set_to_desired_contract` 0x85fc50 (`6A FF 68 76 2A 96 00`), depois do original | só para os efetivados: salário = `default_staff_contract` 0x864550 do papel × `grey_signing_wage`; luvas, bônus e cláusulas zerados [observado o ponto; o efeito, inferência] |
| Sempre conversar | `CONTRACT_MANAGER::will_negotiate_offer` 0x4d4660 (`83 EC 10 53 8B 5C 24 18`) | devolve 1 para os efetivados [observado] |
| O aceite | `resolve_contract_offers` 0x4ce4e0 | a oferta que cobre o pedido tem razão ≥ 0,9 e é aceita (3,15 × 0,9 > 2,75): nada a desviar [observado] |
| A resposta no dia seguinte | `schedule_offer_decision` 0x4cd230 | opcional: a data da decisão = amanhã para os efetivados [observado o ponto] |

Todos conferem byte a byte no exe do CLZ 6.1 (NickPatcher) [observado no arquivo; a NickDLL pode mudar em memória: a conferência de desvios da DLL registra].

## 5. As chaves

| Chave | Padrão | O que faz |
|---|---|---|
| `grey_signing` | 0 (sem a linha) | 1 = o item Efetivar e a negociação fácil |
| `grey_signing_wage` | 80 | o pedido do efetivado, em % do salário padrão do papel |
| `grey_signing_days` | 30 | a duração do contrato provisório, em dias |
| `grey_signing_stock` | 300 | as pessoas criadas com a carreira para isso, quando a fornada está desligada |

## 6. A conferir antes e durante o código

- A limpeza da pessoa reaproveitada: a regen faz pela `regenerate_as_youth_player` 0x7abac0, que também escolhe o clube pela disputa; a efetivação precisa das mesmas limpezas, sem a disputa.
- A lista dos efetivados que ainda negociam precisa sobreviver ao save (um arquivo ao lado do save, como o `.publico`, ou uma marca no contrato provisório).
- Se `create_virtual_player` grava o clube em +0x39 do cinzento.
- A tela Set Role pode recusar papéis pela nota da diretoria.
- As provas: efetivar, negociar, gravar, carregar e conferir com o editor; com a fornada ligada e desligada; no 3.9.68 e no CLZ.
