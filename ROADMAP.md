# MVP Roadmap

| ✅   | **Task**                                       | **Description / Focus Area**                                                               |
| --- | ---------------------------------------------- | ------------------------------------------------------------------------------------------ |
| ✅ | **Remove HTTPS callbacks**                     | Guarantee all operations remain 100% local. No network callbacks or telemetry.             |
| ✅ | **Add full Dutch and German i18n coverage**    | Localize UI, error messages, and legal notices in Dutch and German.                        |
| [ ] | **Add GDPR compliance**                        | Implement local privacy features: data minimization, deletion, encryption, audit logs.     |
| [ ] | **Add AI Act compliance**                      | Include transparency labels, explainability, provenance, and edit-before-export workflows. |
| [ ] | **Add local model support with Candle and HF** | Run all inference offline with Candle + Hugging Face. Allow manual model downloads.        |
| [ ] | **Add PII Layer 1 regex**                      | Detect and redact names, emails, IDs using regex patterns.                                 |
| [ ] | **Add PII Layer 2 NER**                        | Add named-entity recognition for context-aware anonymization.                              |
| [ ] | **Add PII Layer 3 optional bridge**            | Optional advanced anonymization plug-in (e.g., Presidio or custom compliance bridge).      |
| [ ] | **Add encryption of chat history**             | Encrypt all chat logs, configs, and project data locally. Optionally lock via OS password. |

# Beyond MVP Roadmap (v2 and later)
| ✅   | **Task**                                | **Description / Focus Area**                                                                                                   |
| --- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| [ ] | **Add local search and RAG**            | Implement on-device retrieval-augmented generation for context-aware legal drafting, summaries, and precedent lookup.          |
| [ ] | **Add assistants**                      | Provide predefined professional profiles such as *Contract Reviewer*, *GDPR Advisor*, *Case Summarizer*, and *Tax Consultant*. |
| [ ] | **Add agent support**                   | Enable multi-step reasoning workflows like “Analyze → Redact → Summarize → Export.”                                            |
| [ ] | **Add projects**                        | Group chats, documents, and outputs under client or case folders with optional encryption and per-project metadata.            |
| [ ] | **Add payment (Mollie)**                | Integrate Mollie for subscription or lifetime license management with optional offline activation keys.                        |
| [ ] | **Add contract analysis toolkit**       | Local clause extraction, risk scoring (offline), and change suggestions for NDAs, DPAs, or lease contracts.                    |
| [ ] | **Add legal template generator**        | Offer structured templates for contracts, privacy policies, and letters, customizable via natural language.                    |
| [ ] | **Add document comparison & redlining** | Show tracked differences between document versions, highlighting legal term changes or risk keywords.                          |
| [ ] | **Add citation and reference linker**   | Automatically detect and hyperlink legal references (e.g., “Art. 6 GDPR”) to local copies of relevant laws.                    |
| [ ] | **Add compliance checklist assistant**  | Allow firms to create internal checklists (GDPR, ISO, or internal policies) and check document compliance automatically.       |
| [ ] | **Add timeline and evidence builder**   | Extract key events, names, and dates from documents to build a case timeline or evidence summary automatically.                |
| [ ] | **Add courtroom preparation mode**      | Convert factual case data into structured summaries or outlines for pleadings and notes.                                       |
| [ ] | **Add document bundle exporter**        | Generate fully redacted and paginated bundles with table of contents, citations, and AI-generated summaries.                   |

# GDPR compliance
| ✅   | **Requirement**                        | **Legal Article(s)** | **How to Do It Practically (Local-Only Context)**                                                                                                                        |
| --- | -------------------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [ ] | **Data Minimization**                  | Art. 5(1)(c)         | Store only files or chats the user explicitly creates or imports. Automatically delete temporary or cache files. Avoid storing unnecessary intermediate data.            |
| [ ] | **Purpose Limitation**                 | Art. 5(1)(b)         | Keep user data organized by project or case. Prevent automatic mixing of unrelated files or contexts.                                                                    |
| [ ] | **Transparency & Notice**              | Arts. 12, 13         | Include a short in-app privacy notice explaining: “All data stays on your device. No data is sent or shared externally.” Make this visible at first run and in settings. |
| [ ] | **Data Deletion (“Right to Erasure”)** | Art. 17              | Add a *Delete All My Data* button that clears all local data: files, logs, models, and settings.                                                                         |
| [ ] | **Data Correction**                    | Art. 16              | Allow users to preview and edit files before processing or saving (e.g., before anonymization or analysis).                                                              |
| [ ] | **Security: Encryption at Rest**       | Art. 32              | Encrypt all local storage (project files, logs, chats). Use Rust crypto libraries such as `ring` or OS-native APIs. Optionally, password-protect access.                 |
| [ ] | **Security: Access Control**           | Art. 32              | Integrate OS-level authentication or optional workspace password. Auto-lock app when the system locks or after inactivity.                                               |
| [ ] | **Storage Limitation**                 | Art. 5(1)(e)         | Let users configure auto-deletion of old projects or logs after a set number of days. Default to minimal retention.                                                      |
| [ ] | **Anonymization**                      | Arts. 5, 25          | Run local anonymization (regex + NER + optional advanced layer) before saving, searching, or indexing data.                                                              |
| [ ] | **Audit Logging**                      | Arts. 5(2), 30       | Maintain a local-only audit log of anonymization, deletions, and data actions. Never transmit logs.                                                                      |
| [ ] | **Security by Design & Default**       | Art. 25              | Default all settings to local-only mode. Ensure privacy-friendly defaults (no telemetry, no network requests).                                                           |
| [ ] | **Privacy Notice Accessibility**       | Arts. 12–13          | Keep a *Privacy Info* or *About Data* section in settings showing key rights and explaining local-only processing.                                                       |

# AI Act compliance
| ✅   | **Requirement**            | **Legal Article(s)** | **How to Do It Practically (Local-Only Context)**                                                                                                                |
| --- | -------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [ ] | **AI Transparency**        | Art. 52              | Clearly label AI-generated or AI-assisted content in the interface, such as an **“AI response”** badge in chats or **watermark** in exported documents.          |
| [ ] | **AI Use Explanation**     | Art. 52              | Provide a short **About / Help** section explaining: “This app uses local AI to assist with drafting, summarizing, and search. No data ever leaves your device.” |
| [ ] | **Output Provenance**      | Art. 52              | Embed basic metadata in AI-generated outputs: model name, version/date, anonymization settings. Can be stored in file properties or as a small footer.           |
| [ ] | **Human-in-the-loop**      | Art. 52              | Always let the user review and optionally edit AI outputs before saving or exporting. No automatic saving or submission of AI results.                           |
| [ ] | **Label AI-modified Data** | Art. 52              | Add a visible note or metadata tag: “Processed by AI (local model).” Include it automatically in exports or generated documents.                                 |
| [ ] | **Local Model Default**    | Art. 52              | Run entirely offline with models stored locally (e.g., via Candle or Hugging Face). Only download model files after explicit user consent.                       |
| [ ] | **Low-Risk by Design**     | Art. 52, Recitals    | Since the tool assists with text but does not automate or decide outcomes, classify as *minimal risk*. Keep user-in-control workflows to maintain this status.   |