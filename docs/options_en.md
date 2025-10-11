# 🤖Supported configs & options

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

[![en-icon]](./options_en.md)
[![zh-hans-icon]](./options_zh-Hans.md)
[![fr-icon]](./options_fr.md)
[![de-icon]](./options_de.md)
[![nl-icon]](./options_nl.md)

**Symbols:** ✅ - Supported, ❌ - Not supported, 📌 - Plan to support

## Ollama ✅

### API configurations

| Field | Description |
| - | - |
| Endpoint | The endpoint for your Ollama API. |
| Model | The model to use. |

### Conversation options

| Option | Description | Supported |
| - | - | - |
| num_ctx | Number of input tokens. Sets the size of the context window used to generate the next token. (Default: 2048) | ✅ |
| num-predict | Number of output tokens. Maximum number of tokens to predict when generating text. (Default: 128, -1 = infinite generation, -2 = fill context) | ✅ |
| temperature | The temperature of the model. Increasing the temperature will make the model answer more creatively. (Default: 0.8)  | ✅ |
| top_p | Works together with top-k. A higher value (e.g., 0.95) will lead to more diverse text, while a lower value (e.g., 0.5) will generate more focused and conservative text. (Default: 0.9) | ✅ |
| mirostat | Enable Mirostat sampling for controlling perplexity. (default: 0, 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0) | 📌 |
| mirostat_eta | Influences how quickly the algorithm responds to feedback from the generated text. A lower learning rate will result in slower adjustments, while a higher learning rate will make the algorithm more responsive. (Default: 0.1) | 📌 |
| mirostat_tau | Controls the balance between coherence and diversity of the output. A lower value will result in more focused and coherent text. (Default: 5.0) | 📌 |
| repeat_last_n | Sets how far back for the model to look back to prevent repetition. (Default: 64, 0 = disabled, -1 = num_ctx) | 📌 |
| repeat_penalty | Sets how strongly to penalize repetitions. A higher value (e.g., 1.5) will penalize repetitions more strongly, while a lower value (e.g., 0.9) will be more lenient. (Default: 1.1) | 📌 |
| seed | Sets the random number seed to use for generation. Setting this to a specific number will make the model generate the same text for the same prompt. (Default: 0) | 📌 |
| stop | Sets the stop sequences to use. When this pattern is encountered the LLM will stop generating text and return. Multiple stop patterns may be set by specifying multiple separate stop parameters in a modelfile. | 📌 |
| tfs_z | Tail free sampling is used to reduce the impact of less probable tokens from the output. A higher value (e.g., 2.0) will reduce the impact more, while a value of 1.0 disables this setting. (default: 1) | 📌 |
| top_k | Reduces the probability of generating nonsense. A higher value (e.g. 100) will give more diverse answers, while a lower value (e.g. 10) will be more conservative. (Default: 40) | 📌 |
| min_p | Alternative to the top_p, and aims to ensure a balance of quality and variety. The parameter p represents the minimum probability for a token to be considered, relative to the probability of the most likely token. For example, with p=0.05 and the most likely token having a probability of 0.9, logits with a value less than 0.045 are filtered out. (Default: 0.0) | 📌 |

### References
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square
[fr-icon]: https://img.shields.io/badge/Français-teal?style=flat-square
[de-icon]: https://img.shields.io/badge/Deutsch-teal?style=flat-square
[nl-icon]: https://img.shields.io/badge/Nederlands-teal?style=flat-square
