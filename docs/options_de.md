# 🤖Unterstützte Konfigurationen & Optionen

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

[![en-icon]](./options_en.md)
[![zh-hans-icon]](./options_zh-Hans.md)
[![fr-icon]](./options_fr.md)
[![de-icon]](./options_de.md)
[![nl-icon]](./options_nl.md)

**Symbole:** ✅ - Unterstützt, ❌ - Nicht unterstützt, 📌 - Geplant

## Ollama ✅

### API-Konfigurationen

| Feld | Beschreibung |
| - | - |
| Endpunkt | Der Endpunkt für Ihre Ollama-API. |
| Modell | Das zu verwendende Modell. |

### Konversationsoptionen

| Option | Beschreibung | Unterstützt |
| - | - | - |
| num_ctx | Anzahl der Eingabe-Tokens. Legt die Größe des Kontextfensters fest, das zur Generierung des nächsten Tokens verwendet wird. (Standard: 2048) | ✅ |
| num-predict | Anzahl der Ausgabe-Tokens. Maximale Anzahl von Tokens, die bei der Textgenerierung vorhergesagt werden. (Standard: 128, -1 = unendliche Generierung, -2 = Kontext füllen) | ✅ |
| temperature | Die Temperatur des Modells. Eine Erhöhung der Temperatur führt zu kreativeren Antworten des Modells. (Standard: 0.8) | ✅ |
| top_p | Funktioniert zusammen mit top-k. Ein höherer Wert (z.B. 0,95) führt zu vielfältigerem Text, während ein niedrigerer Wert (z.B. 0,5) fokussierteren und konservativeren Text erzeugt. (Standard: 0.9) | ✅ |
| mirostat | Aktiviert Mirostat-Sampling zur Kontrolle der Perplexität. (Standard: 0, 0 = deaktiviert, 1 = Mirostat, 2 = Mirostat 2.0) | 📌 |
| mirostat_eta | Beeinflusst, wie schnell der Algorithmus auf Feedback aus dem generierten Text reagiert. Eine niedrigere Lernrate führt zu langsameren Anpassungen, während eine höhere Lernrate den Algorithmus reaktionsschneller macht. (Standard: 0.1) | 📌 |
| mirostat_tau | Steuert das Gleichgewicht zwischen Kohärenz und Vielfalt der Ausgabe. Ein niedrigerer Wert führt zu fokussierterem und kohärenterem Text. (Standard: 5.0) | 📌 |
| repeat_last_n | Legt fest, wie weit das Modell zurückblickt, um Wiederholungen zu vermeiden. (Standard: 64, 0 = deaktiviert, -1 = num_ctx) | 📌 |
| repeat_penalty | Legt fest, wie stark Wiederholungen bestraft werden. Ein höherer Wert (z.B. 1,5) bestraft Wiederholungen stärker, während ein niedrigerer Wert (z.B. 0,9) nachsichtiger ist. (Standard: 1.1) | 📌 |
| seed | Legt den Zufallszahlenseed für die Generierung fest. Wenn dies auf eine bestimmte Zahl gesetzt wird, generiert das Modell denselben Text für denselben Prompt. (Standard: 0) | 📌 |
| stop | Legt die zu verwendenden Stoppsequenzen fest. Wenn dieses Muster erkannt wird, stoppt das LLM die Textgenerierung und kehrt zurück. Mehrere Stoppmuster können durch Angabe mehrerer separater Stoppparameter in einer Modelldatei festgelegt werden. | 📌 |
| tfs_z | Tail-free Sampling wird verwendet, um den Einfluss weniger wahrscheinlicher Tokens auf die Ausgabe zu reduzieren. Ein höherer Wert (z.B. 2,0) reduziert den Einfluss stärker, während ein Wert von 1,0 diese Einstellung deaktiviert. (Standard: 1) | 📌 |
| top_k | Reduziert die Wahrscheinlichkeit, Unsinn zu generieren. Ein höherer Wert (z.B. 100) führt zu vielfältigeren Antworten, während ein niedrigerer Wert (z.B. 10) konservativer ist. (Standard: 40) | 📌 |
| min_p | Alternative zu top_p, die darauf abzielt, ein Gleichgewicht zwischen Qualität und Vielfalt sicherzustellen. Der Parameter p repräsentiert die minimale Wahrscheinlichkeit, mit der ein Token berücksichtigt wird, relativ zur Wahrscheinlichkeit des wahrscheinlichsten Tokens. Zum Beispiel werden bei p=0,05 und dem wahrscheinlichsten Token mit einer Wahrscheinlichkeit von 0,9 Logits mit einem Wert unter 0,045 herausgefiltert. (Standard: 0.0) | 📌 |

### Referenzen
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square
[fr-icon]: https://img.shields.io/badge/Français-teal?style=flat-square
[de-icon]: https://img.shields.io/badge/Deutsch-teal?style=flat-square
[nl-icon]: https://img.shields.io/badge/Nederlands-teal?style=flat-square
