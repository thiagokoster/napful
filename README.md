# Napful
Napful is a command-line tool designed to help developers interact with HTTP APIs by running predefined requests.

## Current features
- **List Requests**: Easily list all requests saved in the requests folder.
- **Execute Requests**: Run a specific request by name.

## Getting Started
To get started, you can clone the repository and compile the project using Cargo:
```sh  
git clone https://github.com/thiagokoster/napful
cd napful
cargo build --release
```

## Usage

### Request File format
Requests are defined in `.txt` files within the `requests` folder.
The file format follows:
```
# Request name
HTTP_METHOD(GET or POST) URL 
Header1: value
Header2: value

Body (if applicable)
```
Here is an example:
```
# Get posts
GET https://jsonplaceholder.typicode.com/posts

# Create post
POST https://jsonplaceholder.typicode.com/posts
Content-Type: application/json
Accept: application/json
   
{
  "title": "foo",
  "body": "bar"
}
```

### Listing Requests
To list all the requests from the `requests` folder, use the following command:
```sh 
napful list
```

### Executing Requests
To execute a request, use:
```sh 
napful run "Get posts"
```
Napful will omit response headers by default. They can be enabled `--headers` flag if necessary
```sh 
napful run "Get posts" --headers
```

## License
Napful is released under the MIT License. See the [LICENSE](https://github.com/thiagokoster/napful/blob/main/LICENSE) file for more details.
