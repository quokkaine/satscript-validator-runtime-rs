# Satscript Validator Runtime

the following is an experiment for inscribing smart contracts and transactions on BTC layer 1.

## Transactions

a simplistic way to inscribe transactions with json.

deploy a contract
```json
{
    "code": "<contract_code>",
}
```

transact with a contract
```json
{
    "to": "<contract_ordinal_address>",
    "selector": "<function_name_and_args>",
    "args": "<function_args_in_order>"
}
```

_some general thoughts here_

It is obvious that there are more efficient ways of inscribing these transactions, for example, we could develop something
akin to the EVM that compresses contracts/transactions into bytecode that can be ran more efficiently. This approach
would most likely homogenize our transaction formats: meaning that contract deployment/update transactions would share
the same format.

_potentially_

deploy
```json
{
    "to": "0x0", // not needed for deployment
    "data": "<contract_bytecode>"
}

transact
```json
{
    "to": "<contract_address>",
    "data": "<transaction_bytecode>"
}

```