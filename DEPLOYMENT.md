# Deployment

1. Set up a google compute engine VM with Debian 12
    - 300usd in free credits
    - ec2-micro, us-central1, 30gb is free? https://cloud.google.com/free/docs/free-cloud-features#compute
2. Install docker on debian 12 instance


```shell
sudo apt update
sudo apt upgrade -y

sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian bookworm stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update
sudo apt-get install -y docker-ce
# sudo apt install docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo usermod -aG docker $USER
echo "User ${USER} added to docker group. Re-login to assume docker group membership."

sudo docker run hello-world
```

3. Create dockerfile for rust application
4. Set up caddy for https
5. build and run docker containers

```shell
docker-compose up -d
```



Goals
- docker + caddy deployment, pointing to benburk.ca
- mount database file separately


- google cloud free tier https://cloud.google.com/free/docs/free-cloud-features#compute
- want to use a generic compute engine VM, easy to migrate to other cloud providers

going to use caddy

```shell
sudo apt update
sudo apt-get install docker.io
sudo usermod -aG docker ${USER}
# logout and log back in
```


## On mac

```shell
brew install caddy
```


## Links

- https://github.com/sambacha/caddyflask