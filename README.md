# distributed_cache
### My first thought of building an async distributed cache for that i have planned to write server in rust and client in go and client is the one that maintains the decoradam of corun(concencus algo) and consistent hashing.


#### To implement the above first i needed to build a server we have many stable server outside made in using Tokio rust library but i want to explore how actually async work so made an entire TCP Async server from scratch Using system calls BTW in rust system calls are marked as unsafe as their behaviour is predetermined.

#### In the above code I have build both async and sync server using system calls and async server has the ability to handle 1000 conccruent request as of now.

#### Beware:- the above code for TCP async is highly unsafe as we have made raw system calls and Rust compiler doesnt gurantee the behaviour of memory here so memeory leaks might be common.



my own implementation of distributed cache using rust and go
