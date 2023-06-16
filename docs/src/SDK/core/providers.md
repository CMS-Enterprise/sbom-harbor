## Providers

The concept of a **provider** is fundamental to the design of Harbor. The term provider has become 
synonymous with the [strategy pattern](https://en.wikipedia.org/wiki/Strategy_pattern) 
popularized by the Gang of Four in their seminal book [Design Patterns: Elements of Reusable 
Object-Oriented Software](https://en.wikipedia.org/wiki/Design_Patterns). The basic idea behind 
a provider is that you can define an abstract interface, or trait in Rust terminology, and 
types that implement that trait can vary in terms of the algorithm they apply internally when 
called. This is very useful in scenarios where you have a finite set of operations that you want to 
perform, but you want to be able to swap out the concrete implementation based on some context.

The `TaskProvider` trait is an example of a provider that you can use. With this trait, Harbor 
can be extended with new tasks that fit within the model, and can be swapped based on parameters 
or configuration. See the `cli` crate for more concrete examples.
