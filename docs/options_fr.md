# 🤖Configurations et options prises en charge

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

[![en-icon]](./options_en.md)
[![zh-hans-icon]](./options_zh-Hans.md)
[![fr-icon]](./options_fr.md)
[![de-icon]](./options_de.md)
[![nl-icon]](./options_nl.md)

**Symboles :** ✅ - Pris en charge, ❌ - Non pris en charge, 📌 - Prévu

## Ollama ✅

### Configurations de l'API

| Champ | Description |
| - | - |
| Point de terminaison | Le point de terminaison pour votre API Ollama. |
| Modèle | Le modèle à utiliser. |

### Options de conversation

| Option | Description | Pris en charge |
| - | - | - |
| num_ctx | Nombre de jetons d'entrée. Définit la taille de la fenêtre de contexte utilisée pour générer le jeton suivant. (Par défaut : 2048) | ✅ |
| num-predict | Nombre de jetons de sortie. Nombre maximum de jetons à prédire lors de la génération de texte. (Par défaut : 128, -1 = génération infinie, -2 = remplir le contexte) | ✅ |
| temperature | La température du modèle. Augmenter la température rendra les réponses du modèle plus créatives. (Par défaut : 0.8) | ✅ |
| top_p | Fonctionne avec top-k. Une valeur plus élevée (par ex., 0.95) conduira à un texte plus diversifié, tandis qu'une valeur plus basse (par ex., 0.5) générera un texte plus ciblé et conservateur. (Par défaut : 0.9) | ✅ |
| mirostat | Active l'échantillonnage Mirostat pour contrôler la perplexité. (par défaut : 0, 0 = désactivé, 1 = Mirostat, 2 = Mirostat 2.0) | 📌 |
| mirostat_eta | Influence la rapidité de réaction de l'algorithme au feedback du texte généré. Un taux d'apprentissage plus bas entraînera des ajustements plus lents, tandis qu'un taux plus élevé rendra l'algorithme plus réactif. (Par défaut : 0.1) | 📌 |
| mirostat_tau | Contrôle l'équilibre entre cohérence et diversité de la sortie. Une valeur plus basse donnera un texte plus ciblé et cohérent. (Par défaut : 5.0) | 📌 |
| repeat_last_n | Définit jusqu'où le modèle doit regarder en arrière pour éviter la répétition. (Par défaut : 64, 0 = désactivé, -1 = num_ctx) | 📌 |
| repeat_penalty | Définit l'intensité de la pénalité pour les répétitions. Une valeur plus élevée (par ex., 1.5) pénalisera plus fortement les répétitions, tandis qu'une valeur plus basse (par ex., 0.9) sera plus tolérante. (Par défaut : 1.1) | 📌 |
| seed | Définit la graine aléatoire à utiliser pour la génération. La définition d'un nombre spécifique fera générer le même texte pour le même prompt. (Par défaut : 0) | 📌 |
| stop | Définit les séquences d'arrêt à utiliser. Lorsque ce motif est rencontré, le LLM arrêtera de générer du texte et retournera. Plusieurs motifs d'arrêt peuvent être définis en spécifiant plusieurs paramètres stop séparés dans un modelfile. | 📌 |
| tfs_z | L'échantillonnage sans queue est utilisé pour réduire l'impact des jetons moins probables de la sortie. Une valeur plus élevée (par ex., 2.0) réduira davantage l'impact, tandis qu'une valeur de 1.0 désactive ce paramètre. (par défaut : 1) | 📌 |
| top_k | Réduit la probabilité de générer du non-sens. Une valeur plus élevée (par ex., 100) donnera des réponses plus diverses, tandis qu'une valeur plus basse (par ex., 10) sera plus conservatrice. (Par défaut : 40) | 📌 |
| min_p | Alternative au top_p, vise à assurer un équilibre entre qualité et variété. Le paramètre p représente la probabilité minimale pour qu'un jeton soit considéré, par rapport à la probabilité du jeton le plus probable. Par exemple, avec p=0.05 et le jeton le plus probable ayant une probabilité de 0.9, les logits avec une valeur inférieure à 0.045 sont filtrés. (Par défaut : 0.0) | 📌 |

### Références
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square
[fr-icon]: https://img.shields.io/badge/Français-teal?style=flat-square
[de-icon]: https://img.shields.io/badge/Deutsch-teal?style=flat-square
[nl-icon]: https://img.shields.io/badge/Nederlands-teal?style=flat-square
