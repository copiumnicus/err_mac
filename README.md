nice, simple, rustic beef wellington.

# Add
```toml
# using rev will prevent cargo from checking if there are updates to the repo
err_mac = { git = "https://github.com/copiumnicus/err_mac.git", rev = "08f6335" }
```

# Usage
```rust
use err_mac::create_err_with_impls;

#[derive(Debug, PartialEq)]
struct MarketErr;
#[derive(Debug, PartialEq)]
struct ConvertErr;
#[derive(Debug, PartialEq)]
struct EthRpcErr;

create_err_with_impls!(
    #[derive(Debug, PartialEq)]
    SplitErr,
    InputZero,
    MissingAsset,
    MissingPrice,
    Market(MarketErr),
    Convert(ConvertErr),
    NotFeasibleSplit,
    EthRpc(EthRpcErr),
    InterOutTaken,
    WorseOut,
    SumNotOne,
    ;
    Hmm {a: f64},
);
```

# Produces
```rust
#[derive(Debug, PartialEq)]
enum SplitErr {
    InputZero,
    MissingAsset,
    MissingPrice,
    Market(MarketErr),
    Convert(ConvertErr),
    NotFeasibleSplit,
    EthRpc(EthRpcErr),
    InterOutTaken,
    SumGradsNotZero,
    WorseOut,
    SumNotOne,
    Hmm { a: f64 },
}
impl From<MarketErr> for SplitErr {
    fn from(v: MarketErr) -> Self {
        Self::Market(v)
    }
}
impl From<ConvertErr> for SplitErr {
    fn from(v: ConvertErr) -> Self {
        Self::Convert(v)
    }
}
impl From<EthRpcErr> for SplitErr {
    fn from(v: EthRpcErr) -> Self {
        Self::EthRpc(v)
    }
}
impl std::fmt::Display for SplitErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt($crate::format_args!("{:?}", self))
    }
}
```
