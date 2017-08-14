You want to help contribute? Awesome! Thanks for taking the time to look at the
guidelines for this repo. Here's what you need to know!

## Code

Want to help make `ferris_says` faster, want to help expand the `fsays` binary
to have more features? Then just follow these steps:

1) Fork and clone the repo
2) Create a new feature branch
3) Work on the feature or bug fix and push it to GitHub
4) Open up a PR on this repo with your feature branch
5) After code review and changes are done it'll be merged!

## Documentation

Documentation is an important part of any project and `ferris_says` is no
exception. If you see something has no documentation, could have it's
documentation made more clear, or there are too few examples please either open
up a PR to fix the issue or at the very least file an issue in the issue
tracker so that it can be updated.

### Examples

Do you want to help show off some ways for how the library works? Feel free to
work on an example and open up a PR! All examples should go under the `examples`
folder so that people can execute them using `cargo run --example example_name`.

## Issue Tracker

Have you encountered a problem with the API? Please take a look at the
guidelines below for filing bugs:

### Filing Issues

If you're here then you've probably encountered an issue with `ferris_says`,
uh oh! While we would love to make sure your code works we need a little help
from you.

#### Before opening up an issue

- Have you checked to see if there is already an issue open?
  - It might have also recently been fixed and a new release
    wasn't pushed so check the closed issues too

If you've answered yes please do not open open up another duplicate issue. It
will be flagged and closed.

- If it is open is there already a solution or plan available?

If not then just comment that you've faced the same issue so that it has more
visibility. Please do not ask how long or when this will be fixed. While we like
working on this and open source software, we aren't paid for it and do have
lives of our own.

#### What to include in an issue:

- What did you expect to happen?
- What happened?
- What steps did you take with `ferris_says` to try to make that happen?
- If the program crashed please provide a full stack trace by setting this:

  ```bash
  export RUST_BACKTRACE=full
  ```

  and re-running your code so that it crashes again. Paste the output of that
  in your issue.
- Any other useful information you think could help diagnose the issue.

#### Thanks

Most importantly though, you're using this library which is in and of itself
a rewarding thing for us. The fact you want to help make it better by reporting
an issue is great and helpful. Just make sure you help us by following these
guidelines! Thanks!
