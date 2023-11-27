# Slow-Server
A test service to mock slow server responses

### Description

**Slow-Server** lets you simulate a slow server response, when you make a request.
This can come handy while testing how your  application responds to a slow external service or API.

Right now **Slow-Server** supports only `GET` requests with configurable delay and URL parameters.

Using **Slow-Server** is really easy.

### Request Structures

##### Delay Mode
```bash
curl -XGET http://localhost:3000/delay/{delay_time}
```

### Dev Setup

##### Running the app
```bash
$ git clone https://github.com/helloimalemur/Slow-Server.git
$ cd Slow-Server/
$ cargo build --release
$ target/release/slow_server
```

### Contributions

For contributing, please raise a pull request with your code and test. You can also contribute by reporting issues and adding feature requests.


### License

This project is available for use under the GNU General Public v3.0 license.
