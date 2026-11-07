# Compatibilidade com outros patches

*English version: [COMPATIBILIDADE.en.md](COMPATIBILIDADE.en.md)*

O que cada ajuste do Tweaks faz quando o exe, ou uma DLL ao lado dele, já mudou o mesmo trecho. **A regra é fazer o possível:** cada ajuste confere o seu trecho sozinho, e o que não serve fica de fora com o motivo no `tweaks.log`. Os outros ajustes ligam normalmente, e o jogo nunca é interrompido por isso. **[observado]** = visto no código, nos bytes ou no log; **[inferência]** = a confirmar no jogo.

Para quem usa o NickPatcher, o aviso opção por opção está em [NICKPATCHER.md](NICKPATCHER.md).

## 1. Os estados

Os mesmos que o configurador vai mostrar ([CONFIGURADOR.md](CONFIGURADOR.md)):

| Estado | Quando | Exemplo |
|---|---|---|
| ✅ compatível | o trecho está como no 3.9.68, ou o exe já tem esta mesma mudança | o save acima de 2 GB em qualquer exe |
| ⚠️ sobrescreve | o exe tem uma mudança **conhecida** no trecho, e `override_exe_changes = 1` manda trocá-la pela nossa | a disputa sobre o Regen Tweak |
| 🔁 fica com o outro | uma mudança **conhecida** faz o mesmo papel; a nossa fica de fora, e a do exe ou da outra DLL vale | a resolução da NickDLL do CLZ 6.1; o CD da LaMOHatesCD |
| ❓ desconhecido | o trecho difere do 3.9.68 de um jeito que o catálogo não reconhece; só este ajuste fica de fora | um patch novo de alguém |

"Conhecido" quer dizer nomeado pelo retrato do exe (`src/exe/retrato.rs`, os casos conhecidos), pelo catálogo do NickPatcher (`tools/exe/retrato_exe.py`) ou por este documento.

## 2. A matriz

| Ajuste | 3.9.68 (e os exes do projeto CM0102) | Regen Tweak (xeno/Saturn) | CLZ 6.1: exe (NickPatcher) | CLZ 6.1: NickDLL e LaMOHatesCD |
|---|---|---|---|---|
| `save_over_2gb` | ✅ | ✅ | ✅ | ✅ a NickDLL chama a 0x933610 (outra rotina do contêiner), que o ajuste não troca |
| `portable_folder`, `clear_admin_flag` | ✅ | ✅ | ✅ | ✅ feitos para o CLZ |
| `ages` | ✅ | ✅ | 🔁 o NickPatcher mudou as faixas raras de idade; ⚠️ com override 1 | ✅ |
| `ages_out_of_game` | ✅ | ✅ | 🔁 o ramo foi pulado no exe; ⚠️ com override 1 (o ramo continua sem rodar) | ✅ |
| `rule = new` (a disputa) | ✅ | 🔁 o Regen Tweak (elenco 45); ⚠️ com override 1 | 🔁 / ⚠️ igual | ✅ a NickDLL troca a rotina do time B (0x540a50) pela dela, com a lista do banco; a disputa a usa como usaria a do exe |
| `retire_idle_players` | ✅ | ✅ | ✅ liga, mas o RetirePatchV4 não deixa ninguém com menos de 34 chegar à regra (o log avisa) | ✅ |
| `unattached` | ✅ | ✅ | ✅ | ✅ |
| `no_cd` | ✅ | ✅ | ✅ por checagem: o exe já tem 14 das 33; a DLL completa as outras | 🔁 com a `LaMOHatesCD.dll` ao lado, fica com ela |
| `resolution = 1080` | ✅ (no exe de 1080, "já tem") | ✅ | ✅ o exe tem 5 dos 201 trechos já trocados (o mesmo código do NickPatcher); a DLL completa os outros | 🔁 com a NickDLL do 6.1 e `"resolucao"` diferente de 800x600 no `NickDLL_config.json`, fica com ela |
| a trava das 30 linhas (com `resolution`, `video` ou a resolução da NickDLL) | ✅ | ✅ | ✅ | ✅ nenhuma correção da NickDLL toca o `Area::init` |
| `video = window` | ✅ | ✅ | ❓ não testado: o CLZ em janela com a `ddraw.dll` do Windows para em "16 bit colour"; a camada de vídeo deve resolver [inferência] | ❓ |
| `log = 1` (o registro das caixas, 0x5e8290) | ✅ | ✅ | ✅ | ⚠️ a LaMOHatesCD e a NickDLL do 6.1 (`hook_msgbox`) também desviam a 0x5e8290; o registro encadeia no desvio que já estiver lá; se a NickDLL instalar o dela depois e conferir os bytes originais, o dela pode se recusar [inferência, a confirmar no jogo] |

## 3. A NickDLL do CLZ 6.1

O que a NickDLL do CLZ 6.1 e a LaMOHatesCD mudam nos trechos que o Tweaks usa, observado pelo retrato em memória (as linhas `runtime` do `tweaks.log`) e nos testes:

| DLL | O que muda nos nossos trechos |
|---|---|
| NickDLL do 6.1 | **a resolução**, quando há uma `"resolucao"` escolhida no `NickDLL_config.json`: aí a nossa fica de fora (é uma ou outra); **a caixa de mensagem** (0x5e8290): o nosso registro das caixas encadeia; **o time B** (0x540a50): a disputa usa a rotina dela para deixar os times B de fora. Os outros ajustes não são tocados |
| LaMOHatesCD | só a caixa de mensagem |
