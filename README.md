# ePay

ePay is a payment solution on Internet Computer that accommodate the needs of both physical and virtual commerce. In the ever-evolving digital landscape, the need for robust and secure payment solutions is paramount.  ePay is an innovative, blockchain-based payment platform built on the Internet Computer. Designed to foster seamless and private transactions, ePay aims to bridge the gap between users and merchants through a decentralized, scalable, and highly efficient ecosystem.

## Architecture

While pertaining the payment flows that centralized services as Paypal and Alipay provide, the main problem solved in this project was the scalability of the payment solution, which was designed using a bus-based architecture, i.e. a single management canister with a slave user canister and multiple slave merchant canisters. This has the advantage of greatly reducing the indexing problems caused by an excessive number of orders, improving the efficiency of queries, additions and udpates, and similarly, even if a merchant canister stores a huge number of orders and causes the above indexing problem, it can be solved by adding new merchant canisters.

## Payment Flow



## Features & Mechanisms

+ User to Merchant Payments 
+ Proxy Payments
+ On-hold Mechanism 

## TODOs

+ Bill Splitting
+ Extensive tests
+ Payment SDK 

## Development guides 

