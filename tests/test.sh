#! /usr/bin/env bash

url="https://admin.j2y.localhost:5003/"

read -p "Submitting JSON content"
curl --header "Content-Type: application/json" --request POST -k --data @test.json $url

echo ""
read -p "Submitting YAML content as regular data (which removes line breaks)"
curl --header "Content-Type: application/yaml" --request POST -k --data @test.yml $url

echo ""
read -p "Submitting YAML content as binary data to preserve line breaks"
curl --header "Content-Type: application/yaml" --request POST -k --data-binary @test.yml $url

echo ""
read -p "Submitting JSON with no content type specified"
curl --request POST -k --data @test.json $url
