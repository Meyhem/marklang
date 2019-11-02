# marklang

Library for Markov chain language learning & generating.

1. Feed in (fit) your language (e.g. Black speech of Mordor)
2. Generate words that are random, but similar fitted language

## How it works

Fitting is process of generating markov probability matrix that represents [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) of your language. This square matrix contains probabilities of successive overlapping n-grams.

Example:

_ngram size_ = 2  
_text_ = "hello"

Created bigram probability matrix can be interpreted as **"After 'he' there is 100% probability that 'el' will follow"**. During generation only first ngram chars are used.

|   |he |el |ll |lo |oh |
|---|---|---|---|---|---|
|he |   |1.0|   |   |   |
|el |   |   |1.0|   |   |
|ll |   |   |   |1.0|   |
|lo |   |   |   |   |1.0|
|oh |1.0|   |   |   |   |

Notice that last bigram is chained to the first one.
This chain will always generate shifted sequences of 'hello' as there is only one deterministic chain **he->el->ll->lo->oh**.  
See [Black speech](https://github.com/Meyhem/marklang/blob/master/examples/black_speech_mordor.rs) example.

## Gram size
Choosing gram size is important as it affect the quality of generated text. However, in practice, it's dependent on size of learning text. Choosing ngram size > 2 might require a lot of training data & memory.

## Preprocessing
This library does absolutely no preprocessing of input text. Any valid rust string will do. If your text contains spaces, punctation, accents... it's up to you to whether you want keep it or transform it before fitting.

## Examples

```
$ cargo run --example simple
$ cargo run --example black_speech_mordor
```
