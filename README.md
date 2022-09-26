# Fuber

## How to start the server
- To run the server you need to have the rust toolchain installed.

### Installing Rust and Cargo
- Goto [rustup.rs](https://rustup.rs/) and over there you will find the instructions to install cargo and rust. 

    The instructions are tailored to the OS that you are currently using and follow through all the steps to the point where you have everything related to cargo and rust installed.

    For me it shows up something like this 
    ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    The cli tool that runs the installation would have the option to choose **recommended** stuff, I would personally opt for that.
    
- After the installation of Rust and Cargo is finished just to check everything went well run the following commands 
    ```bash
    cargo --version && rustc --version
    ```
    to get the desired output
    ```bash
    ❯ cargo --version && rustc --version
    cargo 1.63.0 (fd9c4297c 2022-07-01)
    rustc 1.63.0 (4b91a6ea7 2022-08-08)
    ```
    The version names can be different depending upon the release.

- Now let's unzip file



### Running the Fuber server

- if on linux `unzip fuber.zip` should work. On Mac and Windows I think the GUI is the best way to unzip it.

- so the path that you extracted the project file let's say it's name is `PATH-TO-FUBER` let's go inside the path `cd <PATH-TO_FUBER>`

- Inside the path there needs to be a `.env` file to connect to the Database, since we are using MongoDB we need to create a project on a MongoDB account, for now I am using my MongoDB URI locally. If you need to run the server I would suggest setting up locally. I don't think the MongoDB ones are easy to follow [the one I used](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5) to build the REST api and as well as understand the usage of rust with mongo would be better. Whatever path you choose the the `.env` file should contain the following variable.
    ```bash
    MONGOURI=mongodb+srv://shubham:[REDACTED]@rust-api.71yxxhx.mongodb.net/?retryWrites=true&w=majority
    ```

- The directory structure should be ready to build so lets run `cargo build` in the fuber directory. It should end with no errors, warnings can be ignored. The text you are looking for is : 
     ```bash
        Finished dev [unoptimized + debuginfo] target(s) in 1m 01s
     ```
    this might take time because all the packages needed are getting and it's hard to be impatient at these times specially the anxiety of something going wrong might be this will take some time.

    If you don't include the `.env` variable then you might not get any errors while building but in the next step when you run the server that might be an issue.

- The directory structure should now look like the following
    ```bash
        |___ src
              |___ api
                    |___ cab_api.rs
                    |___ person_api.rs
                    |___ mod.rs
              |___ models
                    |___ mod.rs
                    |___ cab_model.rs
                    |___ person_model.rs
                    |___ point_model.rs
              |___ repository
                    |___ mod.rs
                    |___ mongodb_repos.rs
              |___ lib.rs
              |___ main.rs
        |___ target
        |___ tests
              |___ test.rs
              |___ api_test.rs
        |___ .env
        |___ .gitignore
        |___ Cargo.lock
        |___ Cargo.toml
        |___ README.md
    ```
   `main.rs` is where the server is `lib.rs` is a deprecated library which was initially used but was then modularized in the `src/api`, `src/models` and `src/repository` directories.

- Now to run the server one needs to just do `cargo run --release` or `cargo run`. The release flag is for an optimized build and doesn't effect the application. It wouldn't show the log screen with `--release` so I would suggest `cargo run` for the first time.

    #### Tanget to errors

    - What happens when you **don't add the `.env` file** : It PANICS!
        ```bash
                 Finished release [optimized] target(s) in 2m 20s
             Running `target/release/fuber` thread 'main' panicked at 'unable to get a client', src/repository/mongodb_repos.rs:33:23
             note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        ```
        And I love rust so much because it essentially tells you when it panics that it couldn't find a client. So do make sure if you get an error to check if the `.env` exists.

- If the run was successful and if you didn't use the `--release` you'll get the following output on the terminal
    ```bash
            Finished dev [unoptimized + debuginfo] target(s) in 0.07s
             Running `target/debug/fuber`
            🔧 Configured for debug.
            >> address: 127.0.0.1
            >> port: 8000
            >> workers: 8
            >> ident: Rocket
            >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
            >> temp dir: /tmp
            >> http/2: true
            >> keep-alive: 5s
            >> tls: disabled
            >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
            >> log level: normal
            >> cli colors: true
            📬 Routes:
            >> (hello) GET /
            >> (get_fleet) GET /cab/fleet
            >> (create_cab) POST /cab/create
            >> (get_cab) GET /cab/<cab_id>
            >> (delete_fleet) DELETE /cab/delete_fleet
            >> (create_fleet) POST /cab/create/fleet
            >> (generate_fleet) GET /cab/fleet/<size>
            >> (update_cab) PUT /cab/update_cab/<cab_id>
            >> (delete_cab) DELETE /cab/delete_cab/<cab_id>
            >> (update_location) PUT /cab/update_location/<cab_id>
            >> (assign_person) PUT /cab/assign_person/<person_id>
            >> (create_person) POST /person/create
            >> (get_person) GET /person/<person_id>
            >> (delete_all_people) DELETE /person/delete_all_people
            >> (request_cab) GET /person/request_cab/<person_id>
            >> (unassign_cab) GET /person/unassign_cab/<person_id>
            >> (update_person) PUT /person/update_person/<person_id>
            >> (delete_person) DELETE /person/delete_person/<person_id>
            📡 Fairings:
            >> Shield (liftoff, response, singleton)
            🛡️ Shield:
            >> Permissions-Policy: interest-cohort=()
            >> X-Content-Type-Options: nosniff
            >> X-Frame-Options: SAMEORIGIN
            🚀 Rocket has launched from http://127.0.0.1:8000

    ```
    Since we are using the Rocket framework you can clearly see the routes used and which correspond the function that is used in the `src/api` and also if you goto `localhost:8000/` you will see `"Hello from Fuber"`.

### Model

Before we go into api calls it's easier to understand what kind of structure we are dealing with since making the request calls might be easier after understanding the what we are actually sending

The cab entity and person entity are structs or records of types that is often displayed in the API examples above. It's not a complex structure but the following would help in showing what they are made up of and how to instatiate the json body when making a request.

#### Person 

1. id (optional) [type : ObjectId] : This is an optional attribute and is not required when creating a person and is often generated as a response when the person is created. This is created using the MongoDb hashing schemes hence it's better not to modify anything and just use the ones that are generated. 
2. name (required) [type : String] : This is a neccessary attribute when creating a person
3. location (required) [type : Object] : This attribute is neccessary to create a person and the object should contain 2 `integers` representing the `(x,y)` co-ordinates. 
    Example : 

    ```json 
    {
        "x" : 1,
        "y" : 2
    }
    ```
4. destination (required) [type : Object] : This attribute is neccessary to create a person and also is a pair of integers similar to location.

#### Cab

1. id (optional) [type : ObjectId] : This is an optional attribute and is not required when creating a cab and is generated as a response when the cab is generated. Similar to `Person` this is generated by MongoDB hasing schemes.
2. location (required) [type : Object] : This attribute is neccessary to create cab and the object should contain 2 `integers` representing the `(x,y)` co-ordinates. The example is similar to the location attribute in `Person`.
3. destination (optional) [type: Object] : Similar to location but an optional argument often left as null because logically a cab doesn't have to go anywhere if it is unassigned.
4. person_id (hidden) [type : ObjectId] : This is a hidden attribute which is only visible when a person is assigned. The type is similar to `id`. This attribute is only visible when the cab is assigned.

### API
Every API call has 2 different ways of accessing and for different things
        - `localhost:8000/person/...` for accessing function calls for what a person should be able to do
        - `localhost:8000/cab/...` for accessing function calls for cab(s) should be able to do

#### Person
Let's start with `/person` function calls
<table>
    <tr>
        <td>Type of Request</td><td>Request URL</td><td>Body of Request</td><td>Body of Response (Success) </td><td> Error Response </td>
    </tr>
    <tr>
        <td>POST</td>
        <td><code>person/create</code></td>
        <td> Example of how a body of person should be, which is a json with fields name, location and destination where location and destination are objects with x and y co-ordinates
        
   ```json
    {
        "name" : "Shubham Kumar",
        "location" : {
            "x" : 1,
            "y" : 1
        },
        "destination" : {
            "x" : 10,
            "y" : 10
        }
    }
   ```
   </td>
        <td> The response is the id of the person inserted and this can be used to get the cabs
            
  ```json
  "632c104e825fbee945c7d24c"
  ```
   </td>
        <td>
            <ul>
                <li> 500 Internal Server Error : If you are unable to create a person </li>
                <li> 417 Expectation Failed : If you are unable to access the object id </li>
            </ul>
        </td>
    </tr>
    <tr>
        <td>GET</td>
        <td><code>person/[person_id]</code></td>
        <td> Empty request except the <code>person_id</code> which is the oid from the previous post request
 
   ```json
   {}
   ```
   </td>
        <td> Successful response would look something like the following
            
   ```json
{
        "_id": {
            "$oid": "632aa895868574ce15dde868"
        },
        "name": "Shubham Kumar",
        "location": {
            "x": 1,
            "y": 1
        },
        "destination": {
            "x": 10,
            "y": 10
        }
}
   ```
   </td>
        <td>
            <ul>
                <li> 400 Bad Request : If you are trying to get a person without passing the person id </li>
                <li> 500 Internal Server Error : If you are unable to get the person using the person id because the person is absent </li>
            </ul>
        </td>
    </tr>
    <tr>
        <td>GET</td>
        <td><code>person/request_cab/[person_id]</code></td>
        <td> The body is empty while making this request
            
```json
{}
```         
</td>
        <td> The response to the request when <strong>Successful</strong> returns the person which I am trying to assign as well as the cab information with the updated destination of the cab to the person's location. Also the person_id is assigned indicating this cab is assigned.

```json
[
    {
        "_id": {
            "$oid": "632e5c011b54f17eb1c327be"
        },
        "name": "Shubham Kumar",
        "location": {
            "x": 1,
            "y": 1
        },
        "destination": {
            "x": 10,
            "y": 10
        }
    },
    {
        "_id": {
            "$oid": "632e5ba81b54f17eb1c327bd"
        },
        "location": {
            "x": 67,
            "y": -18
        },
        "destination": {
            "x": 1,
            "y": 1
        },
        "person_id": {
            "$oid": "632e5c011b54f17eb1c327be"
        }
    }
]
```
        
</td>
    <td>
        <ul>
            <li> 400 Bad Request : If you are trying to request a person without passing the person id </li>
            <li> 403 Forbidden : If the person is already assigned </li>
            <li> 404 Not Found : If the person is trying to assign to the nearest cab but there are none near it </li>
            <li> 500 Internal Server Error : If you are unable to get the updated cab info with the person assigned or the person is unable to be asssigned because it is already assigned  </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>GET</td><td><code>person/unassign_cab/[person_id]</code></td>
        <td> The body is empty while making this request
        
```json
{}
```
        
</td>
        <td>The response if successful shows the cab has no person_id field and the destination is null. The location of the cab has been updated to the person's destination if it was carrying that person.

```json 
[
    {
        "_id": {
            "$oid": "632e5c011b54f17eb1c327be"
        },
        "name": "Shubham Kumar",
        "location": {
            "x": 1,
            "y": 1
        },
        "destination": {
            "x": 10,
            "y": 10
        }
    },
    {
        "_id": {
            "$oid": "632e5ba81b54f17eb1c327bd"
        },
        "location": {
            "x": 10,
            "y": 10
        },
        "destination": null
    }
]
```
        
</td>
    <td>
        <ul>
            <li> 400 Bad Request : If you are trying to request a person without passing the person id </li>
            <li> 403 Forbidden : If the person is already unassigned </li>
            <li> 404 Not Found : If the person is trying to unassign to the nearest cab cannot be found </li>
            <li> 500 Internal Server Error : If you are unable to unassign or you are not able to get the updated cab info after unassigning it </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>PUT</td><td><code>person/update_person/[person_id]</code></td>
        <td>The body of the put request is a json object with all of the entries of the person structure.
            
```json 
{    
    "name": "Shubham Kumar",
    "location": {
        "x": 100,
        "y": 100
    },
    "destination": {
        "x": 210,
        "y": 210
    }
}
```
     
</td>
     <td>The response of this put request is an updated person entity with the same id 
         
```json
{
    "_id": {
        "$oid": "632e5c011b54f17eb1c327be"
    },
    "name": "Shubham Kumar",
    "location": {
        "x": 100,
        "y": 100
    },
    "destination": {
        "x": 210,
        "y": 210
    }
}
```

</td>
    <td>
        <ul>
            <li> 400 Bad Request : If you trying to update a person without a providing a person_id </li>
            <li> 404 Not Found : If the number of updates is more than one but it specifically should be one update per request</li>
            <li> 500 Internal Server Error : If you are unable to update  </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>DELETE</td><td><code>person/delete_person/[person_id]</code></td>
        <td>The body of the request is empty because you only person_id to delete the person
            
```json 
{}
```
            
</td>
     <td>The response of the delete request is a successful custom message
 
```json 
"Person successfully deleted!"
```
         
 </td>
    <td>
        <ul>
            <li> 400 Bad Request : If you trying to update a person without a providing a person_id </li>
            <li> 404 Not Found : If the number of deletes is more than one but it specifically should be one delete per request</li>
            <li> 500 Internal Server Error : If you are unable to delete person  </li>
        </ul>
    </td>
    </tr>
</table>

#### Cab

<table>
    <tr>
        <td>Type of Request</td><td>Request URL</td><td>Body of Request</td><td>Body of Response (Success) </td>
    </tr>
    <tr>
        <td>POST</td><td><code>cab/create</code></td>
        <td>The request to create a single cab entity, The entity comprises of location and destination
            
```json 
{
    "location": {
        "x": 15,
        "y": -23
    },
    "destination": null
}
```
            
</td>
     <td>The body of a successful response. The object Id for the response can be used to access cab entity.
 
```json 
"63300dfd3ae7cab7efd03885"
```
         
 </td>
    <td>
        <ul>
            <li> 500 Internal Server Error : If you are unable to create a cab due to some inconsistency in the request body or in the database </li>
            <li> 417 Expectation Failed : If you are unable to access the object id </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>POST</td><td><code>cab/create/fleet</code></td>
        <td>The body of the request is an array of object. The structure of the object is similar to the previous request where the cab location is initialized.
            
```json 
[
    {
        "location": {
            "x": 15,
            "y": -23
        },
        "destination": null
    },
    {
        "location": {
            "x": 1,
            "y": -58
        },
        "destination": null
    },
    {
        "location": {
            "x": -42,
            "y": 31
        },
        "destination": null
    }
]
```
            
</td>
     <td>The body of a successful response. The array of object Ids for the response can be used to access cab model.
 
```json 
[
    "63313ee04fafd031e499bd9d",
    "63313ee04fafd031e499bd9e",
    "63313ee04fafd031e499bd9f"
]
```
         
 </td>
    <td>
        <ul>
            <li> 500 Internal Server Error : If you are unable to create a fleet due to some inconsistency in the request body or in the database </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>GET</td><td><code>cab/fleet</code></td>
        <td>The body of the request is empty because we are trying to get all the members of the fleet in the response.
            
```json 
{}
```
            
</td>
     <td>The response of the get request is a json array with a complete fleet of cabs
 
```json 
[
    {
        "_id": {
            "$oid": "632e5ba81b54f17eb1c327bb"
        },
        "location": {
            "x": 35,
            "y": -74
        },
        "destination": null
    },
    {
        "_id": {
            "$oid": "632e5ba81b54f17eb1c327bc"
        },
        "location": {
            "x": 79,
            "y": -56
        },
        "destination": null
    },
         
    ...
    ...
    ...
]
```
         
 </td>
    <td>
        <ul>
            <li> 500 Internal Server Error : If you are unable to find a fleet due to absence of any cab </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>GET</td><td><code>cab/[cab_id]</code></td>
        <td>The body of the request is empty. The only important thing needed is the cab_id which is part of request URL.
            
```json 
{}
```
            
</td>
     <td>The response of the get request is a json object with a single cab entity
 
```json 
{
    "_id": {
        "$oid": "632e5ba81b54f17eb1c327bd"
    },
    "location": {
        "x": 10,
        "y": 10
    },
    "destination": null
}
```
         
 </td>
    <td>
        <ul>
            <li> 400 Bad Request : If you provide an empty cab_id </li>
            <li> 500 Internal Server Error : If you are unable to get a cab with the cab_id provided </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>PUT</td><td><code>cab/update_cab/[cab_id]</code></td>
        <td>The body of the request contains the entities of cab struct - id, location, destination, person_id if there is a person assigned to the cab. It requires the cab id of the cab you want to update in the request URL.
            
```json 
{
        "location": {
            "x": 10,
            "y": 10
        },
        "destination": null
}
```
            
</td>
     <td>The response of the put request is an updated cab entry
 
```json 
{
    "_id": {
        "$oid": "633056e7fadb2ff07c443edf"
    },
    "location": {
        "x": 10,
        "y": 10
    },
    "destination": null
}
```
         
 </td>
    <td>
        <ul>
            <li> 400 Bad Request : If you provide an empty cab_id </li>
            <li> 404 Not Found : If the cab you are trying to update is not found </li>
            <li> 500 Internal Server Error : If you are unable to get the cab you want to update with the cab_id provided </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>PUT</td><td><code>cab/update_location/[cab_id]</code></td>
        <td>The body of the request contains the location co-ordinates that need to be updated.
            
```json 
{
    "x":0,
    "y":0
}
```
            
</td>
     <td>The response of the put request is an updated cab entry
 
```json 
{
    "_id": {
        "$oid": "633056e7fadb2ff07c443edf"
    },
    "location": {
        "x": 0,
        "y": 0
    },
    "destination": null
}
```
         
 </td>
    <td>
        <ul>
            <li> 400 Bad Request : If you provide an empty cab_id or you cannot get the cab because the cab_id is wrong and no such cab with that cab_id exists </li>
            <li> 404 Not Found : If the cab you are trying to update is not found </li>
            <li> 500 Internal Server Error : If you are unable to get the cab you want to update with the cab_id provided </li>
        </ul>
    </td>
    </tr>
    <tr>
        <td>DELETE</td><td><code>cab/delete_cab/[cab_id]</code></td>
        <td>The body of the request is empty because you only cab_id to delete the cab
            
```json 
{}
```
            
</td>
     <td>The response of the delete request is a successful custom message
 
```json 
"Cab successfully deleted!"
```
         
 </td>
    <td>
        <ul>
            <li> 400 Bad Request : If you provide an empty cab_id </li>
            <li> 404 Not Found : If the cab you are trying to delete is not found </li>
            <li> 500 Internal Server Error : If you are unable to delete a cab with the cab_id provided </li>
        </ul>
    </td>
    </tr>
</table>

### Tests
The following are not api calls just the description of the function which runs unit tests. The tests are made using the specifications.

To run the tests inside the root directory use `cargo test`. If all the tests pass the following line show up.
```bash
     Running tests/api_test.rs (target/debug/deps/api_test-7ee2d6439d41dd70)

running 3 tests
test test_request_cab_panic_when_fleet_occupied - should panic ... ok
test test_assign_cab_panic - should panic ... ok
test test_get_nearest_cab ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.44s
```

1. `tests/api_test/test_get_nearest_cab` : Tests if the `person/request_cab/<person_id>` assigned the cab nearest to it's location by manually finding the nearest cab and comparing if they both are the same.
2. `tests/api_test/test_assign_cab_panic` : Panic tests if a cab of cab_id is already assigned to a person of person_id and another person is forced to assign to the already assigned cab then it panics. And it is expected to panic to make sure that the tests pass.
3. `tests/api_test/test_request_cab_panic_when_fleet_occupied` : Panic tests if a fleet which is already occupied (in this case a fleet of size 3 with 3 people) and if another person tries to request a cab it panics and returns an error. This is expected to panic so that tests pass.