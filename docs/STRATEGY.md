# Pimping Strategies

A strategy is defined as an implementation of the following trait:

```rs
pub trait Strategy: ParallelIterator<Item = (SecretKey, PublicKey)> + Debug {}
```

What matters is how the `ParallelIterator` is defined.

## Random Strategy

## Linear Strategy
