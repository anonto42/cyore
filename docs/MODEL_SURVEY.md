# Model Survey

Stage 1's last item: the main neural-network families, what each one is good at, and which one fits each Cyore model in Stage 2. These are study notes, not a spec. Every family below is built from the same parts you already wrote in `xor` and `mnist`: weighted sums, a non-linearity, a loss, and backpropagation.

**What changes from one family to the next is how the weights are wired to the input.** The wiring is a built-in assumption about the data (an *inductive bias*). A good assumption means fewer weights and less data are needed to learn the same thing.

---

## 1. Dense networks (MLPs)
*What you built in `xor` and `mnist`.*

Every input connects to every neuron: `a = σ(W·x + b)`.

- **Assumption**: None. Pixel 0 and pixel 783 are treated as unrelated numbers, and the network has to discover that neighbouring pixels belong together.
- **Cost**: `mnist`'s 784 → 128 → 10 network already has **101,770** parameters, almost all of them in the first layer.
- **Fixed-size input**: A sentence has to be squashed into one fixed-length vector first, for example a *bag of words* (a count for each vocabulary word). That throws away word order: "list files in docs" and "docs in files list" look identical.
- **Good for**: Small fixed-size inputs, the last layers of other networks, and quick baselines.

---

## 2. Convolutional networks (CNNs)
*Built for images. Also useful for short text.*

A small filter (for example 3×3) slides across the input and computes the same weighted sum at every position. A layer is a stack of such filters, usually followed by pooling, which shrinks the map by keeping the strongest responses.

- **Assumption**: Useful patterns are **local** (edges, strokes, short word groups) and can appear **anywhere**. One filter is reused at every position (*weight sharing*).
- **Cost**: 8 filters of 3×3 on a 28×28 image use only **80** parameters, compared with 100,480 for `mnist`'s first dense layer.
- **On MNIST**: A small CNN reaches about 99% test accuracy, compared with `mnist`'s 97.96%.
- **For text**: A 1-D convolution over word vectors acts as an *n-gram detector* ("disk space", "list files"). It is fast and works well for sentence classification.
- **Weak at**: Relationships between positions far apart, unless many layers are stacked.

---

## 3. Recurrent networks (RNNs, LSTMs, GRUs)
*Read a sequence one step at a time.*

A hidden state carries a memory forward: `h_t = tanh(W·x_t + U·h_{t−1} + b)`. The same `W` and `U` are reused at every time step.

- **Assumption**: Order matters, and the past summarises what came before.
- **Training**: *Backpropagation through time* unrolls the loop and runs the chain rule backwards across the steps. The gradient is multiplied by `U` once per step, so over long sequences it tends to shrink to nothing or blow up (*vanishing* and *exploding gradients*).
- **LSTM / GRU**: Add *gates*, learned sigmoids between 0 and 1 that decide what to keep, forget and output. Gates let useful gradients survive across many steps.
- **Bidirectional**: Run one RNN left-to-right and another right-to-left, then join their states. Each word's state then knows its whole sentence, which is ideal for tagging every word.
- **Weak at**: Speed. Step `t` has to wait for step `t − 1`, so training can't run the steps in parallel.

---

## 4. Transformers
*The basis of today's large language models.*

Each word is turned into a vector (an *embedding*), and **self-attention** lets every word look at every other word in one step:

```
Attention(Q, K, V) = softmax(Q·Kᵀ / √d) · V
```

Each word asks a *query* (`Q`), every word offers a *key* (`K`), and the softmax of their dot products decides how much of each word's *value* (`V`) to mix in. It is the same softmax you wrote for `mnist`'s output layer. Since attention ignores order, a *positional encoding* is added to each embedding.

- **Assumption**: Very little. Any word may relate to any other, and the network learns which relations matter.
- **Strengths**: Every position is processed in parallel, it captures long-range relations directly, and it scales extremely well with data and parameters.
- **Cost**: Attention compares every pair of words, so cost grows with the square of the sentence length. That doesn't matter for 5–15-word requests. It also needs more data than CNNs or RNNs to learn what their built-in assumptions give them for free.
- **Encoder–decoder (seq2seq)**: An encoder reads the request, and a decoder writes the output one token at a time while attending to the encoder. This is how a model can *write* a command instead of choosing one from a list.

---

## Side by side

| Family | Wiring | Shares weights across | Strength | Weakness | Role in Cyore |
|---|---|---|---|---|---|
| Dense | Everything to everything | Nothing | Simple, a good baseline | Ignores structure; fixed-size input | Stage 1; the final layers of Stage 2 models |
| CNN | Local filters | Positions | Few parameters, fast | Far-apart relations need depth | Option for the intent classifier |
| RNN (GRU / LSTM) | Step-by-step memory | Time steps | Order-aware; a natural fit for tagging | Slow, sequential training | Option for the slot tagger |
| Transformer | Everyone attends to everyone | Positions (per layer) | Long-range relations, parallel, scales | Needs more data; cost grows quadratically with length | Stage 2's advanced seq2seq model |

---

## What this means for Stage 2

Stage 2 has to read a short English request and produce:
1. An **intent**: one label out of 51, each mapped to a command template (for example, "how much disk space" → `df -h`). This is *sentence classification*.
2. **Slots**: a tag on every word marking values like folder and file names (BIO tags: `B-` begins a value, `I-` continues one, `O` is any other word). This is *sequence tagging*.

The [command dataset](../dataset/README.md) (`make commands-data`) provides exactly these labels. A sensible order:

1. **Baseline: dense network on a bag of words**, for the intent only. It's the `mnist` network with a different input, so it shows how far plain counting gets. It can't tag slots, because a bag of words has lost the positions.
2. **Joint model: embeddings → bidirectional GRU (or a 1-D CNN) → two heads.** One dense + softmax head per word gives the slot tags, and one head on the pooled sentence gives the intent. This is the plan's "intent classifier + slot model".
3. **Advanced: a small transformer seq2seq** that writes the command text itself. This is the plan's 5–50M-parameter model.

### The footprint argument
The Stage 4 budget is a 5–30 MB binary that uses under 100 MB of RAM. Rough sizes:

| Part | Parameters | float32 | int8 |
|---|---|---|---|
| Word embeddings, 2,000 words × 64 dims | 128,000 | 512 KB | 128 KB |
| Bidirectional GRU, 64 → 64 | ≈ 50,000 | ≈ 200 KB | ≈ 50 KB |
| Tiny seq2seq transformer | 5–50 M | 20–200 MB | 5–50 MB |

The joint classifier fits in well under 1 MB. The transformer only fits after int8 quantisation, and only at the small end of its range. That is why the plan builds the classifier first and treats seq2seq as the advanced step.

---

## Further study
- 3Blue1Brown, *Neural Networks*, chapters 5–7: attention and transformers, visually.
- Andrej Karpathy, *Neural Networks: Zero to Hero*: the `makemore` lectures build an MLP, then an RNN-style model, then a transformer, one step at a time.
- Vaswani et al., *Attention Is All You Need* (2017): the original transformer paper.
- Yoon Kim, *Convolutional Neural Networks for Sentence Classification* (2014): 1-D CNNs for text.
- The Burn Book: modules for `Embedding`, `Gru`, `Conv1d` and `TransformerEncoder`, which are the building blocks Stage 2 will use.
