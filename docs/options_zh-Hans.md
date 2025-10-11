# 🤖支持的配置项

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

[![en-icon]](./options_en.md)
[![zh-hans-icon]](./options_zh-Hans.md)
[![fr-icon]](./options_fr.md)
[![de-icon]](./options_de.md)
[![nl-icon]](./options_nl.md)

**符号:** ✅ - 支持, ❌ - 不支持, 📌 - 计划支持

## Ollama ✅

### API 配置

| 字段 | 描述 |
| - | - |
| Endpoint | Ollama API 的端点。 |
| Model | 要使用的模型。 |

### 对话选项

| 选项 | 描述 | 支持 |
| - | - | - |
| num_ctx | 输入令牌数。设置用于生成下一个令牌的上下文窗口大小。（默认值：2048） | ✅ |
| num-predict | 输出令牌数。生成文本时预测的最大令牌数。（默认值：128，-1 = 无限生成，-2 = 填充上下文） | ✅ |
| temperature | 模型的温度。增加温度将使模型回答更具创造性。（默认值：0.8） | ✅ |
| top_p | 与 top-k 一起工作。较高的值（如 0.95）将产生更多样化的文本，而较低的值（如 0.5）将生成更集中和保守的文本。（默认值：0.9） | ✅ |
| mirostat | 启用 Mirostat 采样以控制困惑度。（默认值：0，0 = 禁用，1 = Mirostat，2 = Mirostat 2.0） | 📌 |
| mirostat_eta | 影响算法对生成文本反馈的响应速度。较低的学习率将导致调整较慢，而较高的学习率将使算法更加敏感。（默认值：0.1） | 📌 |
| mirostat_tau | 控制输出的连贯性和多样性之间的平衡。较低的值将产生更集中和连贯的文本。（默认值：5.0） | 📌 |
| repeat_last_n | 设置模型回看以防止重复的距离。（默认值：64，0 = 禁用，-1 = num_ctx） | 📌 |
| repeat_penalty | 设置惩罚重复的强度。较高的值（如 1.5）将更强烈地惩罚重复，而较低的值（如 0.9）将更宽松。（默认值：1.1） | 📌 |
| seed | 设置用于生成的随机数种子。将其设置为特定数字将使模型为相同的提示生成相同的文本。（默认值：0） | 📌 |
| stop | 设置要使用的停止序列。当遇到此模式时，LLM 将停止生成文本并返回。可以通过在模型文件中指定多个单独的 stop 参数来设置多个停止模式。 | 📌 |
| tfs_z | 尾部自由采样用于减少较不可能的令牌对输出的影响。较高的值（如 2.0）将减少更多影响，而值为 1.0 则禁用此设置。（默认值：1） | 📌 |
| top_k | 降低生成无意义内容的概率。较高的值（如 100）将给出更多样化的答案，而较低的值（如 10）将更保守。（默认值：40） | 📌 |
| min_p | top_p 的替代方案，旨在确保质量和多样性的平衡。参数 p 表示相对于最可能令牌的概率，令牌被考虑的最小概率。例如，当 p=0.05 且最可能的令牌概率为 0.9 时，值小于 0.045 的对数将被过滤掉。（默认值：0.0） | 📌 |

### 参考资料
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square
[fr-icon]: https://img.shields.io/badge/Français-teal?style=flat-square
[de-icon]: https://img.shields.io/badge/Deutsch-teal?style=flat-square
[nl-icon]: https://img.shields.io/badge/Nederlands-teal?style=flat-square
