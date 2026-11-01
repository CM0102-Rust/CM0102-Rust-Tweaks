# Oportunidades: o que a comunidade reclama e pede

*English version: [OPORTUNIDADES.en.md](OPORTUNIDADES.en.md)*

Leitura do fórum champman0102.co.uk em 01/11/2026, por script, só páginas públicas e sem login. A ideia é não repetir o que o NickPatcher e os outros autores já fazem, e atacar o que está em aberto e que só a nossa base em Rust permite.

**Escopo (decisão do Lucas, 01/11/2026):** este repositório é a frente da DLL. As correções e o que se aprende aqui vão ser consolidados no projeto CM0102 quando ele estiver 100% em Rust. **Estruturas de competições ficam de fora:** o davdav já faz isso, e bem (o patch de reestruturação). Na tabela "Onde temos vantagem", as frentes escolhidas são a queda de dezembro (estudo), os empréstimos e a economia.

## Método

- **10 seções:** Technical Support, Patches, The Workshop, Dressing Room, Databases, Tactics, Talents/Tips/Training, Research e Archives. Ao todo, 1.330 tópicos.
- **944 tópicos lidos** (a primeira página; nos megatópicos, também as duas últimas): fora de Tactics, os que tinham discussão ou título com sinal de pedido ou reclamação; em Tactics, os muito discutidos e os que falam do motor de partidas.
- **Temas por palavra-chave**, pesados pelo engajamento (respostas e visualizações), e a leitura dos trechos com cara de pedido ("is it possible", "would be nice", "wish"...).
- **A fonte mais valiosa:** a lista "Summary of improvement ideas, bugs" do xeno ([t=2410](https://champman0102.co.uk/viewtopic.php?t=2410), 222 respostas). São 104 itens, cada um marcado como resolvido (FIXED, quase sempre por patches de Tapani, Saturn e Nick+Co), bug, recurso ou incômodo. Também "How would you improve the game?" ([t=1593](https://champman0102.co.uk/viewtopic.php?t=1593)), com a voz dos jogadores comuns.

## Onde a comunidade vive (o que mais engaja)

| Tópico | Respostas | Visualizações |
|---|---|---|
| Atualização retrô de banco (Season 1984/85) | 5.922 | 554 mil |
| Nick's CM0102Patcher | 3.441 | 927 mil |
| [WIP/TESTING] 2026-27 restructures patch | 1.423 | 127 mil |
| Data Update - Technical Issues (2023-2026) | 1.301 | 248 mil |
| Instalar patches (Nick+Co, Saturn, Tapani) | 1.171 | 480 mil |
| Starter Kit FAQ | 1.127 | 371 mil |
| User Tactics, Tactic Benchmark League | 853 e 684 | 334 mil e 268 mil |
| Game Crashing | 829 | 150 mil |
| [RESOURCES] Offsets | 788 | 254 mil |

A comunidade gira em torno de **três atividades**: atualizar o banco de dados (temporadas atuais e retrô), instalar e combinar patches, e estudar táticas como esporte (ligas de benchmark). O resto do engajamento é suporte: instalar, fazer o jogo rodar, consertar quedas.

## O que já está resolvido por outros (não repetir)

Pela lista do xeno, os patches de Tapani, Saturn e Nick+Co (reunidos no NickPatcher) já resolveram, entre outros:
- calendário sem conflitos e remarcação de jogos;
- atributos do técnico humano visíveis, números de camisa;
- fundos acima de 3 bilhões;
- táticas melhores da IA, formações padrão melhores;
- janelas de transferência por país, datas de fim de temporada;
- decisões de compra e venda da IA, valores e salários, orçamento dos clubes;
- empréstimos (o "loan boost" do Saturn), o Regen Tweak (elenco 45);
- aposentadorias menos previsíveis, técnicos e presidentes se aposentando mais tarde;
- prêmios e direitos de TV em ligas que não tinham;
- várias competições transformadas (Mundial de Clubes, Gold Cup, Libertadores e Copa Argentina em vagas de copas extintas);
- suspensões imediatas, permissão de trabalho mais curta, a penalidade de posicionamento por idade.

A resolução e as telas maiores também já têm patch (Nick, Tapani, melanzalko). O save acima de 2 GB (item 104) foi resolvido por nós, no save2gb.

## O que está em aberto

### 1. Estruturas de competições: a maior demanda
- "Mudar a estrutura das ligas é o que mais importa para mim: aumentar ou diminuir o número de times e tornar mais ligas jogáveis" (SteveHarle, t=1593). O MLS, por exemplo, tem só 12 times no jogo original.
- O patch de reestruturação do davdav tem 1.423 respostas, mais 335 no tópico de suporte. Os formatos da Champions e da Liga Europa (t=3206, t=10794), os playoffs da National League inglesa (xeno 82) e as copas extintas convertidas em novas (xeno 79–81) são todos pedidos recorrentes.
- Hoje tudo isso se faz trocando ponteiros e ids gravados no exe ("dezenas de pontos do código", ver `CAMPOS.md` do projeto). Cada mudança é um patch frágil.
- Bugs em aberto: o rebaixamento holandês (xeno 5.1), o ranking da FIFA (35), as nações pequenas fora das eliminatórias (83), clube promovido sem estádio (46), o sorteio das quartas da Champions (103), datas das Olimpíadas e da Champions asiática (84, 86).

### 2. Mercado e IA
- **Empréstimos não funcionam:** "a oferta é recusada qualquer que seja o salário, ou ele não tem intenção de ir" (JLa, t=1593). No modo férias, os empréstimos ficam presos (xeno 88). O humano não consegue pegar emprestado jogadores em fim de contrato (58.2).
- **Clubes ricos parados**, com centenas de milhões sem uso (13.1). A IA não investe em instalações nem em estádio (30).
- **Contratos vencidos que não liberam** o jogador ou o treinador, que ficam a £0 por semana (95). O estatuto no elenco fica cinza depois de uma recusa (t=1593).
- **Técnicos trocando de clube de forma estranha:** pela reputação da liga, não do clube (36).

### 3. Juventude, regens e desenvolvimento
- "Nenhum jogador saiu da base em 5 anos" (t=5879, 100 respostas). É a pergunta do Here2Win79, que respondemos.
- **Nomes dos regens:** não usam nome comum (t=2669, 82 respostas). Depois de alguns anos, regens e árbitros ganham nomes de outras nações (xeno 64; t=4121: um "Flemming Beckford-Quailey" no Skonto Riga).
- **Atributos dos regens:** determinação baixa e posicionamento alto (t=4085).
- **O desenvolvimento frustra:** "jogadores pioram sem motivo aparente"; o jogo tira CA de uns para dar aos jovens (t=3528, 119 respostas).
- **Regens concentrados:** a mudança do Tapani que leva regens às nações grandes enche os grandes clubes de jovens (xeno 70). É o que vimos no patch de nacionalidade do CLZ.
- **Mais de 50 jogadores ou times B opcionais** (xeno 21). Ver a fase 6 do roadmap.

### 4. Economia do clube
- O estádio e o público deveriam crescer com o sucesso; a capacidade, aumentar com 90% de ocupação (xeno 8).
- Equilíbrio de receitas e despesas, com preço de ingresso, patrocínio e merchandising pela reputação (12). Os patrocínios quase nunca aparecem (92).
- O público em jogos de copa e da Champions é menor que em jogos comuns da liga (93). O estádio encolhe na falência (57).

### 5. Olheiros e informação
- "Os olheiros sempre voltam com os mesmos jogadores" (leovii, t=1593; xeno 51.1). O olheiro demora muito (50.1), e falta observação permanente (50.2).
- Olheiro e treinador com avaliação de potencial 20/20 deveriam saber que um PA 125 não serve para o Manchester United (Telcontar, t=1593).
- Revelar atributos escondidos por uma pesquisa mais longa do olheiro (Here2Win79, t=1593). Exame médico antes da contratação (51.2).

### 6. Qualidade de vida (telas)
- Todas as vagas de técnico numa tela (Jack Stein); pausa no modo férias quando chega uma proposta (18.2); adiar a resposta a uma proposta de emprego (62).
- Pular os jogos da Copa e da Euro; seguir com notícias não lidas (102); ver as expectativas da diretoria (96).
- Números de camisa até 99 (3.3); separar o lateral do ala e o volante do meia na exibição (Telcontar); elogiar jogadores, não só punir (Here2Win79); a tela do técnico de seleção com os jogadores elegíveis (Flu309).
- Mais opções no modo férias: vender só os da lista de transferências, voltar no fim da temporada (18.1).

### 7. Estabilidade e desempenho
- **A queda de 25 a 28 de dezembro:** 205 respostas (t=3077). Na verdade é um travamento (um laço infinito), e não uma queda.
  - **O contorno existe:** o Alan_v2 achou o ponto em outubro de 2025 e publicou um patch de um byte (0x92EA55 `JNZ` → `JMP`, que pula o laço). Ele mesmo escreveu que não sabe por que acontece, e o tópico continua recebendo casos novos até outubro de 2026.
  - **O ponto:** `world_cup.cpp` (a rotina em 0x92e940). Um laço sorteia uma vaga entre as posições 2 a 7 de uma lista de 8 registros de 5 bytes (o byte 0xFF marca a vaga como ocupada) e repete enquanto a vaga sorteada estiver ocupada. Se as seis estão ocupadas, não termina nunca. O pulo do Alan põe a seleção numa vaga já ocupada.
  - **Hipótese (a provar):** as atualizações de banco mudaram nações e confederações, e em algumas carreiras, pelos resultados das eliminatórias da Copa de 2002, sobram mais seleções do que vagas. Por isso só algumas carreiras, e só com bancos novos.
- "Game Crashing" (829 respostas), erros como "virtual staff cpp 195" (t=3423), a criação lenta da carreira (t=1582).

### 8. Plataformas
- **Mac** (t=1569, 360 respostas) e **Android** (t=1579, 296 respostas): hoje só por Wine e emuladores, lentos e frágeis.
- Resolução e tela (t=1583, 256 respostas); as resoluções maiores não funcionam no jogo em rede (xeno 52).

### 9. Comissão, seleções e nacionalidade
- Presidentes e diretores sem regeneração; o clube que começa sem presidente nunca ganha um (xeno 20).
- Técnicos de seleção treinando clubes pequenos ao mesmo tempo (65); a comissão evolui pouco em carreiras longas (o item 18 do petermf23).
- Segunda nacionalidade e restrições de estrangeiros (t=3668, 100 respostas); a regra de empréstimo de não comunitários anula a permissão de trabalho (xeno 69).

## Onde temos vantagem

| Frente | Por que é nossa | Tamanho |
|---|---|---|
| ~~Estruturas de competições~~ | **fora do escopo:** o davdav já faz isso, e bem | (a maior demanda, mas já atendida) |
| **A queda de dezembro e outras quedas** | o contorno do Alan_v2 pula o laço; com a rotina entendida, dá para corrigir a causa (a vaga que falta) e registrar o que a provoca | pequeno a médio; muito pedido |
| **Regens: fornada, nomes, nacionalidade, atributos** | já estamos dentro dessas rotinas | médio a grande (a fornada é a fase 6) |
| **Empréstimos e IA do mercado** | as rotinas de transferência e contrato estão em Rust | médio |
| **O desenvolvimento dos jogadores** | dá para trocar a regra que redistribui o CA | médio; mexe no equilíbrio do jogo |
| **Economia: estádio, público, patrocínio** | as finanças e a renda estão em Rust | médio |
| **Olheiros e informação** | regra (precisão pelo atributo de avaliação) e, depois, telas | médio |
| **Qualidade de vida (telas)** | exige telas novas: depois da interface idêntica, pela regra do projeto | grande; mais adiante |
| **Mac e Android** | só com o jogo inteiro em Rust; uma DLL não resolve | muito grande; longo prazo |

## Comparação com as hipóteses de antes da leitura

| Hipótese | O que a leitura mostrou |
|---|---|
| 1. Carreiras longas se degradam | **Confirmada:** nomes de outras nações, comissão, regens, presidentes |
| 2. Desempenho | **Parcial:** reclamam da criação da carreira e da velocidade das partidas, não do avançar dos dias |
| 3. Windows moderno, vídeo | **Confirmada**, e maior do que eu pensava: **Mac e Android** são grandes, e eu não tinha previsto |
| 4. IA no mercado | **Confirmada**, com destaque para os **empréstimos**, que eu não tinha previsto |
| 5. Motor de partidas | **Diferente:** a tática é um hobby (ligas de benchmark), não uma reclamação; a penalidade de posicionamento já foi corrigida |
| 6. Estruturas de competições | **Confirmada, e é a maior demanda** |
| 7. Juventude | **Confirmada**, com os **nomes dos regens** e o **desenvolvimento** como temas próprios |
| 8. Comissão técnica | **Confirmada** |
| 9. Regras desatualizadas | **Parcial:** segunda nacionalidade e estrangeiros, sim; o Brexit quase não aparece |
| 10. Moral e contratos | **Confirmada:** contratos vencidos que não liberam, estatuto cinza |
| 11. Ferramentas | **Confirmada:** editor de save, e principalmente o ecossistema de bancos de dados |
| 12. Jogo em rede | **Fraca:** pouca gente, mas existe (resolução no jogo em rede) |
| 13. Seleções | **Confirmada** (convocações, eliminatórias, ranking da FIFA) |

**O que eu não tinha previsto:** Mac e Android; os empréstimos; a economia do estádio e do público; os olheiros que voltam sempre com os mesmos nomes; a queda de dezembro; a frustração com o desenvolvimento; e o tamanho da cultura de **atualização de bancos**, que é a maior atividade do fórum.
