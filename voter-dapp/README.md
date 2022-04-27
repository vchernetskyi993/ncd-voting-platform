# Voting App

Example voting front-end application. Uses NEAR [contract](../contract/) as a back-end.
Like [a gateway dapp](../gateway/) this app is tied to 1 organization. 
So, it provides an ability for users to vote on some organizational decisions.

## Deployment

To start locally:

1. Update env file with network&account details

2. Start server:
```shell
npm start
```

## Possible NEAR platform improvements

1. No out-of-the box event notifications through WebSockets.
With such a feature added this app could dynamically update UI with voting results.
