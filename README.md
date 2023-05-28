# RRaMA - Rustling with LLaMAs

**Local Inference, Model Management and No Nonsense!**

Are you tired of constantly checking Twitter and r/LocalLLaMA for the latest model releases, spamming HuggingFace search bar with '13b', '30b' and '65b' queries, and wracking your brain over how to even start running things? Annoyed at figuring out the compatible quantization methods, looking for the right tokenizer, and the best way to prompt new models?

I feel you. While RRaMA may not be the one-size-fits-all solution we all secretly wish for, it's certainly a start. And who knows? Perhaps, one day, it will grow to solve all these challenges and more.

## The Dream

Imagine if you could summon the powers of a single binary to get a fully equipped suite for **model management**, **quantization**, and **inference**, topped off with a slick Web UI and a nifty REST API. No fuss with conda, no wrangling with PyTorch, and no docker containers playing hide-and-seek with your sanity.

I've heard whispers in the coding grapevine that Rust can conjure this dream into reality. So, armed with unwavering optimism (and a healthy ~~dose of internet tutorials~~ GPT4 subscription), here I am, about to leap into the fascinating world of Rust. I don't know Rust... yet. But, when has a little thing like that ever stopped anyone?

## A Journey of a Thousand Lines Begins with a Single `fn main()`

I'll be documenting my journey as I learn Rust, manage LLaMA models, and work to turn RRaMA into the tool we all need. Will there be bugs? Absolutely! Will I sometimes question the wisdom of this endeavor? Most likely! Will it be fun? You bet!

So, come along on this rollercoaster ride of language learning and llama wrangling. Your support, insights, and pull requests are more than welcome! After all, it takes a village to train a LLaMA.

Let's turn this Rust bucket into a shiny new RRaMA!

## Sample Usage

```
    rrama --config /path/to/your/config
            Start rrama with a custom configuration directory.

    rrama server --port 9000
            Start the rrama web server on port 9000.

    rrama model list
            List all local models.

    rrama model download model_name
            Download a model from HuggingFace.

    rrama model delete model_name
            Delete a local model.

    rrama ask "What's the capital of UK?"
            Ask the model a single question.

    rrama chat
            Start a back-and-forth conversation with the model.

    rrama ask --input minutes.txt "Who proposed downshifting?"
            Ask the model a question about the content of a file.

    rrama chat --input minutes.txt
            Start a back-and-forth conversation with the model about the content of a file.

    rrama ask --model vicuna-13b "What's the capital of UK?"
            Ask the model a question using a specific model.

    rrama model info model_name
            Show information about a local model.

    rrama prompt add introduction "Hello, I am a language model."
            Add a named prompt.

    rrama preset create knowitall --model koala-30b --temperature 0.7 --max_tokens 100
            Create a new preset.

    rrama preset edit knowitall --temperature 0.5
            Edit an existing preset.

    rrama preset delete knowitall
            Delete an existing preset.

    rrama ask --preset knowitall "What is the meaning of life?"
            Ask the model a question using a preset.
```

## REST API (backwards compatible with ClosedAI)

Models

1. **List Existing Models**:

- `GET /api/v1/models`
  - List existing models
  - _ClosedAI compatible_
- `GET /api/v1/models/{modelId}`
  - Retrieve a model instance
  - _ClosedAI compatible_
- `DELETE /api/v1/models/{modelId}`
  - Remove an existing model
- `PUT /api/v1/models/{modelId}`
  - Upload a new model

2. **Search and download models from HuggingFace**:

- `GET /api/v1/hf/models`
  - Search models on HuggingFace
- `POST /api/v1/hf/models/{modelId}`
  - Download Models from HuggingFace
  - Request Body: `{ "modelId": "string" }`

3. **Completions**:

- `POST /api/v1/completions`
  - Create a completion
  - _ClosedAI compatible_
- `POST /api/v1/chat/completions`
  - Create a chat completion
  - _ClosedAI compatible_
- `POST /api/v1/edits`
  - Create an edit
  - _ClosedAI compatible_
- `POST /api/v1/embeddings`
  - Create an embedding
  - _ClosedAI compatible_

4. **Prompts**:

- `GET /api/v1/prompts`
  - List existing prompts
- `PUT /api/v1/prompts/{promptId}`
  - Modify a Prompt
  - Request Body: `{ "content": "string" }`
- `DELETE /api/v1/prompts/{promptId}`
  - Remove a Prompt

5. **Presets**

- `GET /api/v1/presets`
  - List Existing Presets
- `POST /api/v1/presets`
  - Create a New Preset
- `PUT /api/v1/presets/{presetId}`
  - Modify an Existing Preset
- `DELETE /api/v1/presets/{presetId}`
  - Remove a Preset

6. **Chats**

- `GET /api/v1/chats`
  - List Existing Chats
- `POST /api/v1/chats`
  - Start a New Chat
- `GET /api/v1/chats/{chatId}`
  - Load an Existing Chat (return previous history)
- `PUT /api/v1/chats/{chatId}/history`
  - Modify Chat History
- `DELETE /chats/{chatId}/history`
  - Delete Chat History
