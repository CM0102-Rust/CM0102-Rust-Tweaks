# Mercado: transferências e empréstimos (estudo)

*English version: [MERCADO.en.md](MERCADO.en.md)*

Estudo de 07/11/2026, no roadmap depois da fornada. Pedido do Lucas: o jogador que se destaca não ganha reputação suficiente para virar cobiçado pelo mercado, e o clube que faz sucesso demora a ficar interessante. Também as reclamações antigas do fórum sobre empréstimos ([EMPRESTIMOS.md](EMPRESTIMOS.md), [OPORTUNIDADES.md](OPORTUNIDADES.md) §2) e o clube rico parado com centenas de milhões.

Método, o mesmo dos regens: mapear no código (as rotinas já reescritas em Rust no projeto CM0102 e a desmontagem das outras), medir numa carreira automática com todas as ligas e só então propor chaves, com o padrão igual ao original. A reputação, que é o que move o mercado, está em [REPUTACAO_E_PUBLICO.md](REPUTACAO_E_PUBLICO.md).

**[observado]** = lido no código; **[medido]** = carreira automática; **[inferência]** = conclusão a provar.

## 1. Como a IA escolhe quem contratar

- **Os candidatos** vêm das listas de posição, refeitas **a cada 30 dias** e ordenadas pelo **valor** do jogador (+0x52, rotina 0x4d0c60: a habilidade vista pelo técnico × 50, a reputação, a idade, a posição e a nação) [observado].
- **A vitrine:** a 0x842c00 oferece um jogador disponível a até 5 clubes, mas só se a reputação atual dele for **5.500 ou mais** [observado].
- **A escolha** (0x83f3a0 e 0x840d80) usa a nota de seleção (posição, condição, forma, contrato, reputação), vista pelo técnico ou pelos olheiros, e sorteios de reputação para estrangeiros. **A nota média das partidas não entra**: o desempenho só pesa através da forma e da reputação [observado].
- **O clube** pesa pela reputação (+0x80) em quase tudo: o teto da verba (reputação abaixo de 3.500 limita a reputação × 500), o sorteio do lance (2 × reputação + 1.000), a vontade do jogador de ir e a reputação que o clube procura (0x83bf40) [observado].
- **O ritmo:** a lista de observação do clube só é refeita a cada 30 a 100 dias; uma recusa bloqueia a vaga por 120 dias [observado].

Então, para o mercado, **destacar-se é ganhar reputação**. É lá ([REPUTACAO_E_PUBLICO.md](REPUTACAO_E_PUBLICO.md) §3) que o jogo trava: a reputação do jogador anda devagar e tem teto pela liga.

## 2. Empréstimos

Correções ao [EMPRESTIMOS.md](EMPRESTIMOS.md): 0x8c8090, 0x846920 e 0x84a020 já estão em Rust no projeto CM0102, e **quem decide o empréstimo é a 0x84af00**, não a 0x8c8090. A oferta de empréstimo vencida vai, pelo `process_due_offer` (0x8c63a0) e pela resposta automática das férias (0x784100), direto para a 0x84af00; a recusa no topo da 0x8c8090 só é alcançável por chamadores de troca, sem empréstimo (código praticamente morto) [observado].

**A regra da 0x84af00** para um empréstimo [observado]. O clube da IA só empresta:
1. o jogador **listado para empréstimo** (bit 2 de contrato+0x4e), com a reputação de quem pede × 1,25 acima da reputação atual do jogador; ou
2. o jogador de estatuto 6 ou 7 (ou goleiro), com a reputação de quem pede acima de 0,75 × a do jogador, **moral abaixo de −50** e o dono fora da Liga dos Campeões.

Fora disso, recusa. **O salário e o período não entram.** E a IA só põe na lista de empréstimos o jogador do fim do elenco em má fase ou cansado (0x83cb40) [observado]: o jovem promissor que não joga quase nunca entra.

**"Não tem intenção de ir"** (0x8d8310 → 0x8d84e0): sai quando a nota de vontade (0x846920) fica abaixo de 1,0 [observado]. Num empréstimo ela parte de 2,75, e o bônus de reputação dos clubes só vale quando o jogador tem uma queixa (bit 0x10 de contrato+0x45). A IA só pede emprestado com nota 3,5 ou mais [observado].

**Presos no modo férias:** o despacho das ofertas (0x8c63a0) não tem ramo para os estados 8, 9, 0xb, 0xc, 0xd e 0x13. Uma oferta que estava esperando o humano (estado 9) quando ele entrou de férias fica parada para sempre [observado: a falta do ramo; inferência: o caso do fórum].

**Fim de contrato:** `get_loan_period` (0x8d0590) recua o fim do empréstimo até caber no contrato, e `can_loan_player` (0x8d0040) então não acha mês válido [observado].

## 3. O clube rico parado

A verba existe, mas passa por funis [observado]:
1. **A verba** (0x598800): o saldo menos as reservas, metade da sobra, teto de 40 milhões em algumas nações e compressão por nação (Alemanha, França, Escócia e Holanda acima de 17,5 milhões só 25% do excesso; Brasil acima de 2,5 milhões; os nórdicos e outros acima de 2,5 milhões só 10%). Inglaterra, Espanha e Itália sem compressão.
2. **A repartição** (0x83b6e0): só as 8 primeiras vagas recebem verba, dividida pelo número de vagas abertas × 2 a 10.
3. **Os candidatos** (0x83f3a0): preço × 1,3 dentro da verba da vaga, vontade 3,15 ou mais, e o salário pedido dentro de 1,2 × o teto salarial.
4. **O lance** (0x84a020): sorteio(valor) > sorteio(2 × reputação + 1.000).
5. **O ritmo:** lista refeita a cada 30 a 100 dias; 120 dias de bloqueio depois de uma recusa.

## 4. O que a carreira mediu

Os empréstimos e a verba foram medidos e viraram chaves: ver [EMPRESTIMOS.md](EMPRESTIMOS.md) §4. O resto do estudo das carreiras de 11 e 12/11/2026, com cada item testado sozinho:

- **O lance mais fácil** (o sorteio contra reputação + 1.000, não 2 × reputação + 1.000): +4% a 7% de atividade.
- **O destaque** (a vontade de um titular de ir para um clube bem maior, × 1,6): +22% de jogadores subindo na 2ª temporada.
- **O preço do titular absoluto** (× o pedido) e o **Business do presidente** na verba: dentro do ruído, ou só mudam a verba.
- **As correções da multa (0x86165b) e do empréstimo da lista de reservas (0x83e482)**: dentro do ruído.

## 5. Chaves

- **Empréstimos e trava da verba** (1.5): `loans`, as `loan_*` e `budget_caps`, em [EMPRESTIMOS.md](EMPRESTIMOS.md) §5.
- **As transferências** (o destaque, o lance, o interesse do mercado): o próximo estudo, com o mesmo método.
