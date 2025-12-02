# Rat-Website
simple website to host RAT tool and info!

## Setup/Install
### Docker
1. First git clone
```shell
git clone https://github.com/TegranGrigorian/rat-website.git
```

2. Install docker, use Debian/Ubuntu script
```shell
chmod +x scripts/install_docker.sh
./scripts/install_docker.sh
```

3. Build docker container
```shell
docker build -t "rat-website" .
```

4. Run docker container on port 3000
```shell
docker run -p 3000:3000 --name "rat-website" "rat-website"
```

5. Its up :)
