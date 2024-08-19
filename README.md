## What the hell is happening?
- This is a repo to understand a basic implementation of gRPC in Rust
- We define a proto service
- Generate client and server code 
- Write a simple client and server for the service

## The Example : 
- We take the simple example of a route mapping app
- Client GETs information about 
  - features on their route
  - summary of the route
  - exchange info with the server and other clients
- The information is passed through protocol buffers

## What is gRPC ? 
- Open source RPC framework that can run in any environament
- Efficiently connects services across data centres
- https://grpc.io/about/

## Overview
- We first create the gRPC service and request response models in `route_guide.proto` file
- All the basic types of RPCs are covered
  - Simple RPC
  - Client streaming
  - Server streaming
  - Bidirectional streaming
- We then use tonic to build this proto file see `build.rs`
  - What this does is generate traits for us to use into our code
  - The generated code contains:
      - Struct definitions for message types Point, Rectangle, Feature, RouteNote, RouteSummary.
      - A service trait we'll need to implement: route_guide_server::RouteGuide.
      - A client type we'll use to call the server: route_guide_client::RouteGuideClient<T>.
- Then we go about implementing the server
  - We need to implement the trait that was generated
  - All the functions mentioned in the proto (that was generated in the trait) are implemented
- `data.rs` contains code to deserialise the response in `route_guide_db.json`
- Then we write our client
  - Client calls each of our service function 
  - Has helper function