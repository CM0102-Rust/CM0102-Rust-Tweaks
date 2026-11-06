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
- ✅ **Sem SetupReg e sem administrador** (`portable_folder`, `clear_admin_flag`, 02/11/2026, pedido do CLZ): a pasta do jogo é a do exe e o `RUNASADMIN` que a NickDLL grava sai (o `WIN95` fica: sem ele o v6.1 cai ao criar a carreira), sem mexer na NickDLL. Provado na cópia do CLZ v6.1; falta a primeira abertura com a marca já gravada (elevada).
- ✅ **Idade dos regens** por tabela (`age_NN`, `contest_max_age`, `ages_out_of_game`).
- ✅ **Regens mais novos**, a ideia de petermf23: com 22 anos ou mais, 6 a menos (`younger_from_age`, `younger_by`).
- ✅ **Sem clube só sem vaga** (`unattached = only_if_no_room`): todo jovem de nação no jogo vai à disputa; em cinco temporadas, os regens sem clube caem de 899 para 353 (fábrica) e de 233 para 77 (CLZ).
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

## Fase 2: a prova no jogo 🔨

Os testes e os números de cada execução: [docs/TESTES.md](docs/TESTES.md).

- ✅ Carreira automática de férias numa bancada, oculta e em janela, com a DLL encadeada atrás da `ddraw.dll` de trabalho (só o vídeo): cada ajuste aparece no `tweaks.log`, as carreiras passam de uma temporada e meia (até quase duas no CLZ) sem travar, e cada regen vai para o log com idade, clube e chance.
- ✅ A distribuição no jogo contra a original, no banco de fábrica, no oficial de outubro de 2026 e no CLZ: a concentração some (máximo de 3 por clube, contra 13), como o simulador previu.
- ✅ **Quantos regens a mais por ano** com `retire_idle_players`: 772 aposentadorias em cinco temporadas no banco de fábrica (cerca de 150 por ano); no CLZ, 9, porque o RetirePatchV4 barra quase todas.
- ✅ **Os jovens que nasciam sem clube** (uns 44%): `unattached = only_if_no_room` os põe na disputa; no banco de 2026, todos os 266 foram a algum clube.
- ✅ Save: o acima de 2 GB carrega com a chave e falha sem ela, no exe do projeto e no CLZ. ⏳ Gravar um save acima de 2 GB numa carreira de verdade, com o Lucas.
- ✅ Exes do NickPatcher e do CLZ, com `override_exe_changes` em 0 e em 1.
- ✅ **A carreira longa**: cinco temporadas com 10 ligas (6 jogáveis e 4 de fundo), no banco de fábrica e no CLZ (regras do Nick e as nossas, com `unattached`), sem nenhuma queda; a linha `season` do log dá a população a cada julho ([docs/TESTES.md](docs/TESTES.md) §4.3).
- ✅ **Defeito do teto corrigido** (02/11): o teto de 2 por 365 dias contava só o elenco e vazava quando o clube emprestava ou vendia o jovem (15 no Real Madrid em quatro anos); agora a DLL guarda cada jovem entregue, e o máximo por temporada ficou em 2.
- 💡 **O acúmulo nas nações fora do jogo:** mesmo com o teto, um clube junta 17 (fábrica) a 24 (CLZ) regens em cinco anos; no CLZ é o AS Douanes (Senegal, fora do jogo), por onde o regen não passa pela disputa. Invisível para quem joga; um teto ali é opção futura.
- ✅ Carreiras de 10 temporadas e variantes ([docs/TESTES.md](docs/TESTES.md) §4.4 e §4.5); `outside_competitions` para os elencos (03/11, em medição).
- ✅ **De quem é cada regen** (`regen_origins`, 03/11): o `regen_origins.csv` com a vaga e a ficha de cada regen, nomes do momento, datas do jogo e do computador e a carreira; busca e limpeza no configurador (§4.6).
- ⏳ A ordem dos bancos: a base de fábrica (referência); o CLZ, com o Lucas; outros pacotes da comunidade, quando saírem.
- 💡 Não cortar a reputação do jovem que nasce sem clube (o jogo a corta para 1250 e a mundial para 1, §4.6).

## Fase 2b: o retrato do exe 🔨

Hoje a DLL só percebe as mudanças nas rotinas que ela troca. No CLZ, o NickPatcher mudou quem chama a regra dos inativos (a busca de aposentadorias, 0x7a5790), e isso passou despercebido. Para documentar comportamentos sem depender do exe alheio (decisão do Lucas, 01/11/2026), em três camadas:

1. ✅ **O retrato do exe inteiro** contra o 3.9.68 original: o código função por função, os dados (`.rdata`, `.data`) em blocos e as seções novas (como a `.nick`), com os desvios que saltam para elas. Na DLL, dois retratos: o do arquivo no disco (o que o patcher gravou) e o da memória depois que as outras DLLs carregaram (o que muda em tempo de execução). Primeiro como ferramenta à parte (`tools/retrato_exe.py`, pronta: no CLZ, reconhece 85 patches do catálogo, entre eles o RetirePatchV4 que zerou os inativos, e lista por função os 16 mil bytes sem nome), depois dentro da DLL (`src/retrato.rs`, pronto: a tabela `src/referencia.rs`, gerada por `tools/gerar_referencia.py`, tem o hash das 11.314 funções e de 373 blocos de dados do 3.9.68; o log diz "the same code and data as the original 3.9.68" ou quantas funções e blocos mudaram, o ponto de entrada, as seções novas e os casos já identificados por nome; com `log = 1`, a lista. No CLZ: 519 funções, 28 blocos, a `.nick`, seis casos conhecidos; na memória, a NickDLL muda mais 1.205 trechos ao rodar).
2. 🔨 **O nome das mudanças** (feito fora da DLL: `tools/ler_retrato.py` dá a cada linha do log a função, o arquivo de código e os patches do catálogo aplicados nela; o catálogo não vai dentro da DLL, para não redistribuir os dados do Nick; dentro dela, só os casos que este projeto identificou), pelo catálogo do NickPatcher (os patches da comunidade em `MiscPatches.zip`, com bytes e descrição, e as opções do próprio Nick no `Patcher.cs`), lido como dado e com o crédito a Nick+Co e aos autores. O que não estiver no catálogo aparece como mudança desconhecida, com o nome da função.
3. 🔨 **O que afeta cada ajuste** (primeira parte pronta: se outra DLL muda, ao rodar, um trecho que um ajuste conferiu na abertura, o log avisa; no CLZ, a NickDLL desvia a 0x540a50, do time B, que a disputa chama, e a 0x5e8290, que o contêiner do save chama; as duas observadas e inofensivas, viraram nota). Falta o grafo de chamadas: pelo grafo de chamadas do exe original, quem chama e quem é chamado pela rotina que cada ajuste troca; alterada alguma, o log avisa.

## Fase 3: o configurador (seletor de opções) 🔨

O projeto da interface e uma maquete interativa: [docs/CONFIGURADOR.md](docs/CONFIGURADOR.md) e `docs/configurador_maquete.html` (02/11). **O protótipo do programa** (`configurador/`, Rust + egui, `tweaks-config.exe`) está pronto desde a noite de 02/11 (§10 de lá): com o diálogo de pasta e a confirmação das predefinições; falta o teste sem placa de vídeo. As decisões que dependem do Lucas estão no §9 de lá. O painel de estado do exe usa os estados de [docs/COMPATIBILIDADE.md](docs/COMPATIBILIDADE.md): ✅ compatível, ⚠️ sobrescreve (com o nome do patch), 🔁 fica com o outro, ❓ desconhecido.

Um `.exe` só, com interface própria (sem copiar a do NickPatcher). Não precisa ser Tauri, mas deve ser bonito e visual:
- cada opção com uma **barra de arrastar junto do campo de digitação**: arrastar ou digitar, os dois andam juntos, dentro da faixa válida;
- as opções agrupadas por ajuste (save, idade, inativos, disputa, nacionalidade), com a ajuda de cada uma (o mesmo texto do `.cfg`) ao passar o mouse ou num painel ao lado;
- recursos visuais onde ajudam a entender: a tabela de idades como gráfico de barras arrastáveis, a curva do nível desenhada, um exemplo de chances por clube que muda enquanto o usuário arrasta;
- lê e grava o `tweaks.cfg` sem apagar os comentários; o arquivo continua sendo a única fonte;
- predefinições ("Original", "Equilibrado"...);
- explica o que é "nação no jogo": a liga escolhida ao criar a carreira, **jogável ou de fundo** (para os regens, as duas contam igual; o byte `NationLeagueSelected` é 2 na jogável e 1 na de fundo, e o código dos regens só testa se não é 0); "fora do jogo" é a nação cuja liga não foi escolhida. Vale para as idades, para `unattached` e para a disputa;
- mostra, para o exe escolhido, o que cada ajuste vai fazer: liga, fica a do exe (com o nome da modificação achada, como o Regen Tweak) ou não serve;
- tecnologia a decidir: um exe só, sem instalador e sem depender de nada instalado (por exemplo, Rust com uma biblioteca de interface que desenha sozinha).

## Fase 4: mais ajustes 💡🔬

- 🔨 **Vídeo, resolução e CD** (pedido do Lucas, 02/11; feito, falta o teste visual), três chaves desligadas por padrão, cada uma conferindo a assinatura do trecho e ficando com o exe se ele já o mudou (o CLZ tem resolução, fundos dinâmicos e NoCD próprios); nada além disso entra na DLL, para não conflitar com o NickPatcher:
  - `video = window`: a camada de vídeo do projeto (o jogo desenha num DirectDraw nosso e o quadro vai para uma janela comum): Print Screen, janela redimensionável com escala nítida, nada troca o modo de vídeo do Windows. Vale em 800×600;
  - `resolution = 1080`: a resolução 1920×1080 do projeto, aplicada em memória, com a trava das 30 linhas no `Area::init` junto (sem ela, o relatório de uma partida antiga dá a caixa "Area.cpp 507");
  - `no_cd = 1`: o NoCD do projeto.
  - ✅ No 3.9.68, em carreiras ocultas: o `cm0102.exe` intacto passa do CD e roda em 1080 pela camada de vídeo; `video = window` sozinho em 800×600; no exe que já é 1080, "already has it" ([docs/TESTES.md](docs/TESTES.md) §3.7). No CLZ, a DLL faz o possível: completa o que o exe ainda não tem e deixa o CD com a LaMOHatesCD e a resolução com a NickDLL do 6.1 ([docs/COMPATIBILIDADE.md](docs/COMPATIBILIDADE.md)).
  - ⏳ Com o Lucas olhando a tela: a janela, o Print Screen, o relatório de uma partida antiga em 1080 (a trava das 30 linhas) e, no CLZ, `video = window` (o CLZ em janela com a `ddraw.dll` do Windows para em "16 bit colour"; a camada de vídeo deve resolver), as resoluções e os fundos dinâmicos.

- ✅ **A linha `ARSE` do `player_setup.cfg`** (aviso da sessão CM0102 e pedido do Lucas de 04/11): a linha marca todo mundo como regen. A DLL reconhece a carreira por um técnico humano com a marca e, nela, o `season_cap` não conta pela marca; o `tweaks.log` avisa. Saiu na 1.1 ([docs/TESTES.md](docs/TESTES.md) §3.10).
- ✅ **Nomes dos regens do próprio país** (`regen_names = own`, pedido do Lucas de 03/11). Os nomes ficam no banco; a regra de quando o país usa os próprios (só acima de 100) está no exe. No update de outubro de 2026, 100 das 231 nações ainda pegam emprestado; com o mínimo de 20, ficam 15, quase todas sem nome nenhum (nações extintas). Corrige também o primeiro nome tirado da tabela dos sobrenomes no grupo de língua 0. Saiu na 1.1. 💡 Depois, se precisar: um banco de nomes para as poucas nações com menos de 20.
- ✅ **Jogadores cinzentos no nível do clube** (`grey_players = squad`, pedido do Lucas de 03/11: "esses jogadores precisam ser fortes no nível do time e não capados para serem ruins"; estudo e código da sessão dos cinzentos do projeto CM0102, pedido P9). Desligado por padrão; a variação em volta do nível é regulável (± 5 por padrão). ✅ Provado no 3.9.68 (três carreiras até 14/09/2001) e no CLZ 6.1 com a NickDLL ([docs/TESTES.md](docs/TESTES.md) §3.9). Saiu na 1.1, com o `multiple_instances`. 💡 Depois: efetivar um cinzento (clicar e contratar), que já tem estudo no projeto CM0102.
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

## Fase 5: a abertura ⏳

- ⏳ Tirar dos documentos os caminhos do projeto privado.
- ⏳ Release: um zip com `ddraw.dll`, `tweaks.cfg` e os README, e o código-fonte no repositório.
- ⏳ O `cm0102-save2gb` passa a apontar para cá.
- ⏳ Postagem no fórum, com o rascunho aprovado pelo Lucas antes.
- ⏳ Créditos: xeno e Saturn (o Regen Tweak), petermf23 (regens mais novos e a evolução da comissão), Nick (NickPatcher, as referências), e a comunidade do champman0102.

## Fase 6: a fornada de jovens 🔨

A "fornada" do FM: uma leva de jovens chega à base de cada clube todo ano, em vez de um regen por aposentadoria. É o objetivo grande deste repositório, e é só nosso: exige criar gente, controlar o save e equilibrar a população. Decidido com o Lucas em 01/11/2026.

**O que não se faz:** aumentar o limite de 50 jogadores. As 50 vagas ficam dentro do registro do clube (+0xd7), e o resto do registro (treinadores +0x19f em diante) é lido por posição fixa em todo o jogo, no save e nos editores. A fornada trabalha dentro dos 50: hoje um clube típico tem de 30 a 40 jogadores, e a IA abre espaço.

**As peças que o jogo já tem:** `regenerate_as_youth_player` (0x7abac0) transforma uma ficha num jovem; `player_regen_sign_for_club` (0x7a6f10) o põe numa vaga do elenco, com contrato. O que falta é a matéria-prima, fichas livres: as tabelas de pessoas têm tamanho fixo, alocado na carga.

1. 🔬 **Prova de conceito**
   - **O estudo (04/11/2026, pedido do Lucas): [docs/FORNADA_DE_REGENS.md](docs/FORNADA_DE_REGENS.md).** Substitui a ideia de subir a constante do corte do banco: as pessoas não podem crescer (os 16 técnicos humanos são as últimas contadas, e a tabela de 79 bytes nasce no tamanho exato), e, numa carreira longa, é a vaga (a pessoa) que limita os regens, não a ficha: no save de todas as ligas, uns 7.500 regens por temporada, e 3 por clube pediria de 11.700 a 20.300. Com a população fixa, regens a mais só vêm de aposentadorias a mais, a partir dos ~13.000 sem clube. A ordem proposta: as aposentadorias e os regens num dia fixo, a cota por clube com os regens que já nascem, o ritmo das aposentadorias, e as pessoas novas com ficha: viável ([docs/PESSOAS_NOVAS.md](docs/PESSOAS_NOVAS.md)), primeiro por um estoque criado com a carreira (um desvio no corte do banco, que já cria pessoas a mais) e depois crescendo em qualquer save (~276 patches nos humanos).
   - **Um item próprio dentro do save.** A DLL controla o contêiner (as rotinas do save2gb); o jogo só lê os itens que conhece. A prova: gravar e ler um item da DLL (a reserva, as datas da fornada) e o jogo carregar o save normalmente, com e sem a DLL.
2. ⏳ **A fornada:** numa data por nação, cada clube recebe N jovens da reserva, com N pelas instalações, pelo treinador de juniores e por um sorteio. O PA vem de uma distribuição própria (instalações 20 passam a gerar mais talento de verdade). Tudo no `.cfg`.
3. ⏳ **O espaço no elenco:** a IA dispensa, empresta ou aposenta os excedentes; quem fica sem clube volta à reserva pela regra dos inativos. O humano recebe a fornada como notícia, e os jovens aparecem no elenco.
4. ⏳ **O equilíbrio da população:** fornada, aposentadoria e reciclagem na reserva, para a carreira não inchar década após década; medido no simulador e em carreiras longas na bancada.

**Os impactos a medir:** mais pessoas (o processamento dos dias); save maior (já resolvido acima de 2 GB); talento demais no mundo (inflação de PA, seleções); mercado inundado de jovens (salários e preços); editores e ferramentas (contagens maiores, formato igual).

**Saiu na 1.2 (05/11/2026, experimental, as chaves `intake_*`):** o estoque criado com a carreira nova (o corte do banco), adormecido e solto no dia da fornada (um dia fixo ou o fim da temporada de cada nação); cada jovem a cópia de um jogador real da nação do clube, com o PA pela reputação do clube; os regens só pela fornada por padrão (a fila de aposentados a alimenta); a mensagem do treinador da base na caixa de entrada, em faixas, na língua do jogo. Detalhes e testes: [docs/PESSOAS_NOVAS.md](docs/PESSOAS_NOVAS.md) §8.

**Próximos, na fornada:**
- 💡 **O dia da fornada por país** (Lucas, 05/11): hoje é uma escolha só para todos os países (um dia fixo, ou o fim da temporada de cada nação como o jogo tem). Uma linha por país no `tweaks.cfg` (ex.: `intake_day_BRA = 10`) por cima da regra geral, e uma lista na aba do configurador.
- ⏳ Gravar e carregar com o Lucas, com e sem a DLL, e nos editores.
- 🔨 **O desenho final (05/11, noite, fechado com o Lucas): regen é regen.** A fornada muda só a data e a quantidade (`intake_per_club` por clube com vaga); os regens que as aposentadorias não cobrem são criados (a ficha misturada de dois aposentados da mesma posição) e todos vão para os clubes pela curva da aba Regens, com cotas. Saíram as chaves de PA do estoque, o `intake_only` e o `intake_squad_max`. Nos testes: [docs/PESSOAS_NOVAS.md](docs/PESSOAS_NOVAS.md) §8.
- 💡 A reserva com todas as ligas: aumentar a alocação (0x529bfe, 0x529b39) se uma carreira com todas as ligas ficar curta.

**Limite:** as fichas novas valem primeiro para carreiras novas (a carga de um save antigo exige o `training.dat` com a contagem exata). O dia fixo e a cota por clube valem em qualquer save.
