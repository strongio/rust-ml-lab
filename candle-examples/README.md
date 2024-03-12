# ML in Rust with Candle

`candle` is a minimalist ML framework for Rust developed by HuggingFace.

Candle has a number of implementations of popular ML models, however it is still in its infancy and lacks many features we're used to in PyTorch. For example there isn't much documentation on training models using candle.

At first glance it seems like `candle` is a good choice for building inference-only systems from pre-trained models. With more work, you could probably train a model from scratch.

Here we have two examples on how to use `candle`:

### 1. `mnist`
 
- Shows how to use `candle` to build a simple two-layer model

```bash
cargo run --example mnist
```


### 2. `gpt-rs`

- Sourced from this [blog post](https://www.perceptivebits.com/building-gpt-from-scratch-in-rust-and-candle/) and [repo](https://github.com/jeroenvlek/gpt-from-scratch-rs) by Jeroen Vlek
  - Follows Andrej Karpathy's "Let's Build GPT" tutorial
- Builds GPT and trains it on a small dataset  

```bash
wget https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt -O data/tinyshakespeare.txt
```

```bash
cargo run --example gpt-rs -- --input-path 'data/tinyshakespeare.txt'
```