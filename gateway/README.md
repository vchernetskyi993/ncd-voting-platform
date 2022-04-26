# Elections contract gateway

Example back-end application of some abstract organization. 
Provides common REST-like gateway to elections contract.
Intended to be used as an integration point with the rest of the organization's web2 system.

Start server:
```shell
npm start
```

## Testing

```shell
npm test
```
This starts integration tests. That use ci-testnet as a blockchain network.

Ideally we should have lighter suite with something like Ganache or Hardhat for quicker and more isolated test experience.
For now NEAR ecosystem doesn't have one.
