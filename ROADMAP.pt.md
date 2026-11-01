# Roadmap do CM0102-Rust-Tweaks

*English version: [ROADMAP.md](ROADMAP.md)*

Uma `ddraw.dll` de ajustes para o CM 01/02 3.9.68, com um seletor de opções. Na abertura, este repositório substitui o `cm0102-save2gb`. Atualizado em 01/11/2026.

O que a comunidade pede e o que está em aberto: [docs/OPORTUNIDADES.md](docs/OPORTUNIDADES.md).

Legenda: ✅ feito · 🔨 em andamento · ⏳ a fazer · 🔬 a pesquisar · 💡 ideia, sem decisão

## Fase 0: a base ✅

- ✅ A regra original dos regens decifrada ([docs/DISTRIBUICAO_DOS_REGENS.md](docs/DISTRIBUICAO_DOS_REGENS.md)): um regen por aposentadoria, a disputa pelo maior sorteio(nota), o piso da IA, o degrau do Training 20.
- ✅ A DLL repassadora, com cada ajuste ligado por uma chave do `tweaks.cfg`. Sem o arquivo, nada muda.
- ✅ A regra do projeto para exe já modificado:
  - as bordas têm de bater, senão o ajuste nunca liga;
  - o corpo alterado (o Regen Tweak dos exes do CLZ, por exemplo) fica com o exe, salvo `override_exe_changes = 1`.
- ✅ **Save acima de 2 GB** (`save_over_2gb`), o mesmo do save2gb.
- ✅ **Idade dos regens** por tabela (`age_NN`, `contest_max_age`, `ages_out_of_game`).
- ✅ **Regens mais novos**, a ideia de petermf23: com 22 anos ou mais, 6 a menos (`younger_from_age`, `younger_by`).
- ✅ **A disputa por sorteio proporcional** (`rule = new`): nível × instalações × comissão × vaga.
- ✅ As ferramentas: `calibrar.py` (as duas regras com os clubes reais de um save), `regens_por_clube.py`, `simula_nota.py`.

## Fase 1: o equilíbrio dos regens ✅

A configuração que a DLL distribui.

**Decidido:**

| Chave | Valor | Por quê |
|---|---|---|
| `sigma` | 2500 | curva do nível mais larga, acima do alvo |
| `sigma_below` | 1200 | abaixo do alvo a curva cai mais depressa: com uma largura só, na Itália quase metade dos jovens de PA ≥ 140 ia para clubes de reputação abaixo de 2000 |
| `staff_min`, `staff_max` | 0,8 e 1,2 | o melhor treinador com jovens vale 50% a mais que o pior; o humano conta igual à IA |
| `temperature` | 1 | o sorteio proporcional puro; a aleatoriedade fica com a surpresa |
| `facilities_base`, `facilities_k` | 0,4 e 0,06 | Training 20 vale 3,5 vezes o Training 1, sem degrau |
| `season_cap` | 2 | no máximo 2 jovens da disputa por clube a cada 365 dias |
| `squad_limit` | 48 | os elencos grandes continuam na disputa; quem freia é o teto de 2 por ano (o Saturn usa 45) |
| `surprise` | 15% | o jovem de PA alto que nasce no clube pequeno (Ronaldo no São Cristóvão), só entre clubes de competição; o mercado o leva depois |
| idades | a tabela original + regens mais novos | a ideia de petermf23 |
| `retire_idle_players` | ligado | aposentar quem ninguém quer libera fichas, e mais fichas viram mais regens por ano (implementado: a 0x7a8050 trocada) |
| `idle_days`, `idle_min_age` | 180 dias, 28 anos | meia temporada sem clube, a partir dos 28 |
| `idle_rep`, `idle_rep_per_year` | 1500, +250 por ano | a chance sobe com a idade (reputação 3000: 50% aos 28, 100% aos 34) |
| `protect_young_unemployed` | 0 | sem proteção: numa carreira de teste, proteger os menores de 23 cortou a maior parte da reciclagem em regens (a original aposenta em março centenas de jovens sem clube há mais de um ano) |

✅ **Calibrado** em 01/11/2026 na Inglaterra, no Brasil e na Itália (os números estão no README, § A calibração). O que o simulador não mede, os elencos reais ao longo da carreira, fica para a prova no jogo.

## Fase 2: a prova no jogo ⏳

- ⏳ Carreira automática de férias numa bancada, com o `cm0102.exe` original, a DLL encadeada na `ddraw.dll` do Windows, sempre em janela. Conferir:
  - que o `tweaks.log` mostra cada ajuste ligado;
  - que a carreira passa de uma ou duas temporadas sem travar;
  - os regens com `log = 1`: idade, clube e chance de cada um.
- ⏳ Comparar a distribuição por faixa com e sem a DLL (`regens_por_clube.py`) contra o que o simulador previu.
- ⏳ Medir **quantos regens a mais por ano** com `retire_idle_players` e a idade dos que se aposentam por ela.
- ⏳ Medir **quem contrata os jovens que nascem sem clube** (uns 44%). Isso decide a ideia da fase 4.
- ⏳ Save: gravar e carregar com e sem a DLL, e um save acima de 2 GB.
- ⏳ Exes do NickPatcher e do CLZ, com `override_exe_changes` em 0 e em 1.

## Fase 3: o configurador (seletor de opções) ⏳

Um `.exe` só, com interface própria (sem copiar a do NickPatcher). Não precisa ser Tauri, mas deve ser bonito e visual:
- cada opção com uma **barra de arrastar junto do campo de digitação**: arrastar ou digitar, os dois andam juntos, dentro da faixa válida;
- as opções agrupadas por ajuste (save, idade, inativos, disputa, nacionalidade), com a ajuda de cada uma (o mesmo texto do `.cfg`) ao passar o mouse ou num painel ao lado;
- recursos visuais onde ajudam a entender: a tabela de idades como gráfico de barras arrastáveis, a curva do nível desenhada, um exemplo de chances por clube que muda enquanto o usuário arrasta;
- lê e grava o `tweaks.cfg` sem apagar os comentários; o arquivo continua sendo a única fonte;
- predefinições ("Original", "Equilibrado"...);
- mostra, para o exe escolhido, o que cada ajuste vai fazer: liga, fica a do exe (com o nome da modificação achada, como o Regen Tweak) ou não serve;
- tecnologia a decidir: um exe só, sem instalador e sem depender de nada instalado (por exemplo, Rust com uma biblioteca de interface que desenha sozinha).

## Fase 4: mais ajustes 💡🔬

- 🔬 **A queda de 27 de dezembro** (o laço da Copa do Mundo em `world_cup.cpp`, 0x92ea3b..0x92ea71; o contorno de um byte é do Alan_v2):
  - entender a rotina 0x92e940: o que são os 8 registros de 5 bytes, quem os preenche e por que sobram seleções;
  - reproduzir: um modo de diagnóstico na DLL registra a lista inteira quando o laço passa de algumas voltas; carreiras automáticas com um banco atualizado (o 25/26 da comunidade, a obter com o Lucas) até 28/12/2001, com várias sementes;
  - corrigir na causa, com o crédito ao Alan_v2 pela descoberta do ponto.
- 🔬 **Empréstimos** (a reclamação mais repetida do mercado; **mapa em [docs/EMPRESTIMOS.md](docs/EMPRESTIMOS.md)**: o clube da IA recusa todo lance de empréstimo na resposta a lances, e a previsão só aceita jogador listado com reputação suficiente): mapear nas rotinas já reescritas em Rust quem decide (o clube que recusa, o jogador "sem intenção de ir", o modo férias que trava), registrar numa carreira automática cada oferta e o seu desfecho, e só então propor regras configuráveis.
- 🔬 **Economia do clube:** o público e o estádio que não crescem com o sucesso, o patrocínio raro, o público menor nas copas, a IA que não investe em instalações. O mesmo método: mapear nas rotinas (a renda da partida, os eventos das finanças e as obras do estádio já estão em Rust no projeto; ver `OBRAS_DO_ESTADIO.md` lá), medir e propor.
- 🔬 **A nacionalidade dos regens, com regras definidas** (a decidir com o Lucas). Hoje há duas regras em uso [observado no código]:
  - **a original** (0x7abcd7..0x7abd1d): se a nação do jovem tem menos de 100 pessoas cadastradas (+0x88), em 1 de 4 vezes ele ganha a nacionalidade de outra nação sorteada, também pequena;
  - **a dos exes do CLZ** (autor a confirmar): todo jovem de PA 120 ou mais ganha a nacionalidade de uma nação sorteada entre as de reputação 5632 ou mais (+0x8e); os outros ficam com a de sempre. Concentra os craques nas nações grandes.
  - A ideia é um ajuste `nationality` com modos e chaves: o original; "nações fortes" como o CLZ, com o PA e a reputação mínimos ajustáveis; e um sorteio proporcional à reputação da nação, com a chance de manter a nação de origem. O trecho fica dentro da 0x7abac0 e seria trocado como o da idade, respeitando o exe que já o altera (o CLZ).
- 🔬 **Comissão evolui por mais tempo**, a ideia de petermf23 (item 18 do mod dele das divisões polonesas para o 3.9.60):
  - técnicos, auxiliares, treinadores, olheiros e fisioterapeutas continuam evoluindo até uns 55 anos, em vez de uns 40;
  - melhora a qualidade e a variedade da comissão em carreiras longas;
  - a fazer: localizar o limite no código (a evolução dos atributos de não jogador), conferir a rotina e expor a idade no `.cfg` (`staff_development_age`), com o crédito a ele.
- 💡 **Os que nascem sem clube** passarem pelo sorteio, só entre clubes de divisões inferiores. Depende da medição da fase 2.
- 💡 **As instalações no PA**: na assinatura, PA + (Training − 10) × 0,5 ± 2. Desligado por padrão.
- 🔬 **Criar fichas novas** para ter mais regens do que aposentadorias: aumentar `player.dat` e as tabelas por pessoa. É um projeto de pesquisa, com risco para o save; fora da primeira versão.

## Fase 5: a abertura ⏳

- ⏳ Tirar dos documentos os caminhos do projeto privado.
- ⏳ Release: um zip com `ddraw.dll`, `tweaks.cfg` e os README, e o código-fonte no repositório.
- ⏳ O `cm0102-save2gb` passa a apontar para cá.
- ⏳ Postagem no fórum, com o rascunho aprovado pelo Lucas antes.
- ⏳ Créditos: xeno e Saturn (o Regen Tweak), petermf23 (regens mais novos e a evolução da comissão), Nick (NickPatcher, as referências), e a comunidade do champman0102.

## Fase 6: a fornada de jovens 🔬

A "fornada" do FM: uma leva de jovens chega à base de cada clube todo ano, em vez de um regen por aposentadoria. É o objetivo grande deste repositório, e é só nosso: exige criar gente, controlar o save e equilibrar a população. Decidido com o Lucas em 01/11/2026.

**O que não se faz:** aumentar o limite de 50 jogadores. As 50 vagas ficam dentro do registro do clube (+0xd7), e o resto do registro (treinadores +0x19f em diante) é lido por posição fixa em todo o jogo, no save e nos editores. A fornada trabalha dentro dos 50: hoje um clube típico tem de 30 a 40 jogadores, e a IA abre espaço.

**As peças que o jogo já tem:** `regenerate_as_youth_player` (0x7abac0) transforma uma ficha num jovem; `player_regen_sign_for_club` (0x7a6f10) o põe numa vaga do elenco, com contrato. O que falta é a matéria-prima, fichas livres: as tabelas de pessoas têm tamanho fixo, alocado na carga.

1. 🔬 **Prova de conceito**
   - **A reserva de fichas na criação da carreira.** O jogo já fabrica pessoas a mais ao criar uma carreira: 1% (2% numa opção), por uma constante em 0x524dcf/0x524de1 (`database_trim_staff_tables` 0x524d70). A DLL sobe esse número e marca essas fichas como reserva da fornada; as tabelas nascem com folga, e o save grava as contagens. A prova: uma carreira com 10% de pessoas a mais nasce e roda estável na bancada (tempo por dia, memória, save).
   - **Um item próprio dentro do save.** A DLL controla o contêiner (as rotinas do save2gb); o jogo só lê os itens que conhece. A prova: gravar e ler um item da DLL (a reserva, as datas da fornada) e o jogo carregar o save normalmente, com e sem a DLL.
2. ⏳ **A fornada:** numa data por nação, cada clube recebe N jovens da reserva, com N pelas instalações, pelo treinador de juniores e por um sorteio. O PA vem de uma distribuição própria (instalações 20 passam a gerar mais talento de verdade). Tudo no `.cfg`.
3. ⏳ **O espaço no elenco:** a IA dispensa, empresta ou aposenta os excedentes; quem fica sem clube volta à reserva pela regra dos inativos. O humano recebe a fornada como notícia, e os jovens aparecem no elenco.
4. ⏳ **O equilíbrio da população:** fornada, aposentadoria e reciclagem na reserva, para a carreira não inchar década após década; medido no simulador e em carreiras longas na bancada.

**Os impactos a medir:** mais pessoas (o processamento dos dias); save maior (já resolvido acima de 2 GB); talento demais no mundo (inflação de PA, seleções); mercado inundado de jovens (salários e preços); editores e ferramentas (contagens maiores, formato igual).

**Limite:** vale para carreiras novas, porque a reserva nasce na criação. Num save antigo, a fornada só usaria fichas recicladas.
