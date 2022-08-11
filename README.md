### Lambda requirements

All lambdas should implement the same algorithm in different programming languages:

1. Parse `iteration` parameter
2. Respond with `400 BAD REQUEST` if the params wasn't provieded
3. Save all params to `dynamodb` alongside with the programming language and AWS Lambda event

| langCase | iteration | ... all other parameters as separate columns ... | raw_event |
|----------|-----------|--------------------------------------------------|-----------|
| ruby-2.7-x86 | 1 | ... | ... |

4. Try to read item with iteration = `<current iteration from params> - 1`  and langCase = `<your language>`
5. Respond with previous item (if it is missing, respond with the newly inserted item)


### Let the battle begin!
![λ-battle](./ledovoe-poboishhe.jpeg "λ-довое побоище") 
