# What is this easyweb
Easyweb is a versitile tool to transfer files among devices via Web.
So you do not have to use your cloud storage.

# How to use it
1. Build vue front UI if needed, by `vue build` under `front-ui` folder.
2. `cargo run` or `cargo build` to start the app at port `8000`.
3. Access the app via `http://localhost:8000` or `http://YOUR-IP-ADDR:8000` from other devices.

# What common problems
## Firewall issue 
- Make sure you have the firewall of the host machine allow the traffic from your client machine to your host machine at port `8000`.
  And allow the traffic back to your client from the host machine.
