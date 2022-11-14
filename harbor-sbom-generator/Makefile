BINARY_NAME=harbor-sbom-generator

git:
	docker build -t harbor/gitlayer:latest .
	docker create --platform linux/amd64 --name git-layer harbor/gitlayer:latest
	docker cp git-layer:/tmp/dependencies.zip bin/dependencies.zip
	docker rm -f git-layer

lambda:
	GOOS=linux GOARCH=amd64 go build -o main .
	zip main.zip main
	rm main

race:
	CGO_ENABLED=1 GOOS=linux GOARCH=amd64 go build -o main -race .

build: lint
	GOOS=linux GOARCH=amd64 go build -o main .

run:
	./${BINARY_NAME}

build_and_run: build run

clean:
	go clean
	rm main

test:
	go test ./...

test_coverage:
	go test ./... -coverprofile=coverage.out

dep:
	go mod download

tidy:
	go mod tidy

vet:
	go vet

lint:
	golangci-lint run --enable-all
