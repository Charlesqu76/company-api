#! /bin/bash

echo "***** start *****"

dockerOperate(){
    echo "docker build start"
    docker build -t company-api .
    docker stop company-api
    docker rm company-api
    docker run -d --name company-api -p 3002:3002 company-api
}

dockerOperate

echo "***** end *****"
