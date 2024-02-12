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
