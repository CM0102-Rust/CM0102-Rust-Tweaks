# Roadmap do CM0102-Rust-Tweaks

*English version: [ROADMAP.md](ROADMAP.md)*

Uma `ddraw.dll` de ajustes para o CM 01/02 3.9.68, com um seletor de opções. Na abertura, este repositório substitui o `cm0102-save2gb`. Atualizado em 01/11/2026.

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

## Fase 1: o equilíbrio dos regens 🔨

A configuração que a DLL distribui.

**Decidido:**

| Chave | Valor | Por quê |
|---|---|---|
| `sigma` | 2500 | curva do nível mais larga |
| `facilities_base`, `facilities_k` | 0,4 e 0,06 | Training 20 vale 3,5 vezes o Training 1, sem degrau |
| `season_cap` | 2 | no máximo 2 jovens da disputa por clube a cada 365 dias |
| `squad_limit` | 48 | os elencos grandes continuam na disputa; quem freia é o teto de 2 por ano (o Saturn usa 45) |
| `surprise` | 15% | o jovem de PA alto que nasce no clube pequeno (Ronaldo no São Cristóvão), só entre clubes de competição; o mercado o leva depois |
| idades | a tabela original + regens mais novos | a ideia de petermf23 |

**Aberto:**
- ⏳ **A aleatoriedade** (`temperature`, proposta 1, o sorteio proporcional puro): com a surpresa, ela deixa de ser o botão da aleatoriedade.
- ⏳ **A comissão** (`staff_min` e `staff_max`, hoje de 0,8 a 1,2): confirmar o peso.
- ⏳ **Calibrar** a configuração completa na Inglaterra, no Brasil e numa liga média (Itália ou Portugal), e fechar os números.

## Fase 2: a prova no jogo ⏳

- ⏳ Carreira automática de férias numa bancada, com o `cm0102.exe` original, a DLL encadeada na `ddraw.dll` do Windows, sempre em janela. Conferir:
  - que o `tweaks.log` mostra cada ajuste ligado;
  - que a carreira passa de uma ou duas temporadas sem travar;
  - os regens com `log = 1`: idade, clube e chance de cada um.
- ⏳ Comparar a distribuição por faixa com e sem a DLL (`regens_por_clube.py`) contra o que o simulador previu.
- ⏳ Medir **quem contrata os jovens que nascem sem clube** (uns 44%). Isso decide a ideia da fase 4.
- ⏳ Save: gravar e carregar com e sem a DLL, e um save acima de 2 GB.
- ⏳ Exes do NickPatcher e do CLZ, com `override_exe_changes` em 0 e em 1.

## Fase 3: o configurador (seletor de opções) ⏳

Um `.exe` só, com interface própria (sem copiar a do NickPatcher):
- lê e grava o `tweaks.cfg` sem apagar os comentários; o arquivo continua sendo a única fonte;
- confere as faixas de cada valor e oferece predefinições ("Original", "Equilibrado"...);
- mostra, para o exe escolhido, o que cada ajuste vai fazer: liga, fica a do exe (com o nome da modificação achada, como o Regen Tweak) ou não serve;
- opcional: para a curva dos regens, a tabela do simulador com os clubes de um save.

## Fase 4: mais ajustes 💡🔬

- 🔬 **Comissão evolui por mais tempo**, a ideia de petermf23 (item 18 do mod dele das divisões polonesas para o 3.9.60):
  - técnicos, auxiliares, treinadores, olheiros e fisioterapeutas continuam evoluindo até uns 55 anos, em vez de uns 40;
  - melhora a qualidade e a variedade da comissão em carreiras longas;
  - a fazer: localizar o limite no código (a evolução dos atributos de não jogador), conferir a rotina e expor a idade no `.cfg` (`staff_development_age`), com o crédito a ele.
- ⏳ **Aposentar os inativos** (`retire_idle_players`, aprovado em 01/11; os números estão em discussão): antecipar a aposentadoria de quem está sem clube há muito tempo, com 30 anos ou mais e CA baixo.
  - Libera fichas de jogador, e mais fichas viram mais regens por ano.
  - A população total não muda, nem o formato do save.
  - A regra da aposentadoria (0x7a7310) já está decifrada no projeto CM0102.
- 💡 **Os que nascem sem clube** passarem pelo sorteio, só entre clubes de divisões inferiores. Depende da medição da fase 2.
- 💡 **As instalações no PA**: na assinatura, PA + (Training − 10) × 0,5 ± 2. Desligado por padrão.
- 🔬 **Criar fichas novas** para ter mais regens do que aposentadorias: aumentar `player.dat` e as tabelas por pessoa. É um projeto de pesquisa, com risco para o save; fora da primeira versão.

## Fase 5: a abertura ⏳

- ⏳ Tirar dos documentos os caminhos do projeto privado.
- ⏳ Release: um zip com `ddraw.dll`, `tweaks.cfg` e os README, e o código-fonte no repositório.
- ⏳ O `cm0102-save2gb` passa a apontar para cá.
- ⏳ Postagem no fórum, com o rascunho aprovado pelo Lucas antes.
- ⏳ Créditos: xeno e Saturn (o Regen Tweak), petermf23 (regens mais novos e a evolução da comissão), Nick (NickPatcher, as referências), e a comunidade do champman0102.
