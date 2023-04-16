1) Middleware
2) Error handling:
    a) Define enum of your errors
    b) Implement Response error for your errors
    c) Implement From<sqlx::Error> for your error so that sqlx errors can be automatically converted to your errors
    d) Use question mark operator where you can
    e) Try to use less unwraps
    f) Try to clone less
3) Don't use select * in code.
