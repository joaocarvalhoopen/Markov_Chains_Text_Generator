# Markov Chains Text Generator   
A simple text generator model in a world of GPT3 :-)

## Description
This program extract the n-grams probabilistic Markov Chaines model from a source text and generates random text from the model, with same statistical proprieties. <br>
<br> 
This code is a modified re-implementation in Rust of the code in Javascript from the Coding Train episode (See the project page for all the references):

* The Coding Train <br>
  Session 6: **N-Grams and Markov Chains - Programming with Text** <br>
  [https://www.youtube.com/playlist?list=PLRqwX-V7Uu6ah9Oqs_BFQIbGIn1XynsVT](https://www.youtube.com/playlist?list=PLRqwX-V7Uu6ah9Oqs_BFQIbGIn1XynsVT)


## Notes 
**This is a experimental project just for fun not a complete program, so experiment with it.** To add a new text just add the test to the string of last function get_dostoyevsky(), change the starting word and the n-grams length inside generate_dostoyevsky(). The results don't have the quality of GPT3 but the training of the model is instant and doesn't cost millions of dollars :-)


## Todo 
* Do the Command Line parser by hand or using the Clap lib.

* Modify the code to allow it to generate at the same time separate models from more then one book, and then, sample from the different models por each n-gram at a specific probability, example 20 % from one book, 30 % from other, and 50 % from other, while generating only one text string with n characters.


## How to
```
To only build do:
> cargo build

To run do:
> cargo run --release

To run the tests do:
> cargo test --release
```


## References

* See last chapter on Markov Chains of the book.<br>
  **Introduction to Probability, Statistics, and Random Processes** <br>
  by Hossein Pishro-Nik <br>
  [http://www.probabilitycourse.com/preface.php](http://www.probabilitycourse.com/preface.php)

* The Coding Train <br>
  Session 6: **N-Grams and Markov Chains - Programming with Text** <br>
  [https://www.youtube.com/playlist?list=PLRqwX-V7Uu6ah9Oqs_BFQIbGIn1XynsVT](https://www.youtube.com/playlist?list=PLRqwX-V7Uu6ah9Oqs_BFQIbGIn1XynsVT)

* **Markov Chains Explained Visually** <br>
  [https://setosa.io/ev/markov-chains/](https://setosa.io/ev/markov-chains/)

* **Markov Chains - Part 1** <br>
  Coding Challenge #42.1 <br>
  [https://thecodingtrain.com/CodingChallenges/042.1-markov-chains.html](https://thecodingtrain.com/CodingChallenges/042.1-markov-chains.html)

* **Markov Chains - Part 2** <br>
  Coding Challenge #42.2 <br>
  [https://thecodingtrain.com/CodingChallenges/042.2-markov-chains](https://thecodingtrain.com/CodingChallenges/042.2-markov-chains)

* **GitHub Code** <br>
  Github CodingTrain / website/CodingChallenges/CC_042.1_markov-chain/ <br>
  [https://github.com/CodingTrain/website/tree/main/CodingChallenges/CC_042.1_markov-chain](https://github.com/CodingTrain/website/tree/main/CodingChallenges/CC_042.1_markov-chain)


## License: 
The License is MIT Open Source.


## Have fun!
Best regards, <br>
Joao Nuno Carvalho