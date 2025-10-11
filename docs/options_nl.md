# 🤖Ondersteunde configuraties & opties

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

[![en-icon]](./options_en.md)
[![zh-hans-icon]](./options_zh-Hans.md)
[![fr-icon]](./options_fr.md)
[![de-icon]](./options_de.md)
[![nl-icon]](./options_nl.md)

**Symbolen:** ✅ - Ondersteund, ❌ - Niet ondersteund, 📌 - Gepland

## Ollama ✅

### API-configuraties

| Veld | Beschrijving |
| - | - |
| Eindpunt | Het eindpunt voor uw Ollama-API. |
| Model | Het te gebruiken model. |

### Conversatie-opties

| Optie | Beschrijving | Ondersteund |
| - | - | - |
| num_ctx | Aantal invoer-tokens. Stelt de grootte van het contextvenster in dat wordt gebruikt om het volgende token te genereren. (Standaard: 2048) | ✅ |
| num-predict | Aantal uitvoer-tokens. Maximaal aantal tokens dat wordt voorspeld bij het genereren van tekst. (Standaard: 128, -1 = oneindige generatie, -2 = context vullen) | ✅ |
| temperature | De temperatuur van het model. Een hogere temperatuur zorgt ervoor dat het model creatiever antwoordt. (Standaard: 0.8) | ✅ |
| top_p | Werkt samen met top-k. Een hogere waarde (bijv. 0,95) leidt tot meer diverse tekst, terwijl een lagere waarde (bijv. 0,5) meer gefocuste en conservatieve tekst genereert. (Standaard: 0.9) | ✅ |
| mirostat | Schakel Mirostat-sampling in voor het beheersen van perplexiteit. (standaard: 0, 0 = uitgeschakeld, 1 = Mirostat, 2 = Mirostat 2.0) | 📌 |
| mirostat_eta | Beïnvloedt hoe snel het algoritme reageert op feedback van de gegenereerde tekst. Een lagere leersnelheid resulteert in langzamere aanpassingen, terwijl een hogere leersnelheid het algoritme responsiever maakt. (Standaard: 0.1) | 📌 |
| mirostat_tau | Regelt het evenwicht tussen samenhang en diversiteit van de uitvoer. Een lagere waarde resulteert in meer gefocuste en samenhangende tekst. (Standaard: 5.0) | 📌 |
| repeat_last_n | Stelt in hoe ver het model terugkijkt om herhaling te voorkomen. (Standaard: 64, 0 = uitgeschakeld, -1 = num_ctx) | 📌 |
| repeat_penalty | Stelt in hoe sterk herhalingen worden bestraft. Een hogere waarde (bijv. 1,5) bestraft herhalingen sterker, terwijl een lagere waarde (bijv. 0,9) toleranter is. (Standaard: 1.1) | 📌 |
| seed | Stelt de seed voor het willekeurig getal in voor generatie. Als dit op een specifiek getal wordt ingesteld, genereert het model dezelfde tekst voor dezelfde prompt. (Standaard: 0) | 📌 |
| stop | Stelt de te gebruiken stopsequenties in. Wanneer dit patroon wordt aangetroffen, stopt het LLM met het genereren van tekst en keert terug. Meerdere stoppatronen kunnen worden ingesteld door meerdere afzonderlijke stopparameters op te geven in een modelbestand. | 📌 |
| tfs_z | Tail-free sampling wordt gebruikt om de impact van minder waarschijnlijke tokens op de uitvoer te verminderen. Een hogere waarde (bijv. 2,0) vermindert de impact meer, terwijl een waarde van 1,0 deze instelling uitschakelt. (standaard: 1) | 📌 |
| top_k | Vermindert de kans op het genereren van onzin. Een hogere waarde (bijv. 100) geeft meer diverse antwoorden, terwijl een lagere waarde (bijv. 10) conservatiever is. (Standaard: 40) | 📌 |
| min_p | Alternatief voor top_p, met als doel een evenwicht tussen kwaliteit en variatie te waarborgen. De parameter p vertegenwoordigt de minimale waarschijnlijkheid dat een token in overweging wordt genomen, relatief aan de waarschijnlijkheid van het meest waarschijnlijke token. Bijvoorbeeld, met p=0,05 en het meest waarschijnlijke token met een waarschijnlijkheid van 0,9, worden logits met een waarde lager dan 0,045 uitgefilterd. (Standaard: 0.0) | 📌 |

### Referenties
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square
[fr-icon]: https://img.shields.io/badge/Français-teal?style=flat-square
[de-icon]: https://img.shields.io/badge/Deutsch-teal?style=flat-square
[nl-icon]: https://img.shields.io/badge/Nederlands-teal?style=flat-square
