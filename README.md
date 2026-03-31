[# SCELE Front API

An API for fetching latest announcements from SCELE's frontpage.

## Quick, In-Class Exercise

This is an exercise to identify and fix a concurrency issue in a Rust project.
Every group will work at the same project codebase and present their work independently from the other groups.
The exercise should be done during online learning weeks and will be presented at the next offline class session (31 March 2026).

Your tasks as a group:

0. [X] Fork this project codebase into your group's namespace/organization on GitHub.
1. [X] Identify the concurrency issue(s) that is currently present in the project.
   Specifically, look at the [`lib.rs`](./src/main.rs) and try to answer these questions:
   - What is the critical section of the code?
   
     the request count increment in the unsafe block (line 61-67)
     it does a read of val = *request_count_ptr
     then later write  *request_count_ptr = val + 1
     to a shared state

   - Is there a race condition in the code?
     data.request_count inside the get_all_documents:
   


   - Is there a lost update in the code?

     request_count_ptr points at shared memory that multiple request can touch at the same time
     if there is 2 request, they both read the same val before the writes and write val+1
     so it can increment one for 2 request


   - How does the race condition correlate with the lost update, if any?
     a possibility of two threads can read the same old values before either writes
     if it happens both compute the val+1 and later overwrites the later one , there will be one increment 
     that is lost


   - What is the synchronization approach to fix the issue?
     Maybe a Mutual Excusion



   - Is there a possibility of deadlock when applying the proposed synchronization?
     This is a matter of a read write race, a Mutex implementation of simple mutex would not make a deadlock



2. [X] Fix the project by applying the synchronization approach you proposed.
   Flow:
   Generate a random delay

   Wait asynchronously for that delay

   Run the blocking scraping/parsing work in a blocking thread pool

   Get the parsed announcements result back

   Lock the mutex and increment request_count

   Print the updated request count

   Return the announcements as JSON response

3. [ ] Run the project and ensure the request count is correct.
4. [ ] Implement an improvement to the project that involves the shared `ServerState`. You can choose to:
   - Implement a cache that will be used to store the initial response from SCELE and will be used to serve subsequent requests.
   - Any other improvement that you come up with.
5. [ ] Answer these questions:
   - How does the new improvement affect the shared data in the `ServerState`?
     The new improvement adds a cache into ServerState. Before the improvement, the shared state only stored 
   request_count, which was used to count how many requests had been handled. After the improvement, ServerState now 
   stores two shared data items: request_count and cache.

    The cache stores the latest list of announcements and also the time when the data was fetched. This affects the 
shared state because now multiple requests may access not only the counter, but also the cached announcements at the 
same time. Because of that, both shared variables must be synchronized properly using Mutex.

    This improvement is useful because the server does not need to fetch and parse the SCELE frontpage on every request. 
If the cached data is still valid within the TTL, the server can return the cached result directly, which improves 
efficiency and reduces repeated work.
   - Is there a new concurrency issue?
     If several requests arrive at almost the same time when the cache is empty or already expired, they can all check 
   the cache before any one of them updates it. As a result, multiple requests may detect the same cache miss and perform 
   the same fetch and parsing work simultaneously. This does not create a data race, because access to the cache is still 
   protected by a Mutex, but it can create redundant work and reduce performance.

    In other words, the shared data is still safe, but the caching system is not fully optimal yet. This situation is 
similar to a cache stampede, where many requests try to refresh the same cache at once. However, the code already avoids 
a worse problem by not holding the mutex during the slow network request and parsing process. The lock is only used 
briefly when checking or updating the cache, so other threads are not blocked for too long.

Also, there is still no deadlock in this implementation because the mutexes are only locked for a short time and there 
is no circular waiting between threads.
6. [ ] Save your work as new commits in a new branch, and push them to your forked repository on GitHub.
7. [ ] Create a short, 3-minutes presentation that briefly explain your work.
]
## License

This project is licensed under either the following licenses:

- [Apache License 2.0](./LICENSE-APACHE)
- [MIT License](./LICENSE-MIT)