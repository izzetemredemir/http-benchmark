# HTTP Get Request Performance Comparison

## About the Project
This project aims to compare the performance of HTTP GET requests using various programming languages and libraries. Our goal is to understand how different technologies fare in handling network requests and to provide measurable data for comparison.

## Methodology
Performance tests are conducted by sending 100 HTTP GET requests to a specific web address (`https://example.com/`). The response time for each request is recorded, and these data are used to calculate the average response time. This time is used to compare across different programming languages and libraries.

### Test Steps:
1. Write a simple script in the chosen programming language/library to make an HTTP GET request.
2. Send 100 requests to `https://example.com/`.
3. Record the response time for each request.
4. Calculate the average of the recorded times once all requests are completed.

## Performance Comparison Table
| Language   | Library/Packet        | Average Response Time |
|------------|-----------------------|----------------------:|
| TypeScript | untici                |                -   ms |
| Go         | net/http              |                  - ms |
| Rust       | reqwest               |                  - ms |
| PHP        | file_get_contents     |                  - ms |

