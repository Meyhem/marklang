Ever seen a cool language, perhaps a made up fantasy language, and wanted to learn it ? With [Markov chain model](https://en.wikipedia.org/wiki/Markov_chain) you can at least teach it your computer... _sort of_.

## goal
We want to to be able to take existing language (words & sentences), and be able to generate words that sound very similar to the original. We won't get exact same words but we can come very close if we have enough training data.

## markov chain model

A simple representation of markov chain is similar to state machine, however this chain does not accept sequences, but generates them. Node contains ngram (in the picture an unigram) and edge weights are probabilities of follow up ngrams. By rolling dice each step and choosing path we can walk trough the model and generate letters.

<figure>
  <img class="article-image" src="{% link assets/images/markov_machine.png %}" width="40%" alt="Simple markov chain">
</figure>

If we start at _Node A_ then we have 100% probability we will take path to _Node B_ which means that every letter __A__ will be followed by letter __B__. Not very practical in language, but lets continue.  
We are now at _Node B_ which has two paths. First path is to _Node D_ (20% probability) and second path back to _Node A_ (80% probability). Then we can continue by _Node D_ to _Node C_ and back to _Node A_. As you can see the sum of all output edges is always 100%. 
So if we attempt to generate random word from this primitive model we can get words like (let's always start from _Node A_):
* A-B-A-B-A-B-D-C-B-A-B-D-C-A ...
* A-B-D-C-A ...
* A-B-A-B-A-B-D-C-B-A-B ...

Each time we generate a letter, we roll dice and choose a path according to model. If our path splits we choose one based on rolled number and path probabilities.

If model loops (our does) we can generate infinitely long sequences. To generate natural language we will most likely stick with couple of characters per word and stop generation before we reach heat death of the universe.  
Model can of course contain _edge to self_ so an ngram can be followed by itself. These edges must count as output edges and must be summed with all output edge probabilities.  

## representation
To represent complete markov chain model by a _graph_ structure might be inefficient so we will reach for matrices. More specificaly a [Stochastic square matrix](https://en.wikipedia.org/wiki/Stochastic_matrix).  
With this matrix we can describe our previous model transition probabilities very easily (pick row then column):

|     |A  |B  |C  |D  |
|-----|---|---|---|---|
|__A__|   |1.0|   |   |
|__B__|0.8|   |   |0.2|
|__C__|0.5|0.5|   |   |
|__D__|   |   |1.0|   |

For real world data it might be worth considering [Sparse matrix](https://en.wikipedia.org/wiki/Sparse_matrix) implementations, as the matrices can grow in size and you can find yourself out of memory very fast.

## learning phase / fitting
To train a model, first we need train data, the more data we have, the higher quality model we will achieve. Also choosing __ngram size__ is important, if you pick unigrams, you might not get good results for complex languages (like human ones), but choosing large size (3 and more) you won't have enough training data. For our demonstrative purposes let's settle on using only unigrams and bigrams.  
The the learning itself works by __iterating over training data by overlapping ngrams and building probability matrix__. The probabilities in matrix represents probability that certain ngram will follow another.

Let's have:
training data: 'bananas'
ngram size: 2

1. Select bigram 'ba'
2. Select following bigram (overlapping) 'an'
3. Write to matrix that 'ba' is followed by 'an'
4. Select bigram 'an'
5. Select following bigram (overlapping) 'na'
6. Write to matrix that 'an' is followed by 'na'
7. Continue by next overlapping ngram...

After exhausting all bigrams we will end up matrix of __counts__ (not probabilities yet).

|   |ba |an |na |as |
|---|---|---|---|---|
|ba |   | 1 |   |   |
|an |   |   | 2 |   |
|na |   | 1 |   | 1 |
|as |   |   |   |   |


We read our matrix as _After bigram 'ba' a bigram 'an' occured once_.

Now that we have occurences we can calculate individual probabilities fairly simply.  
We take cell value (occurences) and divide it by sum of all row occurences.

```c
prob[i][j] = cell[i][j] / sum(cell[i])
```

And if our algo is right, we should end up with this:

|   |ba |an |na |as |
|---|---|---|---|---|
|ba |   |1.0|   |   |
|an |   |   |1.0|   |
|na |   |0.5|   |0.5|
|as |1.0|   |   |   |

Note that we taught our model that after each 'ba' we will generate 'an' and after each 'an' we will generate 'na', deterministically. However bigram 'na' will be followed be either 'an' or 'as', each with 50% probability.

It's also worth noting that the chain would be terminated by walking to bigram 'as', because it's not followed by anything, however I chained it to the first bigram 'ba'. It prevents the generation algorithm (walking trough graph) to stale.

## generation
Once we got trained model we can happily start to generate words. This process starts by selecting either random or any desired ngram we want our word to start with. Using our example let's randomly pick bigram 'an'.

1. Take first letter of selected ngram and append it to result word
2. Find row with selected ngram and generate number <0.00-1.00)
3. Iterate said row while summing cell probabilities until your sum has higher or equal value than generated number (higher than cumulative probability).
4. Check on which column you stopped and that's your new ngram
5. goto step 1 or break if you have enough letters

A pseudo code can be more descripting:

```c#
word = ""
current_gram = "an"

// generate 10 letters
for i = 0; i < 10; i++ {
    // use first letter
    word += current_gram[0]

    cumul_prob = 0
    random_uniform = rand(0.0, 1.0)
    // find row of current ngram
    foreach cell in matrix.find_row_of(current_gram) {
        // cumulate probability until its higher than random
        cumul_prob += cell.probability
        if cumul_prob >= random_uniform {
            // find on which column we stopped and get its ngram
            current_gram = matrix.find_col_ngram(cell)
            break
        }
    }
}
print(word)
```

## generating tolkien's black speech of mordor
You are going to need [Rust lang](https://www.rust-lang.org) installed if you want to run this implementation of markov chain. Then just [clone git repository](https://github.com/Meyhem/marklang).  

If we take a look inside the _examples/black_speech_mordor.rs_ you can see we first initialize bigram-based markov model generator:
```rust
MarkovLanguageGenerator::new(2);
```
Then a really long training string of actual black speech language generated by this awesome [translator](https://lingojam.com/BlackSpeechTranslator). In the example we run the training text trough bit of processing to prevent generating any punctuation or spaces or different character casing.

Then we call the training routine which counts the bigram occurences and then calculate probabilities generating stochastic matrix, like we explained.
```rust
fit_str(black_speech);
```
And finally we start generating words using our Markov model:

```rust
for _ in 0..100 {
    print!("{} ", g.gen(8).unwrap());
}
```
When we run the example by **cargo run --example black_speech_mordor** we can see series of randomly generated words that closely resembles the trained black speech.
```
fineobje amesfaid iavavopr tporavop lnodangw wshuzudc izamukgr nkuklaav gajoravr oijcraug hghtishi speaukco rzboozej dweavukc  ...
```

## sources
* [Black speech translator](https://lingojam.com/BlackSpeechTranslator)
* [Markov chain - Wikipedia](https://en.wikipedia.org/wiki/Markov_chain)
* [Github repository](https://github.com/Meyhem/marklang)