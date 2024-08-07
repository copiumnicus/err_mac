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
