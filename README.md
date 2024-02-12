# API Assignment

## Overview

This document provides testing documentation for the project using APIs from [JSONPlaceholder](https://jsonplaceholder.typicode.com/). Two test cases are covered in this documentation: TC Get Username using the `/users` API and TC Get Photos Title using the `/photos` API.

## Test Cases

### TC Get Username

**Description:** This test case aims to retrieve user information by email using the `/users` API and verify the correctness of the returned username.

**Steps:**

1. **Get User by Email**
   - Endpoint: https://jsonplaceholder.typicode.com/users
   - Method: GET
   - Parameters: email
   - Example Request: `/users?email=test@example.com`

2. **Verify Username**
   - Extract the username from the response.
   - Verify that the extracted username matches the expected username.

### TC Get Photos Title
**Description:** This test case aims to retrieve photo information by ID using the /photos API and verify the correctness of the returned photo title.

**Steps:**

1. **Get Photo by ID**
   - Endpoint: https://jsonplaceholder.typicode.com/photos/{photoId}
   - Method: GET
   - Parameters: photoId
   - Example Request: /photos/1
     
2. **Verify Photo Title**
   - Extract the title from the response.
   - Verify that the extracted title matches the expected title.
  
## Conclusion
This documentation covers two test cases involving the use of APIs from JSONPlaceholder. Ensure that the provided sample codes are adapted to the actual structure of the responses and expectations in your specific project.
