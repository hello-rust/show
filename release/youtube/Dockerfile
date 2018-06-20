FROM golang:1.10-alpine AS build
LABEL maintainer="Matthias Endler <matthias.endler@trivago.com>"
RUN apk update && apk upgrade && \
    apk add --no-cache git 
WORKDIR /go/src/github.com/hello-rust/show/
COPY . .
#RUN go get golang.org/s/gogetcmd
#RUN go get golang.org/x/net/context
RUN go get -u github.com/golang/dep/cmd/dep
RUN dep init
RUN dep ensure
RUN GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o /bin/app

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=build /bin/app .
CMD ["./app"]  %