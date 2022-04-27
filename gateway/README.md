# Elections contract gateway

Example back-end application of some abstract organization. 
Provides common REST-like gateway to elections contract.
Intended to be used as an integration point with the rest of the organization's web2 system.

## Deployment

To start locally:

1. Update env file with network&account details

2. Start server:
```shell
npm start
```

## Testing

```shell
npm test
```
This starts integration tests. That use ci-testnet as a blockchain network.

## Possible NEAR platform improvements

1. Test utilities. 
Ideally we should have lighter test nodes (something like Ganache or Hardhat) for quicker and more isolated test experience. 
Also such a suite can have customizable setup. 
For example, setting desired block_timestamp and other properties for a specific test.
