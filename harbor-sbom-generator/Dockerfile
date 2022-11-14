FROM golang:1.19.3 as build
WORKDIR /app
ADD go.mod go.sum ./
RUN go mod download
# build
ADD . .
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o /main
# copy artifacts to a clean image
FROM public.ecr.aws/lambda/provided:al2-x86_64
RUN yum install -y git curl tar
RUN mkdir -p /opt/bin
RUN curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /opt/bin v0.60.3
COPY --from=build /main /main
ENTRYPOINT [ "/main" ]
