# regens: outra regra para a distribuição dos regens (protótipo)

*English version: [README.md](README.md)*

Uma `ddraw.dll` só dos regens, no molde da do save2gb (`save/`). Ela troca a disputa dos clubes pelo jovem regen (`PLAYER_REGEN::pick_best_club_for_player`, 0x7ad0e0) por um sorteio proporcional configurável em `regens.cfg`. O resto do jogo, inclusive a criação do jovem, segue igual. A regra original está em [docs/DISTRIBUICAO_DOS_REGENS.md](docs/DISTRIBUICAO_DOS_REGENS.md).

A mesma DLL traz o **save acima de 2 GB** do save2gb (`save_over_2gb = 1` no `.cfg`, ligado por padrão): as três rotinas do contêiner (`zopen` 0x9326e0, `zseek` 0x933080, `ztell` 0x933170) vão para as de `src/conteiner.rs`, com a mesma assinatura da `save/`. Assim quem usa o save2gb troca uma DLL pela outra, sem precisar encadear duas `ddraw.dll`. Cada parte confere a sua assinatura e liga sozinha. Na cópia do repositório separado, o `conteiner.rs` vai em `src/` e o `#[path]` sai, como no save2gb.

**Estado: protótipo para discussão.** Compila e confere a assinatura, mas ainda não rodou no jogo. A curva do `regens.cfg` é um ponto de partida, não uma proposta fechada.

## Arquivos

| Arquivo | O quê |
|---|---|
| `src/lib.rs` | A DLL: repassa o `DirectDrawCreate` (a `ddraw_chain.dll` ou a do Windows), lê `regens.cfg`, confere as assinaturas e desvia o contêiner do save e a 0x7ad0e0 |
| `regens.cfg` | As regras; vai ao lado do exe. `rule = original` desliga tudo |
| `tools/calibrar.py` | Simulador fora do jogo: os clubes reais de um país, lidos de um save, disputam várias temporadas de jovens pela regra original e pela nova |

Compilar: `cargo build --release` (sai em `target/i686-pc-windows-msvc/release/ddraw.dll`).

Assinatura (FNV-1a dos trechos de que depende: a entrada da 0x7ad0e0, a chamada dela e o começo das rotinas que a nova chama) [observado]:
- **bate** no `cm0102.exe`, no `cm0102-1080.exe`, no portátil e nos três exes do CLZ;
- **não bate** no `cm0102_GDI.exe`, que tem outro leiaute.

O corpo da 0x7ad0e0 fica fora da assinatura, então o exe com o Regen Tweak (0x7ad1bd) também serve.

## A regra nova

Os clubes elegíveis são os mesmos da original:
- o jovem não é estrangeiro no clube;
- não é time B;
- não é vetado pela regra basca;
- o clube pode chegar ao jogador;
- elenco abaixo do teto.

Cada clube recebe um **peso**, e o clube é sorteado **na proporção do peso**, pelo gerador do próprio jogo. Peso nenhum: o jovem fica sem clube, como na original.

    peso = (nível × instalações × comissão × vaga) ^ temperatura

| Fator | Conta | Ajustes |
|---|---|---|
| Nível | curva em sino da reputação do clube em torno do alvo = target_k × PA (ou a mistura com o CA) | `target_k`, `target_ca`, `sigma`, `level_floor` |
| Instalações | facilities_base + facilities_k × Training | `facilities_base`, `facilities_k` |
| Comissão | de staff_min a staff_max pelo melhor "Working with Youngsters" entre o técnico e os 5 treinadores. Vale para a IA e para o humano igual | `staff_min`, `staff_max` |
| Vaga | ((squad_limit − elenco) / squad_limit) ^ squad_exp; com o elenco no teto, peso 0 | `squad_limit`, `squad_exp` |
| Teto por clube | peso 0 se o clube já recebeu `season_cap` regens nos últimos 365 dias (contados no próprio save: o bit do regen e a data de entrada no clube; sem dado novo no save) | `season_cap` |
| Temperatura | expoente: 1 proporcional, maior mais previsível, menor mais aleatório | `temperature` |

Com `log = 1`, cada escolha vai para `regens.log`: PA, clube, reputação, Training, chance e quantos disputaram.

## A calibração (Inglaterra, `TesteTelas.sav`, 200 temporadas simuladas)

Parâmetros:
- 687 clubes;
- 52 jovens por temporada na disputa (no `Valencia.sav`, a Inglaterra teve ~88 regens por temporada, e ~56% entram na disputa);
- PA sorteado entre os jogadores ingleses de 31 anos ou mais;
- o elenco volta ao do save a cada temporada [simplificação].

| Reputação | Clubes | Original: jovens por clube por temporada | Nova (padrão) | Original: PA ≥ 140 | Nova: PA ≥ 140 |
|---|---|---|---|---|---|
| 0–999 | 499 | 0,00 | 0,03 | 0,000 | 0,000 |
| 1000–1999 | 83 | 0,09 | 0,05 | 0,000 | 0,000 |
| 2000–2999 | 24 | 0,40 | 0,14 | 0,000 | 0,001 |
| 3000–3999 | 21 | 0,28 | 0,28 | 0,002 | 0,004 |
| 4000–4999 | 27 | 0,22 | 0,42 | 0,020 | 0,019 |
| 5000–5999 | 13 | 0,27 | 0,40 | 0,038 | 0,039 |
| 6000–6999 | 13 | 0,54 | 0,37 | 0,083 | 0,068 |
| 7000–7999 | 3 | 1,26 | 0,31 | 0,110 | 0,140 |
| 8000–8999 | 2 | 1,45 | 0,21 | 0,083 | 0,102 |
| 9000+ | 2 | 1,96 | 0,09 | 0,138 | 0,062 |

Na original, Liverpool, Arsenal e Manchester United recebem ~2 por temporada cada, a maioria jovens comuns. Na nova, os grandes recebem menos jovens no total, mas a parte deles nos de PA ≥ 140 é parecida (menos no 9000+, porque esses estão com 47 a 48 no elenco). Os clubes médios, de 4000 a 6999, passam a receber mais.

Duas tentativas descartadas [observado no simulador]:
- **Piso de nível de 5% e teto de elenco de 45:** os grandes zeraram (estavam acima de 45) e uns 30 dos 52 jovens foram para os 499 clubes amadores. Com centenas de clubes pequenos, um piso pequeno somado vira a maior fatia do sorteio.
- **Temperatura 2:** concentra mais nos clubes do tamanho certo; os de 7000+ ficam com o dobro dos PA ≥ 140.

Rodar: `python tools/calibrar.py TesteTelas.sav Inglaterra --humano Fulham --temporadas 200 [--cfg outro.cfg]`.

## Para discutir

1. **O alvo.** Onde o jovem "deveria" ir: pelo nível que vai ter (PA, o padrão) ou pelo que tem hoje (CA, `target_ca`)? O alvo de 50 × PA põe um jovem de PA 180 num clube de 9000.
2. **A largura (`sigma`).** Com 1500, um clube a 3000 do alvo fica com ~14% do peso máximo. Mais estreita: menos surpresa.
3. **Os grandes e os craques.** Quanto dos jovens de PA alto os grandes devem levar? Hoje a nova regra mantém a parte deles parecida com a original.
4. **O peso das instalações.** Hoje Training 20 vale 2,7 vezes o Training 1 (1,5 contra 0,55). Mais forte?
5. **A comissão.** Hoje vai de 0,8 a 1,2. O humano entra igual à IA, pelos treinadores. Mais forte?
6. **O teto por clube** (3 por 365 dias) e o teto de elenco (50).
7. **A aleatoriedade (`temperature`).**
8. **Fora do protótipo:**
   - os ~44% de jovens que nascem sem clube poderiam passar pelo mesmo sorteio com peso menor (um ajuste depois da fila diária, 0x7a65d0);
   - o PA do jovem poderia variar alguns pontos pelo Training do clube que o recebe (ao assinar, 0x7a6f10).

## Próximos passos

- Fechar a curva na discussão.
- Prova no jogo: carreira automática de férias na bancada, com o exe original e esta DLL, comparando a distribuição por faixa (`tools/regens_por_clube.py`) com a original.
- Teste com o NickPatcher e o CLZ.
