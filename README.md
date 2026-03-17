# SCELE Front API

An API for fetching latest announcements from SCELE's frontpage.

## Quick, In-Class Exercise

This is an exercise to identify and fix a concurrency issue in a Rust project.
Every group will work at the same project codebase and present their work independently from the other groups.
The exercise should be done during online learning weeks and will be presented at the next class session.

Your tasks as a group:

0. [ ] Fork this project codebase into your group's namespace/organization on GitHub.
1. [ ] Identify the concurrency issue(s) that is currently present in the project.
   Specifically, look at the [`lib.rs`](./src/main.rs) and try to answer these questions:
   - What is the critical section of the code?
   - Is there a race condition in the code?
   - Is there a lost update in the code?
   - How does the race condition correlate with the lost update, if any?
   - What is the synchronization approach to fix the issue?
   - Is there a possibility of deadlock when applying the proposed synchronization?
2. [ ] Fix the project by applying the synchronization approach you proposed.
3. [ ] Run the project and ensure the request count is correct.
4. [ ] Implement an improvement to the project that involves the shared `ServerState`. You can choose to:
   - Implement a cache that will be used to store the initial response from SCELE and will be used to serve subsequent requests.
   - Any other improvement that you come up with.
5. [ ] Answer these questions:
   - How does the new improvement affect the shared data in the `ServerState`?
   - Is there a new concurrency issue?
6. [ ] Save your work as new commits in a new branch, and push them to your forked repository on GitHub.
7. [ ] Create a short, 3-minutes presentation that briefly explain your work.

## License

This project is licensed under either the following licenses:

- [Apache License 2.0](./LICENSE-APACHE)
- [MIT License](./LICENSE-MIT)