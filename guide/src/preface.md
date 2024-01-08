# Preface

In many ways, this is more than just a passion project; it's a journey through the realms of curiosity and perseverance. While at times it might seem like an exercise in futility, this project and the subsequent documentation of my adventures in its pursuit serve as a testament to the possibility of completion, not only for myself but for all the fellow procrastinators out there.

Esoteric programming languages have always held a particular fascination for me despite the absence of formal training in computer science. This lack, perhaps paradoxically, heightened my interest in implementing these languages. However, attempting to construct languages based on tutorials or delving into [The Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools) presented challenges due to my unfamiliarity with both C and C++, which are considered the premier programming languages for such endeavors.

The turning point came with the discovery of [Rust](https://www.rust-lang.org/) and the associated [_The Rust Programming Language_ Book](https://doc.rust-lang.org/book/), affectionately known as **The Book** in the community. This marked the moment I felt ready to take on a task as significant as creating an interpreter. While much has been written about the merits (and demerits) of using Rust as a backend for programming languages, most of this information was beyond my current understanding. Notably, the Rust community has successfully implemented various languages, such as [Rhai](https://github.com/jonathandturner/rhai), [Kind](https://github.com/HigherOrderCO/Kind), and [Gleam](https://github.com/gleam-lang/gleam), proving its versatility. For a hobbyist like me, Rust provided the perfect foundation to realize my project.

Now, why Brainf*\*k[^note]? Firstly, it's amusing; the name alone is sufficient to capture people's attention. More importantly, it's a simple yet expressive language. With only eight instructions, it discards many other useful features, making it an ideal testbed where someone like me can build, test, iterate, and repeat until the end of time. While a Brainf*\*k interpreter in Rust is not a novel idea and has been done before, the novelty lies in the fact that _I_ hadn't done it, and therein lies the rub.

In conclusion, I wish to express my gratitude to a few individuals.

* My very patient wife and two boisterous daughters, who not only endured my brainstorming sessions but also occasionally tolerated my absence during this project.
* [Tris Oaten](https://www.namtao.com/) of [Lost Terminal](https://lostterminal.com/) and [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) fame, among other things. His [Rust evangelism](https://www.youtube.com/watch?v=Z3xPIYHKSoI) kept my interest in Rust alive, and his [life lesson videos](https://www.youtube.com/watch?v=bJQj1uKtnus) played a pivotal role in keeping me on track.

[^note] I am censoring for the sake of propriety, not because I have any issues with cursing.
