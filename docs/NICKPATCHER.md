# Para quem usa o NickPatcher: o que convive e o que não

*English version: [NICKPATCHER.en.md](NICKPATCHER.en.md)*

A compatibilidade do Tweaks com um exe alterado pelo NickPatcher (ou pelos patches da comunidade que vêm com ele) vale **até onde as alterações aplicadas permitem**. Antes de ligar, cada ajuste confere no exe o trecho que usa. Se uma opção do NickPatcher já mudou esse trecho, o ajuste não liga por cima: fica com a versão do exe e avisa no `tweaks.log` e no configurador. Os outros ajustes ligam normalmente, e o jogo nunca é interrompido por isso. Com `override_exe_changes = 1`, os ajustes das idades, da disputa, dos inativos e do save trocam a versão do exe pela do Tweaks, e o efeito daquela opção do NickPatcher se perde.

O quadro abaixo cruza as opções do NickPatcher com os trechos de cada ajuste, a partir dos próprios dados do NickPatcher (as opções do `Patcher.cs` e os patches do `MiscPatches.zip`, lidos com `tools/exe/nickpatcher_vs_tweaks.py`).

## Opções que mexem no mesmo lugar (uma ou outra)

| Opção do NickPatcher | Ajuste do Tweaks atingido | Sem override | Com `override_exe_changes = 1` |
|---|---|---|---|
| **Regen fixes** (opção do NickPatcher) ou **Regen tweak 2** (Saturn): não manda regens a clubes com 45 jogadores ou mais | a disputa dos clubes (`rule = new`) | fica a disputa do exe (com o limite de 45); a nossa não liga | liga a nossa disputa (o limite de elenco passa a ser o nosso `squad_limit`) |
| **Regen fixes** ou **Younger Regens** (Saturn): pulam o sorteio de idade das nações fora do jogo | `ages_out_of_game` | fica o do exe | o nosso desvio entra, mas o ramo continua pulado: na prática, igual ao do exe |
| **Regen CA level reset** (Saturn): muda as faixas de idade e o nível de CA dos regens | as idades (`ages`) | ficam as do exe (as faixas do Saturn) | liga a nossa tabela de idades; o "CA level reset" do Saturn deixa de valer |
| **Android11Patch**: faz o jogo rodar no Android 11 (muda a rotina que abre o save) | o save acima de 2 GB (`save_over_2gb`) | fica a rotina do exe: o save acima de 2 GB não liga | liga o nosso save; a correção do Android se perde (não use no Android) |
| **1280x800** e as outras resoluções do NickPatcher, o **Tapani Space Maker**, o **Wales Patch**, o **Fix World Cup pre-2000** e o **English League North relegation**: usam o mesmo espaço do exe que a nossa resolução | `resolution = 1080` | a nossa resolução não liga (a do exe fica) | — (a resolução não tem override: metade das trocas daria telas quebradas) |

## Opções que convivem, mas mudam o resultado

| Opção do NickPatcher | O que acontece com o Tweaks |
|---|---|
| **RetirePatchV4**: ninguém se aposenta antes dos 34 | a aposentadoria dos inativos (`retire_idle_players`) liga, mas quase ninguém chega a ela; menos regens novos |
| **NoMoreRetirements**: ninguém se aposenta | sem aposentadorias não nascem regens: as idades, a disputa, `unattached` e `regen_origins` ficam sem nada para fazer |
| **AlwaysRegenToNewGoodCountry**: o regen de PA 120 ou mais muda para uma nação forte | convive; a disputa usa a nação nova, e o `regen_origins` registra a nação final do regen |
| **Tapani's new regen code** (`tapaninewregencode`, **TurnOffTapaniRegens**, **Regens mega boost**): outro gerador de atributos no fim da criação do regen | convive; as nossas idades, a disputa e o `regen_origins` agem antes ou ao redor; os atributos finais são os do código do Tapani |
| **UseCommonNamesForBrazilRegens**, **IncreasePregensMultiplier** | convivem (nomes; jogadores gerados no início da carreira) |
| **NoCD** do NickPatcher | convive com o nosso `no_cd`: as checagens que o exe já tem ficam, a DLL completa as outras |

## A NickDLL (pacotes como o CLZ)

A NickDLL muda o exe em memória, depois de abrir. As que trazem a "resolução ampliada" (`"resolucao"` no `NickDLL_config.json`) cuidam da resolução: com uma resolução escolhida lá, a nossa `resolution` fica de fora. Ela também troca a rotina do time B (a disputa usa a dela, sem problema) e pode desviar a caixa de mensagem (o nosso `log` encadeia; a confirmar no jogo). Detalhes em [COMPATIBILIDADE.md](COMPATIBILIDADE.md).

**Os regens bascos e a NickDLL mais nova (novembro de 2026).** A disputa do Tweaks (`rule = new`) tem a regra dela para os regens bascos: basco pela primeira ou pela segunda nacionalidade, e os 5 clubes da original (Athletic e o B, Real Sociedad e o B, Alavés) mais os que o próprio jogo marca como "só contrata bascos" (Eibar, Barakaldo, Sestao e mais 18); na escolha do clube do regen, o basco não é estrangeiro na Espanha. Testamos com a NickDLL mais nova: as duas convivem. Com `rule = original`, a escolha fica com o exe e com o que estiver ao lado dele.

**NickPatcher v2.33** (04/11/2026): só copas internacionais e as sedes de 2026; nada em comum com os trechos do Tweaks.

## Como saber o que vale no seu exe

Abra o `tweaks-config.exe` na pasta do jogo: a aba Início mostra cada ajuste como **compatível**, **atenção: sobrescreve** (com o nome da opção do NickPatcher), **fica com o outro** ou **não compatível**, e a faixa amarela no topo avisa quando algum ajuste ligado não combina com o seu exe.
