# Napful
Napful is a command-line tool designed to help developers interact with HTTP APIs by running predefined requests.

## Current features
- **List Requests**: Easily list all requests saved in the requests folder.
- **Execute Requests**: Run a specific request by name.
- **Environment variable support**: Define environment variables in a `.env` file or as actual environment variables to customize your API requests on the fly.

## Getting Started
To get stared, you can clone the repository and complie the project using Cargo:
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
Napful will ommit response headers by default. They can be enabled `--headers` flag if necessary
```sh 
napful run "Get posts" --headers
```

### Environment Variables
Define environment variables in a `.env` file located in your `requests` folder. The `.env` file should follow the format:
```env
BASE_URL=https://api.example.com
API_KEY=your_api_key
```

You can also specify environment variables in your shell session, which will take precedence over the `.env` file.

Variables can be used in the request file like so:
```
# Get posts
GET {{BASE_URL}}/posts
```

## License
Napful is released under the MIT License. See the LICENSE file for more details.
