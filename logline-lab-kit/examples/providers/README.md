# Provider examples — nine-slot LogLine Acts

Each `*.example.json` here is a real **nine-slot LogLine Act** (`register_provider`), with
**string slots** matching the canon mold (`logline.receipt.v0`). Provider truth in a Lab is
an Act — **not** a YAML/JSON-Schema config file and not a `.logline` grammar line. The live
provider registry is *projected* from these Acts; there is no registry-of-record file.

**Secrets never enter an Act.** The `this` slot records the env-var **name** (`auth_env=…`)
and the non-secret endpoint — never the key value.

`this` is a `key=value; key=value` descriptor (string slot):
`provider_id`, `kind`, `base_url`, `model`, `auth_env`, `dev_only`.

| File | What |
|---|---|
| `openai-compatible.example.json` | a generic OpenAI-compatible provider (OpenAI/vLLM/LiteLLM/LM Studio) |
| `ollama-openai-compatible.example.json` | a LOCAL Ollama via its OpenAI-compatible port (no key; `auth_env=` empty) |
| `mistral-minilab.example.json` | **PRIVATE / operator-specific** — one operator's endpoint, included only as an example of a private profile. NOT part of the generic kit. The generic claim is "provider registry + OpenAI-compatible adapter," not "ships with Mistral." This endpoint uses an `EMPTY` bearer; express it via the env var, never bake `Bearer EMPTY` into code. |

Register one (which emits the same Act into the Lab):

```sh
labkit settings providers add openai \
  --kind openai-compatible \
  --base-url https://api.openai.com/v1 \
  --model gpt-4o-mini \
  --api-key-env OPENAI_API_KEY \
  --lab examples/manifests/lab.json --profile profiles/local-only/profile.json --store ./lab
```

Note: these are canon-shape Acts (string slots). labd may also use a *richer* internal
candidate/convention with structured slots for ergonomics — but that is an internal
projection and must project to this string-slot receipt shape before any canon/conformance
claim (open divergence #3). Do not conflate the two.
